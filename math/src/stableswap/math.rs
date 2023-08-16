use crate::stableswap::types::{target_precision, AssetReserve};
use crate::to_u256;
use crate::types::Balance;
use num_traits::{CheckedDiv, CheckedMul, One, Zero};
use primitive_types::U256;
use sp_arithmetic::{FixedPointNumber, FixedU128, Permill};
use sp_std::ops::Div;
use sp_std::prelude::*;

pub const MAX_Y_ITERATIONS: u8 = 128;
pub const MAX_D_ITERATIONS: u8 = 64;

const PRECISION: u8 = 1;

/// Calculating amount to be received from the pool given the amount to be sent to the pool and both reserves.
/// D - number of iterations to use for Newton's formula to calculate parameter D ( it should be >=1 otherwise it wont converge at all and will always fail
/// Y - number of iterations to use for Dewton's formula to calculate reserve Y ( it should be >=1 otherwise it wont converge at all and will always fail
pub fn calculate_out_given_in<const D: u8, const Y: u8>(
	balances: &[AssetReserve],
	idx_in: usize,
	idx_out: usize,
	amount_in: Balance,
	amplification: Balance,
) -> Option<Balance> {
	if idx_in >= balances.len() || idx_out >= balances.len() {
		return None;
	}
	let target_precision = target_precision(balances);
	let reserves = normalize_reserves(balances);
	let amount_in = normalize_value(amount_in, balances[idx_in].decimals, target_precision, Rounding::Down);
	let new_reserve_out = calculate_y_given_in::<D, Y>(amount_in, idx_in, idx_out, &reserves, amplification)?;

	let amount_out = reserves[idx_out].checked_sub(new_reserve_out)?;
	let amount_out = normalize_value(amount_out, target_precision, balances[idx_out].decimals, Rounding::Down);
	Some(amount_out)
}

/// Calculating amount to be sent to the pool given the amount to be received from the pool and both reserves.
/// D - number of iterations to use for Newton's formula ( it should be >=1 otherwise it wont converge at all and will always fail
/// Y - number of iterations to use for Dewton's formula to calculate reserve Y ( it should be >=1 otherwise it wont converge at all and will always fail
pub fn calculate_in_given_out<const D: u8, const Y: u8>(
	balances: &[AssetReserve],
	idx_in: usize,
	idx_out: usize,
	amount_out: Balance,
	amplification: Balance,
) -> Option<Balance> {
	if idx_in >= balances.len() || idx_out >= balances.len() {
		return None;
	}
	let target_precision = target_precision(balances);
	let reserves = normalize_reserves(balances);
	let amount_out = normalize_value(amount_out, balances[idx_out].decimals, target_precision, Rounding::Down);
	let new_reserve_in = calculate_y_given_out::<D, Y>(amount_out, idx_in, idx_out, &reserves, amplification)?;
	let amount_in = new_reserve_in.checked_sub(reserves[idx_in])?;
	let amount_in = normalize_value(amount_in, target_precision, balances[idx_in].decimals, Rounding::Up);
	Some(amount_in)
}

/// Calculating amount to be received from the pool given the amount to be sent to the pool and both reserves and apply a fee.
pub fn calculate_out_given_in_with_fee<const D: u8, const Y: u8>(
	balances: &[AssetReserve],
	idx_in: usize,
	idx_out: usize,
	amount_in: Balance,
	amplification: Balance,
	fee: Permill,
) -> Option<(Balance, Balance)> {
	let amount_out = calculate_out_given_in::<D, Y>(balances, idx_in, idx_out, amount_in, amplification)?;
	let fee_amount = calculate_fee_amount(amount_out, fee, Rounding::Down);

	let amount_out = amount_out.checked_sub(fee_amount)?;

	Some((amount_out, fee_amount))
}

/// Calculating amount to be sent to the pool given the amount to be received from the pool and both reserves with fee applied.
pub fn calculate_in_given_out_with_fee<const D: u8, const Y: u8>(
	balances: &[AssetReserve],
	idx_in: usize,
	idx_out: usize,
	amount_out: Balance,
	amplification: Balance,
	fee: Permill,
) -> Option<(Balance, Balance)> {
	let amount_in = calculate_in_given_out::<D, Y>(balances, idx_in, idx_out, amount_out, amplification)?;
	let fee_amount = calculate_fee_amount(amount_in, fee, Rounding::Up);

	let amount_in = amount_in.checked_add(fee_amount)?;

	Some((amount_in, fee_amount))
}

