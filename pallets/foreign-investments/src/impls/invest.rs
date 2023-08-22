// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge Chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

use core::cmp::Ordering;

use cfg_types::investments::Swap;
use frame_support::dispatch::fmt::Debug;
use sp_runtime::{
	traits::{EnsureAdd, EnsureSub},
	ArithmeticError, DispatchError,
};

use crate::types::{InvestState, InvestTransition};

impl<Balance, Currency> InvestState<Balance, Currency>
where
	Balance: Clone + Copy + EnsureAdd + EnsureSub + Ord + Debug,
	Currency: Clone + Copy + PartialEq + Debug,
{
	/// Solely apply state machine to transition one `InvestState` into another
	/// based on the transition, see https://centrifuge.hackmd.io/IPtRlOrOSrOF9MHjEY48BA?view#State-diagram.
	///
	/// NOTE: MUST call `apply_invest_state_transition` on the post state to
	/// actually mutate storage.
	pub fn transition(
		&self,
		transition: InvestTransition<Balance, Currency>,
	) -> Result<Self, DispatchError> {
		match transition {
			InvestTransition::IncreaseInvestOrder(swap) => Self::handle_increase(self, swap),
			InvestTransition::DecreaseInvestOrder(swap) => Self::handle_decrease(self, swap),
			InvestTransition::FulfillSwapOrder(swap) => {
				Self::handle_fulfilled_swap_order(self, swap)
			}
			InvestTransition::CollectInvestment(amount_unprocessed) => {
				Self::handle_collect(self, amount_unprocessed)
			}
		}
	}

	/// Returns the potentially existing active swap into either pool or return
	/// currency:
	/// * If the state includes `ActiveSwapInto{Pool, Return}Currency`, it
	///   returns `Some(swap)`.
	/// * Else, it returns `None`.
	pub(crate) fn get_active_swap(&self) -> Option<Swap<Balance, Currency>> {
		match *self {
			InvestState::NoState => None,
			InvestState::InvestmentOngoing { .. } => None,
			InvestState::ActiveSwapIntoPoolCurrency { swap } => Some(swap),
			InvestState::ActiveSwapIntoForeignCurrency { swap } => Some(swap),
			InvestState::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing { swap, .. } => Some(swap),
			InvestState::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing { swap, .. } => Some(swap),
			InvestState::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDone { swap, .. } => Some(swap),
			InvestState::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone { swap, .. } => Some(swap),
			InvestState::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing { swap, .. } => {
				Some(swap)
			},
			InvestState::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing { swap, .. } => {
				Some(swap)
			},
			InvestState::SwapIntoForeignDone { .. } => None,
			InvestState::SwapIntoForeignDoneAndInvestmentOngoing { .. } => None,
		}
	}

	/// Returns the `invest_amount` if existent, else zero.
	pub(crate) fn get_investing_amount(&self) -> Balance {
		match *self {
			InvestState::InvestmentOngoing { invest_amount}  |
			InvestState::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing { invest_amount, .. } |
			InvestState::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing { invest_amount, .. } |
			InvestState::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing { invest_amount, .. } |
			InvestState::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing { invest_amount, .. } |
			InvestState::SwapIntoForeignDoneAndInvestmentOngoing { invest_amount, .. } => invest_amount,
			_ => Balance::zero()
		}
	}
}

