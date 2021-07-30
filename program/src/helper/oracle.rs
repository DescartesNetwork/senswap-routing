const FEE: u64 = 2500000; // 0.25%
const EARNING: u64 = 500000; // 0.05%
const DECIMALS: u64 = 1000000000; // 10^9

pub struct Oracle {}

impl Oracle {
  pub fn curve(new_bid_reserve: u64, bid_reserve: u64, ask_reserve: u64) -> Option<u64> {
    if new_bid_reserve == 0 || bid_reserve == 0 || ask_reserve == 0 {
      return None;
    }
    let new_ask_reserve = (bid_reserve as u128)
      .checked_mul(ask_reserve as u128)?
      .checked_div(new_bid_reserve as u128)? as u64;
    if new_ask_reserve == 0 {
      return None;
    }

    Some(new_ask_reserve)
  }

  pub fn curve_in_fee(
    new_bid_reserve: u64,
    bid_reserve: u64,
    ask_reserve: u64,
    is_exempted: bool,
  ) -> Option<(u64, u64, u64)> {
    let new_ask_reserve_without_fee = Self::curve(new_bid_reserve, bid_reserve, ask_reserve)?;
    let paid_amount_without_fee = ask_reserve.checked_sub(new_ask_reserve_without_fee)?;

    let fee = (paid_amount_without_fee as u128)
      .checked_mul(FEE as u128)?
      .checked_div(DECIMALS as u128)? as u64;
    let mut earning: u64 = 0;
    if !is_exempted {
      earning = (paid_amount_without_fee as u128)
        .checked_mul(EARNING as u128)?
        .checked_div(DECIMALS as u128)? as u64;
    }

    let paid_amount = paid_amount_without_fee
      .checked_sub(fee)?
      .checked_sub(earning)?;
    let new_ask_reserve = new_ask_reserve_without_fee.checked_add(fee)?;
    Some((new_ask_reserve, paid_amount, earning))
  }
}