/// Calculate amount of shares to be given to LP after LP provided liquidity of some assets to the pool.
pub fn calculate_shares<const D: u8>(
	initial_reserves: &[AssetReserve],
	updated_reserves: &[AssetReserve],
	amplification: Balance,
	share_issuance: Balance,
) -> Option<Balance> {
	if initial_reserves.len() != updated_reserves.len() {
		return None;
	}
	let initial_reserves = normalize_reserves(initial_reserves);
	let updated_reserves = normalize_reserves(updated_reserves);

	let initial_d = calculate_d_internal::<D>(&initial_reserves, amplification)?;

	// We must make sure the updated_d is rounded *down* so that we are not giving the new position too many shares.
	// calculate_d can return a D value that is above the correct D value by up to 2, so we subtract 2.
	let updated_d = calculate_d_internal::<D>(&updated_reserves, amplification)?.checked_sub(2_u128)?;

	if updated_d < initial_d {
		return None;
	}

	if share_issuance == 0 {
		// if first liquidity added
		Some(updated_d)
	} else {
		let (issuance_hp, d_diff, d0) = to_u256!(share_issuance, updated_d.checked_sub(initial_d)?, initial_d);
		let share_amount = issuance_hp.checked_mul(d_diff)?.checked_div(d0)?;
		Balance::try_from(share_amount).ok()
	}
}

/// Calculate amount of shares to be given to LP after LP provided liquidity of some assets to the pool.
pub fn calculate_shares_for_amount<const D: u8>(
	initial_reserves: &[AssetReserve],
	asset_idx: usize,
	amount: Balance,
	amplification: Balance,
	share_issuance: Balance,
	fee: Permill,
) -> Option<Balance> {
	if asset_idx >= initial_reserves.len() {
		return None;
	}
	let amount = normalize_value(
		amount,
		initial_reserves[asset_idx].decimals,
		target_precision(&initial_reserves),
		Rounding::Down,
	);
	let n_coins = initial_reserves.len();
	let fixed_fee = FixedU128::from(fee);
	let fee = fixed_fee
		.checked_mul(&FixedU128::from(n_coins as u128))?
		.checked_div(&FixedU128::from(4 * (n_coins - 1) as u128))?;

	let initial_reserves = normalize_reserves(initial_reserves);

	let new_reserve_in = initial_reserves[asset_idx].checked_sub(amount)?;

	let updated_reserves: Vec<Balance> = initial_reserves
		.iter()
		.enumerate()
		.map(|(idx, v)| if idx == asset_idx { new_reserve_in } else { *v })
		.collect();

	let initial_d = calculate_d_internal::<D>(&initial_reserves, amplification)?;
	let updated_d = calculate_d_internal::<D>(&updated_reserves, amplification)?;

	let (d1, d0, asset_reserve) = to_u256!(updated_d, initial_d, initial_reserves[asset_idx]);

	let ideal_balance = d1.checked_mul(asset_reserve)?.checked_div(d0)?;

	let diff = Balance::try_from(asset_reserve.abs_diff(ideal_balance)).ok()?;

	let fee_amount = fee.checked_mul_int(diff)?;

	let adjusted_balances: Vec<Balance> = updated_reserves
		.iter()
		.enumerate()
		.map(|(idx, v)| {
			if idx == asset_idx {
				v.saturating_sub(fee_amount)
			} else {
				*v
			}
		})
		.collect();

	let adjusted_d = calculate_d_internal::<D>(&adjusted_balances, amplification)?;

	let (d_diff, issuance_hp) = to_u256!(initial_d.checked_sub(adjusted_d)?, share_issuance);

	let share_amount = issuance_hp
		.checked_mul(d_diff)?
		.checked_div(d0)?
		.checked_add(U256::one())?;
	Balance::try_from(share_amount).ok()
}