// Actual impl of transition
impl<Balance, Currency> InvestState<Balance, Currency>
where
	Balance: Clone + Copy + EnsureAdd + EnsureSub + Ord + Debug,
	Currency: Clone + Copy + PartialEq + Debug,
{
	/// Handle `increase` transitions depicted by `msg::increase` edges in the
	/// invest state diagram:
	/// * If there is no swap into foreign currency, the pool currency swap
	///   amount is increased.
	/// * Else, resolves opposite swap directions by immediately fulfilling the
	///   side with lower amounts; or both if the swap amounts are equal.
	///
	/// When we increase an investment, we normally have to swap it into pool
	/// currency (`ActiveSwapIntoPoolCurrency`) before it can be invested
	/// (`ActiveInvestmentOngoing`). However, if the current state includes
	/// swapping back into pool currency (`ActiveSwapIntoForeignCurrency`) as
	/// the result of a previous decrement, then we can minimize the amount
	/// which needs to be swapped such that we always have **at most a single
	/// active swap** which is the maximum of `pool_swap.amount` and
	/// `foreign_swap.amount`. When we do this, we always need to bump the
	/// investment amount as well as the `SwapIntoForeignDone` amount as a
	/// result of immediately fulfilling the pool swap order up to the possible
	/// amount.
	///
	/// Example:
	/// * Say before my pre invest state has `foreign_done = 1000` and
	/// `foreign_swap.amount = 500`. Now we look at three scenarios in which we
	/// increase below, exactly at and above the `foreign_swap.amount`:
	/// * a) If we increase by 500, we can reduce the `foreign_swap.amount`
	///   fully, which we denote by adding the 500 to the `foreign_done` amount.
	///   Moreover, we can immediately invest the 500. The resulting state is
	///   `(done_amount = 1500, investing = 500)`.
	/// * b) If we increase by 400, we can reduce the `foreign_swap.amount` only
	///   by 400 and increase both the `investing` as well as `foreign_done`
	///   amount by that. The resulting state is
	/// `(done_amount = 1400, foreign_swap.amount = 100, investing = 400)`.
	/// * c) If we increase by 600, we can reduce the `foreign_swap.amount`
	///   fully and need to add a swap into pool currency for 100. Moreover both
	///   the `investing` as well as `foreign_done` amount can only be increased
	///   by 500. The resulting state is
	/// `(done_amount = 1500, pool_swap.amount = 100, investing = 500)`.
	///
	/// NOTE: We can ignore handling all states which include
	/// `*SwapIntoForeignDone` without `ActiveSwapIntoForeignCurrency*` as we
	/// consume the done amount and transition in the post transition phase.
	/// To be safe and to not make any unhandled assumptions, we throw
	/// `DispatchError::Other` for these states though we need to make sure
	/// this can never occur!
	fn handle_increase(&self, swap: Swap<Balance, Currency>) -> Result<Self, DispatchError> {
		if swap.currency_in == swap.currency_out {
			return Self::handle_increase_non_foreign(self, swap);
		}

		match &self {
			Self::NoState => Ok(Self::ActiveSwapIntoPoolCurrency { swap }),
			// Add pool swap
			Self::InvestmentOngoing { invest_amount } => {
				Ok(Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
					swap,
					invest_amount: *invest_amount,
				})
			}
			// Bump pool swap
			Self::ActiveSwapIntoPoolCurrency { swap: pool_swap } => {
				swap.ensure_currencies_match(pool_swap, true)?;
				Ok(Self::ActiveSwapIntoPoolCurrency {
					swap: Swap {
						amount: swap.amount.ensure_add(pool_swap.amount)?,
						..swap
					},
				})
			}
			// Reduce foreign swap amount by the increasing amount and increase investing amount as
			// well adding foreign_done amount by the minimum of active swap amounts
			Self::ActiveSwapIntoForeignCurrency { swap: foreign_swap } => {
				swap.ensure_currencies_match(foreign_swap, false)?;
				let invest_amount = swap.amount.min(foreign_swap.amount);
				let done_amount = swap.amount.min(foreign_swap.amount);

				match swap.amount.cmp(&foreign_swap.amount) {
					// pool swap amount is immediately invested and done amount increased equally
					Ordering::Equal => {
						Ok(
							Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe since swap.amount < foreign_swap.amount
									amount: foreign_swap.amount - swap.amount,
									..*foreign_swap
								},
								done_amount,
								invest_amount,
							},
						)
					}
					// swap amount is immediately invested and done amount increased equally
					Ordering::Less => Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
						done_swap: *foreign_swap,
						invest_amount,
					}),
					// foreign swap amount is immediately invested and done amount increased equally
					Ordering::Greater => {
						Ok(
							Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe since swap.amount > foreign_swap.amount
									amount: swap.amount - foreign_swap.amount,
									..swap
								},
								done_amount,
								invest_amount,
							},
						)
					}
				}
			}
			// Bump pool swap
			Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
				swap: pool_swap,
				invest_amount,
			} => {
				swap.ensure_currencies_match(pool_swap, true)?;

				Ok(Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
					swap: Swap {
						amount: swap.amount.ensure_add(pool_swap.amount)?,
						..swap
					},
					invest_amount: *invest_amount,
				})
			}
			// Reduce foreign swap amount by the increasing amount and increase investing amount as
			// well adding foreign_done amount by the minimum of active swap amounts
			Self::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing {
				swap: foreign_swap,
				invest_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, false)?;
				let invest_amount =
					invest_amount.ensure_add(swap.amount.min(foreign_swap.amount))?;
				let done_amount = swap.amount.min(foreign_swap.amount);

				match swap.amount.cmp(&foreign_swap.amount) {
					// pool swap amount is immediately invested and done amount increased equally
					Ordering::Equal => {
						Ok(
							Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe since swap.amount < foreign_swap.amount
									amount: foreign_swap.amount - swap.amount,
									..*foreign_swap
								},
								done_amount,
								invest_amount,
							},
						)
					}
					// swap amount is immediately invested and done amount increased equally
					Ordering::Less => Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
						done_swap: *foreign_swap,
						invest_amount,
					}),
					// foreign swap amount is immediately invested and done amount increased equally
					Ordering::Greater => {
						Ok(
							Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe since swap.amount > foreign_swap.amount
									amount: swap.amount - foreign_swap.amount,
									..swap
								},
								done_amount,
								invest_amount,
							},
						)
					}
				}
			}
			// Reduce amount of foreign by the increasing amount and increase investing as well as
			// foreign_done amount by the minimum of active swap amounts
			Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone {
				swap: foreign_swap,
				done_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, false)?;
				let invest_amount = swap.amount.min(foreign_swap.amount);
				let done_amount = invest_amount.ensure_add(*done_amount)?;

				// pool swap amount is immediately invested and done amount increased equally
				match swap.amount.cmp(&foreign_swap.amount) {
					Ordering::Equal => Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
						done_swap: Swap {
							amount: done_amount,
							..*foreign_swap
						},
						invest_amount,
					}),
					// swap amount is immediately invested and done amount increased equally
					Ordering::Less => {
						Ok(
							Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe since swap.amount < foreign_swap.amount
									amount: foreign_swap.amount - swap.amount,
									..*foreign_swap
								},
								done_amount,
								invest_amount,
							},
						)
					}
					// foreign swap amount is immediately invested and done amount increased equally
					Ordering::Greater => {
						Ok(
							Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe since swap.amount > foreign_swap.amount
									amount: swap.amount - foreign_swap.amount,
									..swap
								},
								done_amount,
								invest_amount,
							},
						)
					}
				}
			}
			// Reduce amount of foreign swap by increasing amount and increase investing as well as
			// foreign_done amount by minimum of swap amounts
			Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
				swap: foreign_swap,
				done_amount,
				invest_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, false)?;
				let invest_amount =
					invest_amount.ensure_add(swap.amount.min(foreign_swap.amount))?;
				let done_amount = swap
					.amount
					.min(foreign_swap.amount)
					.ensure_add(*done_amount)?;

				match swap.amount.cmp(&foreign_swap.amount) {
					// pool swap amount is immediately invested and done amount increased equally
					Ordering::Equal => Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
						done_swap: Swap {
							amount: done_amount,
							..*foreign_swap
						},
						invest_amount,
					}),
					// swap amount is immediately invested and done amount increased equally
					Ordering::Less => Ok(
						Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
							swap: Swap {
								// safe since swap.amount < foreign_swap.amount
								amount: foreign_swap.amount - swap.amount,
								..*foreign_swap
							},
							done_amount,
							invest_amount,
						},
					),
					// foreign swap amount is immediately invested and done amount increased equally
					Ordering::Greater => Ok(
						Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
							swap: Swap {
								// safe since swap.amount > foreign_swap.amount
								amount: swap.amount - foreign_swap.amount,
								..swap
							},
							done_amount,
							invest_amount,
						},
					),
				}
			}
			_ => Err(DispatchError::Other(
				"Invalid invest state, should automatically be transitioned into \
				 ActiveSwapIntoPoolCurrencyAndInvestmentOngoing",
			)),
		}
	}

	/// Handle `decrease` transitions depicted by `msg::decrease` edges in the
	/// state diagram:
	/// * If there is no swap into pool currency, the foreign currency swap
	///   amount is increased up to the ongoing investment amount which is not
	///   yet processed.
	/// * Else, resolves opposite swap directions by immediately fulfilling the
	///   side with lower amounts; or both if the swap amounts are equal.
	///
	/// Throws if the decreasing amount exceeds the amount which is
	/// currently swapping into pool currency and/or investing as we cannot
	/// decrease more than was invested. We must ensure, this can never happen
	/// at this stage!
	///
	/// NOTE: We can ignore handling all states which include
	/// `SwapIntoForeignDone` without `ActiveSwapIntoForeignCurrency` as we
	/// consume the done amount and transition in the post transition phase.
	/// Moreover, we can ignore handling all states which do not include
	/// `ActiveSwapIntoPoolCurrency` or `InvestmentOngoing` as we cannot reduce
	/// further then.
	/// To be safe and to not make any unhandled assumptions, we throw
	/// `DispatchError::Other` for these states though we need to make sure
	/// this can never occur!
	fn handle_decrease(&self, swap: Swap<Balance, Currency>) -> Result<Self, DispatchError> {
		if swap.currency_in == swap.currency_out {
			return Self::handle_decrease_non_foreign(self, swap);
		}

		match &self {
			// Cannot reduce if there is neither an ongoing investment nor an active swap into pool currency
			InvestState::NoState
			| InvestState::ActiveSwapIntoForeignCurrency { .. }
			| InvestState::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone { .. } => {
				Err(DispatchError::Other("Invalid invest state when transitioning a decrease"))
			},
			// Increment foreign swap amount up to ongoing investment
			InvestState::InvestmentOngoing { invest_amount } => {
				match swap.amount.cmp(invest_amount) {
					Ordering::Less => {
						Ok(Self::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing {
							swap,
							invest_amount: *invest_amount - swap.amount,
						})
					},
					Ordering::Equal => {
						Ok(Self::ActiveSwapIntoForeignCurrency { swap })
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Underflow))
					}
				}
			},
			// Increment return done amount up to amount of the active pool swap
			InvestState::ActiveSwapIntoPoolCurrency { swap: pool_swap } => {
				swap.ensure_currencies_match(pool_swap, false)?;

				match swap.amount.cmp(&pool_swap.amount) {
					Ordering::Equal => {
						Ok(Self::SwapIntoForeignDone { done_swap: swap })
					},
					Ordering::Less => {
						Ok(Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDone {
							swap: Swap {
								// safe because swap.amount < pool_swap.amount
								amount: pool_swap.amount - swap.amount,
								..*pool_swap
							},
							done_amount: swap.amount,
						})
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Underflow))
					}
				}
			},
			// Increment `foreign_done` up to pool swap amount and increment foreign swap amount up to ongoing investment
			InvestState::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
				swap: pool_swap,
				invest_amount,
			} => {
				swap.ensure_currencies_match(pool_swap, false)?;
				let done_amount = swap.amount.min(pool_swap.amount);
				let invest_amount = invest_amount.ensure_sub(done_amount)?;
				let max_decrease_amount = pool_swap.amount.ensure_add(invest_amount)?;

				if swap.amount < pool_swap.amount {
					Ok(
						Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
							swap: Swap {
								// safe because done_amount is min
								amount: pool_swap.amount - done_amount,
								..*pool_swap
							},
							done_amount,
							invest_amount,
						},
					)
				} else if swap.amount == pool_swap.amount {
					Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
						done_swap: swap,
						invest_amount,
					})
				} else if swap.amount < max_decrease_amount {
					Ok(
						Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
							swap: Swap {
								// safe because done_amount is min
								amount: swap.amount - done_amount,
								..swap
							},
							done_amount,
							invest_amount,
						},
					)
				} else if swap.amount == max_decrease_amount {
					Ok(Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone {
						swap: Swap {
							// safe because done_amount is min
							amount: swap.amount - done_amount,
							..swap
						},
						done_amount,
					})
				}
				// should never occur but let's be safe here
				else {
					Err(DispatchError::Arithmetic(ArithmeticError::Underflow))
				}
			},
			// Increment foreign swap up to ongoing investment
			InvestState::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing {
				swap: foreign_swap,
				invest_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, true)?;
				let amount = foreign_swap.amount.ensure_add(swap.amount)?;

				match swap.amount.cmp(invest_amount) {
					Ordering::Less => {
						Ok(Self::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing {
							swap: Swap { amount, ..swap },
							// safe because invest_amount > swap_amount
							invest_amount: *invest_amount - swap.amount,
						})
					},
					Ordering::Equal => {
						Ok(Self::ActiveSwapIntoForeignCurrency {
							swap: Swap { amount, ..swap },
						})
					},
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Underflow))
					},
				}
			},
			InvestState::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
				swap: foreign_swap,
				done_amount,
				invest_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, true)?;
				let amount = foreign_swap.amount.ensure_add(swap.amount)?;

				match swap.amount.cmp(invest_amount) {
					Ordering::Less => {
						Ok(
							Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap { amount, ..swap },
								done_amount: *done_amount,
								// safe because swap.amount < invest_amount
								invest_amount: *invest_amount - swap.amount,
							},
						)
					},
					Ordering::Equal => {
						Ok(Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone {
							swap: Swap { amount, ..swap },
							done_amount: *done_amount,
						})
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Underflow))
					}
				}
			},
			_ => Err(DispatchError::Other(
				"Invalid invest state, should automatically be transitioned into \
				 ActiveSwapIntoPoolCurrencyAndInvestmentOngoing",
			)),
		}
	}

	/// Handle partial/full token swap order transitions  depicted by
	/// `order_partial` and `order_full` edges in the state diagram.
	///
	/// Please note, that we ensure that there can always be at most one swap,
	/// either into pool currency (`ActiveSwapIntoPoolCurrency`) or into foreign
	/// currency (`ActiveSwapIntoForeignCurrency`). Thus, if the previous state
	/// (`&self`) is into pool, we know the incoming transition is made from
	/// return into pool currency and vice versa if the previous state is
	/// swapping into foreign currency.
	///
	/// This transition should always increase the active ongoing
	/// investment.
	///
	/// NOTE: We can ignore handling all states which include
	/// `SwapIntoForeignDone` without `ActiveSwapIntoForeignCurrency` as we
	/// consume the done amount and transition in the post transition phase.
	/// Moreover, we can ignore handling all states which do not include
	/// `ActiveSwapInto{Pool, Return}Currency` as else there cannot be an active
	/// token swap for investments.
	/// To be safe and to not make any unhandled assumptions, we throw
	/// `DispatchError::Other` for these states though we need to make sure
	/// this can never occur!

	// FIXME(@review): This handler assumes partial fulfillments and 1-to-1
	// conversion of amounts, i.e., 100 `foreign_currency` equals 100
	// `pool_currency`. If we use the CurrencyConverter, the amounts could be off as
	// the `CurrencyConverter` is decoupled from the `TokenSwaps` trait.
	fn handle_fulfilled_swap_order(
		&self,
		swap: Swap<Balance, Currency>,
	) -> Result<Self, DispatchError> {
		match &self {
			InvestState::NoState | InvestState::InvestmentOngoing { .. } => Err(DispatchError::Other(
				"Invalid invest state when transitioning a fulfilled order",
			)),

			// Increment ongoing investment by swapped amount
			InvestState::ActiveSwapIntoPoolCurrency { swap: pool_swap } => {
				swap.ensure_currencies_match(pool_swap, true)?;

				match swap.amount.cmp(&pool_swap.amount) {
					Ordering::Equal => {
						Ok(Self::InvestmentOngoing {
							invest_amount: swap.amount,
						})
					},
					Ordering::Less => {
						Ok(Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
							swap: Swap {
								// safe because pool_swap.amount > swap.amount
								amount: pool_swap.amount - swap.amount,
								..swap
							},
							invest_amount: swap.amount,
						})
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
					}
				}
			},
			// Increment done_foreign by swapped amount
			InvestState::ActiveSwapIntoForeignCurrency { swap: foreign_swap } => {
				swap.ensure_currencies_match(foreign_swap, true)?;

				match swap.amount.cmp(&foreign_swap.amount) {
					Ordering::Equal => {
						Ok(Self::SwapIntoForeignDone { done_swap: swap })
					}
					Ordering::Less => {
						Ok(Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone {
							swap: Swap {
								// safe because foreign_swap.amount > swap.amount
								amount: foreign_swap.amount - swap.amount,
								..swap
							},
							done_amount: swap.amount,
						})
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
					}
				}
			},
			// Increment ongoing investment by swapped amount
			InvestState::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
				swap: pool_swap,
				invest_amount,
			} => {
				swap.ensure_currencies_match(pool_swap, true)?;
				let invest_amount = invest_amount.ensure_add(swap.amount)?;

				match swap.amount.cmp(&pool_swap.amount) {
					Ordering::Equal => {
						Ok(Self::InvestmentOngoing { invest_amount })
					},
					Ordering::Less => {
						Ok(Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
							swap: Swap {
								// safe because pool_swap.amount > swap.amount
								amount: pool_swap.amount - swap.amount,
								..swap
							},
							invest_amount,
						})
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
					}
				}
			},
			// Increment done_foreign by swapped amount, leave invest amount untouched
			InvestState::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing {
				swap: foreign_swap,
				invest_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, true)?;

				match swap.amount.cmp(&foreign_swap.amount) {
					Ordering::Equal => {
						Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
							done_swap: swap,
							invest_amount: *invest_amount,
						})
					}
					Ordering::Less => {
						Ok(
							Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe because foreign_swap.amount > swap.amount
									amount: foreign_swap.amount - swap.amount,
									..swap
								},
								done_amount: swap.amount,
								invest_amount: *invest_amount,
							},
						)
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
					}
				}
			},
			// Increment done_foreign by swapped amount
			InvestState::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone {
				swap: foreign_swap,
				done_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, true)?;
				let done_amount = done_amount.ensure_add(swap.amount)?;

				match swap.amount.cmp(&foreign_swap.amount) {
					Ordering::Equal => {
						Ok(Self::SwapIntoForeignDone {
							done_swap: Swap {
								amount: done_amount,
								..swap
							},
						})
					}
					Ordering::Less => {
						Ok(Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone {
							swap: Swap {
								// safe because foreign_swap.amount > swap.amount
								amount: foreign_swap.amount - swap.amount,
								..swap
							},
							done_amount,
						})
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
					}
				}
			},
			// Increment done_foreign by swapped amount, leave invest amount untouched
			InvestState::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
				swap: foreign_swap,
				done_amount,
				invest_amount,
			} => {
				swap.ensure_currencies_match(foreign_swap, true)?;
				let done_amount = done_amount.ensure_add(swap.amount)?;

				match swap.amount.cmp(&foreign_swap.amount) {
					Ordering::Equal => {
						Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
							done_swap: Swap {
								amount: done_amount,
								..swap
							},
							invest_amount: *invest_amount,
						})
					}
					Ordering::Less => {
						Ok(
							Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
								swap: Swap {
									// safe because foreign_swap.amount > swap.amount
									amount: foreign_swap.amount - swap.amount,
									..swap
								},
								done_amount,
								invest_amount: *invest_amount,
							},
						)
					}
					// should never occur but let's be safe here
					Ordering::Greater => {
						Err(DispatchError::Arithmetic(ArithmeticError::Overflow))
					}
				}
			},
			_ => Err(DispatchError::Other(
				"Invalid invest state, should automatically be transitioned into state without AndSwapIntoForeignDone",
			)),
		}
	}

	/// Handle increase transitions for the same incoming and outgoing
	/// currencies.
	///
	/// NOTE: We can ignore handling all states which include
	/// `SwapIntoForeignDone` without `ActiveSwapIntoForeignCurrency` as we
	/// consume the done amount and transition in the post transition phase.
	/// Moreover, we can ignore any state which involves an active swap, i.e.
	/// `ActiveSwapInto{Pool, Return}Currency`, as these must not exist if the
	/// in and out currency is the same.
	/// To be safe and to not make any unhandled assumptions, we throw
	/// `DispatchError::Other` for these states though we need to make sure
	/// this can never occur!
	fn handle_increase_non_foreign(
		&self,
		swap: Swap<Balance, Currency>,
	) -> Result<Self, DispatchError> {
		match &self {
			Self::NoState => Ok(Self::InvestmentOngoing {
				invest_amount: swap.amount,
			}),
			Self::InvestmentOngoing { invest_amount } => Ok(Self::InvestmentOngoing {
				invest_amount: invest_amount.ensure_add(swap.amount)?,
			}),
			Self::ActiveSwapIntoPoolCurrency { .. }
			| Self::ActiveSwapIntoForeignCurrency { .. }
			| Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing { .. }
			| Self::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing { .. }
			| Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone { .. }
			| Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
				..
			} => Err(DispatchError::Other(
				"Invalid invest state when transitioning an increased swap order with the same in- \
				 and outgoing currency",
			)),
			_ => Err(DispatchError::Other(
				"Invalid invest state, should automatically be transitioned into state without \
				 AndSwapIntoForeignDone",
			)),
		}
	}

	/// Handle decrease transitions for the same incoming and outgoing
	/// currencies.
	///
	/// NOTE: We can ignore handling all states which include
	/// `SwapIntoForeignDone` without `ActiveSwapIntoForeignCurrency` as we
	/// consume the done amount and transition in the post transition phase.
	/// Moreover, we can ignore any state which involves an active swap, i.e.
	/// `ActiveSwapInto{Pool, Return}Currency`, as these must not exist if the
	/// in and out currency is the same.
	/// To be safe and to not make any unhandled assumptions, we throw
	/// `DispatchError::Other` for these states though we need to make sure
	/// this can never occur!
	fn handle_decrease_non_foreign(
		&self,
		swap: Swap<Balance, Currency>,
	) -> Result<Self, DispatchError> {
		if let Self::InvestmentOngoing { invest_amount } = &self {
			if swap.amount < *invest_amount {
				Ok(InvestState::SwapIntoForeignDoneAndInvestmentOngoing {
					done_swap: swap,
					invest_amount: invest_amount.ensure_sub(swap.amount)?,
				})
			} else {
				Ok(Self::SwapIntoForeignDone { done_swap: swap })
			}
		}
		// should never occur but let's be safe here
		else {
			Err(DispatchError::Other(
				"Invalid invest state when transitioning a decreased swap order with the same in- \
				 and outgoing currency",
			))
		}
	}

	/// Update or kill the state's unprocessed investing amount.
	/// * If the state includes `InvestmentOngoing`, either update or remove the
	///   invested amount.
	/// * Else the unprocessed amount should be zero. If it is not, state is
	///   corrupted as this reflects the investment was increased improperly.
	fn handle_collect(&self, unprocessed_amount: Balance) -> Result<Self, DispatchError> {
		match *self {
			Self::InvestmentOngoing { .. } => {
				if unprocessed_amount.is_zero() {
					Ok(Self::NoState)
				} else {
					Ok(Self::InvestmentOngoing {
						invest_amount: unprocessed_amount,
					})
				}
			}
			Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing { swap, .. } => {
				if unprocessed_amount.is_zero() {
					Ok(Self::ActiveSwapIntoPoolCurrency { swap })
				} else {
					Ok(Self::ActiveSwapIntoPoolCurrencyAndInvestmentOngoing {
						swap,
						invest_amount: unprocessed_amount,
					})
				}
			}
			Self::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing { swap, .. } => {
				if unprocessed_amount.is_zero() {
					Ok(Self::ActiveSwapIntoForeignCurrency { swap })
				} else {
					Ok(Self::ActiveSwapIntoForeignCurrencyAndInvestmentOngoing {
						swap,
						invest_amount: unprocessed_amount,
					})
				}
			}
			Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
				swap,
				done_amount,
				..
			} => {
				if unprocessed_amount.is_zero() {
					Ok(
						Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDone {
							swap,
							done_amount,
						},
					)
				} else {
					Ok(
						Self::ActiveSwapIntoPoolCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
							swap,
							done_amount,
							invest_amount: unprocessed_amount,
						},
					)
				}
			}
			Self::SwapIntoForeignDoneAndInvestmentOngoing { done_swap, .. } => {
				if unprocessed_amount.is_zero() {
					Ok(Self::SwapIntoForeignDone { done_swap })
				} else {
					Ok(Self::SwapIntoForeignDoneAndInvestmentOngoing {
						done_swap,
						invest_amount: unprocessed_amount,
					})
				}
			}
			Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
				swap,
				done_amount,
				..
			} => {
				if unprocessed_amount.is_zero() {
					Ok(Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDone {
						swap,
						done_amount,
					})
				} else {
					Ok(Self::ActiveSwapIntoForeignCurrencyAndSwapIntoForeignDoneAndInvestmentOngoing {
						swap,
						done_amount,
						invest_amount: unprocessed_amount,
					})
				}
			}
			state => {
				if unprocessed_amount.is_zero() {
					Ok(state)
				} else {
					Err(DispatchError::Other(
						"Invalid invest state when transitioning epoch execution",
					))
				}
			}
		}
	}
}
