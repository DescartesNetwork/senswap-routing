use crate::interfaces::iswap::ISwap;
use solana_program::{
  account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
};

pub struct XSwap {}

impl XSwap {
  ///
  /// Add liquidity
  ///
  pub fn add_liquidity<'a>(
    delta_s: u64,
    delta_a: u64,
    delta_b: u64,
    owner_acc: &AccountInfo<'a>,
    pool_acc: &AccountInfo<'a>,
    lpt_acc: &AccountInfo<'a>,
    mint_lpt_acc: &AccountInfo<'a>,
    src_s_acc: &AccountInfo<'a>,
    treasury_s_acc: &AccountInfo<'a>,
    src_a_acc: &AccountInfo<'a>,
    treasury_a_acc: &AccountInfo<'a>,
    src_b_acc: &AccountInfo<'a>,
    treasury_b_acc: &AccountInfo<'a>,
    treasurer: &AccountInfo<'a>,
    splt_program: &AccountInfo<'a>,
    swap_program: &AccountInfo<'a>,
    seed: &[&[&[u8]]],
  ) -> ProgramResult {
    let ix = ISwap::add_liquidity(
      delta_s,
      delta_a,
      delta_b,
      *owner_acc.key,
      *pool_acc.key,
      *lpt_acc.key,
      *mint_lpt_acc.key,
      *src_s_acc.key,
      *treasury_s_acc.key,
      *src_a_acc.key,
      *treasury_a_acc.key,
      *src_b_acc.key,
      *treasury_b_acc.key,
      *treasurer.key,
      *splt_program.key,
      *swap_program.key,
    )?;
    invoke_signed(
      &ix,
      &[
        owner_acc.clone(),
        pool_acc.clone(),
        lpt_acc.clone(),
        mint_lpt_acc.clone(),
        src_s_acc.clone(),
        treasury_s_acc.clone(),
        src_a_acc.clone(),
        treasury_a_acc.clone(),
        src_b_acc.clone(),
        treasury_b_acc.clone(),
        treasurer.clone(),
        splt_program.clone(),
        swap_program.clone(),
      ],
      seed,
    )?;
    Ok(())
  }
  ///
  /// Remove liquidity
  ///
  pub fn remove_liquidity<'a>(
    lpt: u64,
    owner_acc: &AccountInfo<'a>,
    pool_acc: &AccountInfo<'a>,
    lpt_acc: &AccountInfo<'a>,
    mint_lpt_acc: &AccountInfo<'a>,
    dst_s_acc: &AccountInfo<'a>,
    treasury_s_acc: &AccountInfo<'a>,
    dst_a_acc: &AccountInfo<'a>,
    treasury_a_acc: &AccountInfo<'a>,
    dst_b_acc: &AccountInfo<'a>,
    treasury_b_acc: &AccountInfo<'a>,
    treasurer: &AccountInfo<'a>,
    splt_program: &AccountInfo<'a>,
    swap_program: &AccountInfo<'a>,
    seed: &[&[&[u8]]],
  ) -> ProgramResult {
    let ix = ISwap::remove_liquidity(
      lpt,
      *owner_acc.key,
      *pool_acc.key,
      *lpt_acc.key,
      *mint_lpt_acc.key,
      *dst_s_acc.key,
      *treasury_s_acc.key,
      *dst_a_acc.key,
      *treasury_a_acc.key,
      *dst_b_acc.key,
      *treasury_b_acc.key,
      *treasurer.key,
      *splt_program.key,
      *swap_program.key,
    )?;
    invoke_signed(
      &ix,
      &[
        owner_acc.clone(),
        pool_acc.clone(),
        lpt_acc.clone(),
        mint_lpt_acc.clone(),
        dst_s_acc.clone(),
        treasury_s_acc.clone(),
        dst_a_acc.clone(),
        treasury_a_acc.clone(),
        dst_b_acc.clone(),
        treasury_b_acc.clone(),
        treasurer.clone(),
        splt_program.clone(),
        swap_program.clone(),
      ],
      seed,
    )?;
    Ok(())
  }
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