/// Given amount of shares and asset reserves, calculate corresponding amount of selected asset to be withdrawn.
pub fn calculate_withdraw_one_asset<const D: u8, const Y: u8>(
	reserves: &[AssetReserve],
	shares: Balance,
	asset_index: usize,
	share_asset_issuance: Balance,
	amplification: Balance,
	fee: Permill,
) -> Option<(Balance, Balance)> {
	if share_asset_issuance.is_zero() {
		return None;
	}

	if asset_index >= reserves.len() {
		return None;
	}

	if shares > share_asset_issuance {
		return None;
	}

	let n_coins = reserves.len();
	if n_coins <= 1 {
		return None;
	}
	let target_precision = target_precision(reserves);
	let asset_out_decimals = reserves[asset_index].decimals;
	let reserves = normalize_reserves(reserves);

	let fixed_fee = FixedU128::from(fee);
	let fee = fixed_fee
		.checked_mul(&FixedU128::from(n_coins as u128))?
		.checked_div(&FixedU128::from(4 * (n_coins - 1) as u128))?;

	let initial_d = calculate_d_internal::<D>(&reserves, amplification)?;

	let (shares_hp, issuance_hp, d_hp) = to_u256!(shares, share_asset_issuance, initial_d);

	let d1 = d_hp.checked_sub(shares_hp.checked_mul(d_hp)?.checked_div(issuance_hp)?)?;

	let xp: Vec<Balance> = reserves
		.iter()
		.enumerate()
		.filter(|(idx, _)| *idx != asset_index)
		.map(|(_, v)| *v)
		.collect();

	let y = calculate_y_internal::<Y>(&xp, Balance::try_from(d1).ok()?, amplification)?;

	let xp_hp: Vec<U256> = reserves.iter().map(|v| to_u256!(*v)).collect();

	let y_hp = to_u256!(y);

	let mut reserves_reduced: Vec<Balance> = Vec::new();
	let mut asset_reserve: Balance = Balance::zero();

	for (idx, reserve) in xp_hp.iter().enumerate() {
		let dx_expected = if idx == asset_index {
			// dx_expected = xp[j] * d1 / d0 - new_y
			reserve.checked_mul(d1)?.checked_div(d_hp)?.checked_sub(y_hp)?
		} else {
			// dx_expected = xp[j] - xp[j] * d1 / d0
			reserve.checked_sub(reserve.checked_mul(d1)?.checked_div(d_hp)?)?
		};

		let expected = Balance::try_from(dx_expected).ok()?;
		let reduced = Balance::try_from(*reserve)
			.ok()?
			.checked_sub(fee.checked_mul_int(expected)?)?;

		if idx != asset_index {
			reserves_reduced.push(reduced);
		} else {
			asset_reserve = reduced;
		}
	}

	let y1 = calculate_y_internal::<Y>(&reserves_reduced, Balance::try_from(d1).ok()?, amplification)?;

	let dy = asset_reserve.checked_sub(y1)?;

	let dy_0 = reserves[asset_index].checked_sub(y)?;

	let fee = dy_0.checked_sub(dy)?;

	let amount_out = normalize_value(dy, target_precision, asset_out_decimals, Rounding::Down);
	let fee = normalize_value(fee, target_precision, asset_out_decimals, Rounding::Down);
	Some((amount_out, fee))
}

pub fn calculate_d<const D: u8>(reserves: &[AssetReserve], amplification: Balance) -> Option<Balance> {
	let balances = normalize_reserves(reserves);
	calculate_d_internal::<D>(&balances, amplification)
}

/// amplification * n^n where n is number of assets in pool.
pub(crate) fn calculate_ann(len: usize, amplification: Balance) -> Option<Balance> {
	(0..len).try_fold(amplification, |acc, _| acc.checked_mul(len as u128))
}

pub(crate) fn calculate_y_given_in<const D: u8, const Y: u8>(
	amount: Balance,
	idx_in: usize,
	idx_out: usize,
	balances: &[Balance],
	amplification: Balance,
) -> Option<Balance> {
	if idx_in >= balances.len() || idx_out >= balances.len() {
		return None;
	}

	let new_reserve_in = balances[idx_in].checked_add(amount)?;

	let d = calculate_d_internal::<D>(balances, amplification)?;

	let xp: Vec<Balance> = balances
		.iter()
		.enumerate()
		.filter(|(idx, _)| *idx != idx_out)
		.map(|(idx, v)| if idx == idx_in { new_reserve_in } else { *v })
		.collect();

	calculate_y_internal::<Y>(&xp, d, amplification)
}

