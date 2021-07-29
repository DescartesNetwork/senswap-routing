use crate::interfaces::iswap::ISwap;
use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
};

pub struct XSwap {}

impl XSwap {
  ///
  /// Initialize account
  ///
  pub fn swap<'a>(
    amount: u64,
    limit: u64,
    payer_acc: &AccountInfo<'a>,
    pool_acc: &AccountInfo<'a>,
    vault_acc: &AccountInfo<'a>,
    src_acc: &AccountInfo<'a>,
    treasury_bid_acc: &AccountInfo<'a>,
    dst_acc: &AccountInfo<'a>,
    treasury_ask_acc: &AccountInfo<'a>,
    treasury_sen_acc: &AccountInfo<'a>,
    treasurer: &AccountInfo<'a>,
    splt_program: &AccountInfo<'a>,
    swap_program: &AccountInfo<'a>,
    seed: &[&[&[u8]]],
  ) -> ProgramResult {
    let ix = ISwap::swap(
      amount,
      limit,
      *payer_acc.key,
      *pool_acc.key,
      *vault_acc.key,
      *src_acc.key,
      *treasury_bid_acc.key,
      *dst_acc.key,
      *treasury_ask_acc.key,
      *treasury_sen_acc.key,
      *treasurer.key,
      *splt_program.key,
      *swap_program.key,
    )?;
    invoke_signed(
      &ix,
      &[
        payer_acc.clone(),
        pool_acc.clone(),
        vault_acc.clone(),
        src_acc.clone(),
        treasury_bid_acc.clone(),
        dst_acc.clone(),
        treasury_ask_acc.clone(),
        treasury_sen_acc.clone(),
        treasurer.clone(),
        splt_program.clone(),
        swap_program.clone(),
      ],
      seed,
    )?;
    Ok(())
  }
}
