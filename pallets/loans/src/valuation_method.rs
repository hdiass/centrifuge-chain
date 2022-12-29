// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! Module provides functionality for different loan types
use scale_info::TypeInfo;

use super::*;

/// different types of loans
#[derive(Encode, Decode, Copy, Clone, PartialEq, TypeInfo)]
#[cfg_attr(any(feature = "std", feature = "runtime-benchmarks"), derive(Debug))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum ValuationMethod<Rate> {
	DiscountedCashFlows(DiscountedCashFlows<Rate>),
	OutstandingDebt(OutstandingDebt),
}

impl<Rate> ValuationMethod<Rate>
where
	Rate: FixedPointNumber,
{
	pub(crate) fn maturity_date(&self) -> Option<Moment> {
		match self {
			ValuationMethod::DiscountedCashFlows(bl) => Some(bl.maturity_date),
			ValuationMethod::OutstandingDebt(_) => None,
		}
	}

	pub(crate) fn is_valid(&self, now: Moment) -> bool {
		match self {
			ValuationMethod::DiscountedCashFlows(bl) => bl.is_valid(now),
			ValuationMethod::OutstandingDebt(cl) => cl.is_valid(),
		}
	}
}

/// The data structure for Bullet loan type
#[derive(Encode, Decode, Copy, Clone, PartialEq, TypeInfo)]
#[cfg_attr(any(feature = "std", feature = "runtime-benchmarks"), derive(Debug))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct DiscountedCashFlows<Rate> {
	probability_of_default: Rate,
	loss_given_default: Rate,
	discount_rate: Rate,
}

impl<Rate> DiscountedCashFlows<Rate>
where
	Rate: FixedPointNumber,
{
	pub fn new(
		probability_of_default: Rate,
		loss_given_default: Rate,
		discount_rate: Rate,
	) -> Self {
		Self {
			probability_of_default,
			discount_rate,
			loss_given_default,
		}
	}

	/// Calculate the present value based on the discounted future cash flows
	pub fn present_value(
		&self,
		debt: Balance,
		origination_date: Option<Moment>,
		now: Moment,
		interest_rate_per_sec: Rate,
	) -> Option<Balance> {
		if debt.is_zero() {
			return Some(Balance::zero());
		}
	
		// If the loan is overdue, there are no future cash flows to discount,
		// hence we use the outstanding debt as the value.
		if now > maturity_date {
			return Some(debt);
		}
	
		// Calculate the expected loss over the term of the loan
		let tel = Rate::saturating_from_rational(maturity_date - origination_date, seconds_per_year())
			.checked_mul(&pd)
			.and_then(|val| val.checked_mul(&lgd))
			.map(|tel| tel.min(One::one()))?;
		let tel_inv = Rate::one().checked_sub(&tel)?;

		// Calculate the risk-adjusted expected cash flows
		let acc_rate = checked_pow(interest_rate_per_sec, (maturity_date - now) as usize)?;
		let ecf = acc_rate.checked_mul_int(debt)?;
		let ra_ecf = tel_inv.checked_mul_int(ecf)?;

		// Discount the risk-adjust expected cash flows
		let rate = checked_pow(discount_rate, (maturity - now) as usize)?;
		let d = rate.reciprocal()?;
		d.checked_mul_int(ra_ecf)
	}

	/// validates the bullet loan parameters
	pub fn is_valid(&self, now: Moment) -> bool {
		vec![
			// discount should always be >= 1
			self.discount_rate >= One::one(),
			// maturity date should always be in future where now is at this instant
			self.maturity_date > now,
		]
		.into_iter()
		.all(|is_positive| is_positive)
	}
}

/// The data structure for outstanding debt based valuation
#[derive(Encode, Decode, Copy, Clone, PartialEq, TypeInfo)]
#[cfg_attr(any(feature = "std", feature = "runtime-benchmarks"), derive(Debug))]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OutstandingDebt {
}

impl OutstandingDebt {
	#[allow(dead_code)]
	pub fn new() -> Self {
		Self { }
	}

	/// calculates the present value of the credit line loan
	/// https://centrifuge.hackmd.io/uJ3AXBUoQCijSIH9He-NxA#Present-value1
	/// The debt = current outstanding debt * (1 - written off percentage)
	pub fn present_value(&self, debt: Balance) -> Option<Balance> {
		Some(debt)
	}

	/// validates credit line loan parameters
	pub fn is_valid(&self) -> bool {
		true
	}

	/// calculates max_borrow_amount for credit line loan,
	/// max_borrow_amount = advance_rate * collateral_value - debt
	/// https://centrifuge.hackmd.io/uJ3AXBUoQCijSIH9He-NxA#Ceiling1
	pub fn max_borrow_amount(&self, debt: Balance) -> Option<Balance>
	where
		Rate: FixedPointNumber,
		Balance: FixedPointOperand + BaseArithmetic,
	{
		math::max_borrow_amount(self.advance_rate, self.value, debt)
	}
}

#[cfg(test)]
mod tests {
	use cfg_primitives::CFG as CURRENCY;
	use cfg_types::fixed_point::Rate;

	use super::*;

	#[test]
	fn test_bullet_loan_is_valid() {
		let ad = Rate::one();
		let cv: u128 = One::one();
		let pd = Rate::zero();
		let lgd = Rate::zero();
		let now = 200;

		// discount_rate is less than one
		let dr = Zero::zero();
		let md = 300;
		let bl = DiscountedCashFlows::new(ad, pd, lgd, cv, dr, md);
		assert!(!bl.is_valid(now));

		// maturity is in the past
		let dr = Rate::from_inner(1000000001268391679350583460);
		let md = 100;
		let bl = DiscountedCashFlows::new(ad, pd, lgd, cv, dr, md);
		assert!(!bl.is_valid(now));

		// maturity date is at this instant
		let md = 200;
		let bl = DiscountedCashFlows::new(ad, pd, lgd, cv, dr, md);
		assert!(!bl.is_valid(now));

		// valid data
		let md = 500;
		let bl = DiscountedCashFlows::new(ad, pd, lgd, cv, dr, md);
		assert!(bl.is_valid(now));
	}

	#[test]
	fn test_credit_line_max_borrow_amount() {
		let ad = Rate::saturating_from_rational(80, 100);
		let value: u128 = 100 * CURRENCY;
		let cl = OutstandingDebt::new(ad, value);

		// debt can be more
		let debt: u128 = 120 * CURRENCY;
		assert_eq!(cl.max_borrow_amount(debt), None);

		// debt can be same
		let debt: u128 = 80 * CURRENCY;
		assert_eq!(cl.max_borrow_amount(debt), Some(0));

		// debt can be less
		let debt: u128 = 70 * CURRENCY;
		assert_eq!(cl.max_borrow_amount(debt), Some(10 * CURRENCY));
	}
}