/// Calculate new amount of reserve ID given amount to be withdrawn from the pool
pub(crate) fn calculate_y_given_out<const D: u8, const Y: u8>(
	amount: Balance,
	idx_in: usize,
	idx_out: usize,
	balances: &[Balance],
	amplification: Balance,
) -> Option<Balance> {
	if idx_in >= balances.len() || idx_out >= balances.len() {
		return None;
	}
	let new_reserve_out = balances[idx_out].checked_sub(amount)?;

	let d = calculate_d_internal::<D>(balances, amplification)?;
	let xp: Vec<Balance> = balances
		.iter()
		.enumerate()
		.filter(|(idx, _)| *idx != idx_in)
		.map(|(idx, v)| if idx == idx_out { new_reserve_out } else { *v })
		.collect();

	calculate_y_internal::<Y>(&xp, d, amplification)
}

pub(crate) fn calculate_d_internal<const D: u8>(xp: &[Balance], amplification: Balance) -> Option<Balance> {
	let two_u256 = to_u256!(2_u128);

	// Filter out zero balance assets, and return error if there is one.
	// Either all assets are zero balance, or none are zero balance.
	// Otherwise, it breaks the math.
	let mut xp_hp: Vec<U256> = xp.iter().filter(|v| !(*v).is_zero()).map(|v| to_u256!(*v)).collect();
	if xp_hp.len() != xp.len() && !xp_hp.is_empty() {
		return None;
	}

	xp_hp.sort();

	let ann = calculate_ann(xp_hp.len(), amplification)?;

	let n_coins = to_u256!(xp_hp.len());

	let mut s_hp = U256::zero();

	for x in xp_hp.iter() {
		s_hp = s_hp.checked_add(*x)?;
	}

	if s_hp == U256::zero() {
		return Some(Balance::zero());
	}

	let mut d = s_hp;

	let (ann_hp, precision_hp) = to_u256!(ann, PRECISION as u128);

	for _ in 0..D {
		let d_p = xp_hp
			.iter()
			.try_fold(d, |acc, v| acc.checked_mul(d)?.checked_div(v.checked_mul(n_coins)?))?;
		let d_prev = d;

		d = ann_hp
			.checked_mul(s_hp)?
			.checked_add(d_p.checked_mul(n_coins)?)?
			.checked_mul(d)?
			.checked_div(
				ann_hp
					.checked_sub(U256::one())?
					.checked_mul(d)?
					.checked_add(n_coins.checked_add(U256::one())?.checked_mul(d_p)?)?,
			)?
			// adding two here is sufficient to account for rounding
			// errors, AS LONG AS the minimum reserves are 2 for each
			// asset. I.e., as long as xp_hp[0] >= 2 and xp_hp[1] >= 2
			// adding two guarantees that this function will return
			// a value larger than or equal to the correct D invariant
			.checked_add(two_u256)?;

		if has_converged(d_prev, d, precision_hp) {
			// If runtime-benchmarks - don't return and force max iterations
			#[cfg(not(feature = "runtime-benchmarks"))]
			return Balance::try_from(d).ok();
		}
	}

	Balance::try_from(d).ok()
}

pub fn calculate_y<const D: u8>(
	reserves: &[AssetReserve],
	d: Balance,
	amplification: Balance,
	asset_precision: u8,
) -> Option<Balance> {
	let prec = target_precision(reserves);
	let balances = normalize_reserves(reserves);
	let y = calculate_y_internal::<D>(&balances, d, amplification)?;
	Some(normalize_value(y, prec, asset_precision, Rounding::Down))
}

fn calculate_y_internal<const D: u8>(xp: &[Balance], d: Balance, amplification: Balance) -> Option<Balance> {
	// Filter out zero balance assets, and return error if there is one.
	// Either all assets are zero balance, or none are zero balance.
	// Otherwise, it breaks the math.
	let mut xp_hp: Vec<U256> = xp.iter().filter(|v| !(*v).is_zero()).map(|v| to_u256!(*v)).collect();
	if xp_hp.len() != xp.len() && !xp_hp.is_empty() {
		return None;
	}
	xp_hp.sort();

	let ann = calculate_ann(xp_hp.len().checked_add(1)?, amplification)?;

	let (d_hp, n_coins_hp, ann_hp, precision_hp) = to_u256!(d, xp_hp.len().checked_add(1)?, ann, PRECISION as u128);

	let two_hp = to_u256!(2u128);
	let mut s_hp = U256::zero();
	for x in xp_hp.iter() {
		s_hp = s_hp.checked_add(*x)?;
	}
	let mut c = d_hp;

	for reserve in xp_hp.iter() {
		c = c.checked_mul(d_hp)?.checked_div(reserve.checked_mul(n_coins_hp)?)?;
	}

	c = c.checked_mul(d_hp)?.checked_div(ann_hp.checked_mul(n_coins_hp)?)?;

	let b = s_hp.checked_add(d_hp.checked_div(ann_hp)?)?;
	let mut y = d_hp;

	for _i in 0..D {
		let y_prev = y;
		y = y
			.checked_mul(y)?
			.checked_add(c)?
			.checked_div(two_hp.checked_mul(y)?.checked_add(b)?.checked_sub(d_hp)?)?
			.checked_add(two_hp)?;

		if has_converged(y_prev, y, precision_hp) {
			// If runtime-benchmarks - don't return and force max iterations
			#[cfg(not(feature = "runtime-benchmarks"))]
			return Balance::try_from(y).ok();
		}
	}
	Balance::try_from(y).ok()
}

pub fn calculate_amplification(
	initial_amplification: u128,
	final_amplification: u128,
	initial_block: u128,
	final_block: u128,
	current_block: u128,
) -> u128 {
	// short circuit if block parameters are invalid or start block is not reached yet
	if current_block < initial_block || final_block <= initial_block {
		return initial_amplification;
	}

	// short circuit if already reached desired block
	if current_block >= final_block {
		return final_amplification;
	}

	let step = final_amplification
		.abs_diff(initial_amplification)
		.saturating_mul(current_block.saturating_sub(initial_block))
		.div(final_block.saturating_sub(initial_block));

	if final_amplification > initial_amplification {
		initial_amplification.saturating_add(step)
	} else {
		initial_amplification.saturating_sub(step)
	}
}

#[inline]
fn has_converged(v0: U256, v1: U256, precision: U256) -> bool {
	let diff = abs_diff(v0, v1);

	(v1 <= v0 && diff < precision) || (v1 > v0 && diff <= precision)
}

#[inline]
fn abs_diff(d0: U256, d1: U256) -> U256 {
	if d1 >= d0 {
		// This is safe due the previous condition
		d1 - d0
	} else {
		d0 - d1
	}
}

pub(crate) enum Rounding {
	Down,
	Up,
}

fn calculate_fee_amount(amount: Balance, fee: Permill, rounding: Rounding) -> Balance {
	match rounding {
		Rounding::Down => fee.mul_floor(amount),
		Rounding::Up => fee.mul_ceil(amount),
	}
}

pub(crate) fn normalize_reserves(reserves: &[AssetReserve]) -> Vec<Balance> {
	let t = target_precision(reserves);
	reserves
		.iter()
		.map(|v| normalize_value(v.amount, v.decimals, t, Rounding::Down))
		.collect()
}

pub(crate) fn normalize_value(amount: Balance, decimals: u8, target_decimals: u8, rounding: Rounding) -> Balance {
	if target_decimals == decimals {
		return amount;
	}
	let diff = target_decimals.abs_diff(decimals);
	if target_decimals > decimals {
		amount.saturating_mul(10u128.pow(diff as u32))
	} else {
		match rounding {
			Rounding::Down => amount.div(10u128.pow(diff as u32)),
			Rounding::Up => amount.div(10u128.pow(diff as u32)).saturating_add(Balance::one()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_normalize_value_same_decimals() {
		let amount = 1_000_000_000_000_000;
		let decimals = 12;
		let target_decimals = 12;
		let expected: Balance = amount;
		let actual = normalize_value(amount, decimals, target_decimals, Rounding::Down);
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_normalize_value_target_greater_than_decimals() {
		let amount = 1_000_000_000_000;
		let decimals = 12;
		let target_decimals = 18;
		let expected: Balance = 1_000_000_000_000_000_000;
		let actual = normalize_value(amount, decimals, target_decimals, Rounding::Up);
		assert_eq!(actual, expected);
	}

	#[test]
	fn test_normalize_value_target_less_than_decimals() {
		let amount: Balance = 1_000_000_000_000_000_000;
		let decimals = 18;
		let target_decimals = 12;
		let expected: Balance = 1_000_000_000_000;
		let actual = normalize_value(amount, decimals, target_decimals, Rounding::Down);
		assert_eq!(actual, expected);
	}
}
