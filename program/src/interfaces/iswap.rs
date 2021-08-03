use solana_program::{
  instruction::{AccountMeta, Instruction},
  program_error::ProgramError,
  pubkey::Pubkey,
};
use std::mem::size_of;

pub struct ISwap {}

impl ISwap {
  ///
  /// Add liquidity
  ///
  pub fn add_liquidity(
    delta_s: u64,
    delta_a: u64,
    delta_b: u64,
    owner_acc: Pubkey,
    pool_acc: Pubkey,
    lpt_acc: Pubkey,
    mint_lpt_acc: Pubkey,
    src_s_acc: Pubkey,
    treasury_s_acc: Pubkey,
    src_a_acc: Pubkey,
    treasury_a_acc: Pubkey,
    src_b_acc: Pubkey,
    treasury_b_acc: Pubkey,
    treasurer: Pubkey,
    splt_program: Pubkey,
    program_id: Pubkey,
  ) -> Result<Instruction, ProgramError> {
    // Build data
    let mut data = Vec::with_capacity(size_of::<Self>());
    // Add Liquidity - Code 1
    data.push(1);
    data.extend_from_slice(&delta_s.to_le_bytes());
    data.extend_from_slice(&delta_a.to_le_bytes());
    data.extend_from_slice(&delta_b.to_le_bytes());
    // Build accounts
    let mut accounts = Vec::with_capacity(12);
    accounts.push(AccountMeta::new(owner_acc, true));
    accounts.push(AccountMeta::new(pool_acc, false));
    accounts.push(AccountMeta::new(lpt_acc, false));
    accounts.push(AccountMeta::new(mint_lpt_acc, false));
    accounts.push(AccountMeta::new(src_s_acc, false));
    accounts.push(AccountMeta::new(treasury_s_acc, false));
    accounts.push(AccountMeta::new(src_a_acc, false));
    accounts.push(AccountMeta::new(treasury_a_acc, false));
    accounts.push(AccountMeta::new(src_b_acc, false));
    accounts.push(AccountMeta::new(treasury_b_acc, false));
    accounts.push(AccountMeta::new_readonly(treasurer, false));
    accounts.push(AccountMeta::new_readonly(splt_program, false));
    // Return
    Ok(Instruction {
      program_id,
      accounts,
      data,
    })
  }
  ///
  /// Remove liquidity
  ///
  pub fn remove_liquidity(
    lpt: u64,
    owner_acc: Pubkey,
    pool_acc: Pubkey,
    lpt_acc: Pubkey,
    mint_lpt_acc: Pubkey,
    dst_s_acc: Pubkey,
    treasury_s_acc: Pubkey,
    dst_a_acc: Pubkey,
    treasury_a_acc: Pubkey,
    dst_b_acc: Pubkey,
    treasury_b_acc: Pubkey,
    treasurer: Pubkey,
    splt_program: Pubkey,
    program_id: Pubkey,
  ) -> Result<Instruction, ProgramError> {
    // Build data
    let mut data = Vec::with_capacity(size_of::<Self>());
    // Add Liquidity - Code 2
    data.push(2);
    data.extend_from_slice(&lpt.to_le_bytes());
    // Build accounts
    let mut accounts = Vec::with_capacity(11);
    accounts.push(AccountMeta::new(owner_acc, true));
    accounts.push(AccountMeta::new(pool_acc, false));
    accounts.push(AccountMeta::new(lpt_acc, false));
    accounts.push(AccountMeta::new(mint_lpt_acc, false));
    accounts.push(AccountMeta::new(dst_s_acc, false));
    accounts.push(AccountMeta::new(treasury_s_acc, false));
    accounts.push(AccountMeta::new(dst_a_acc, false));
    accounts.push(AccountMeta::new(treasury_a_acc, false));
    accounts.push(AccountMeta::new(dst_b_acc, false));
    accounts.push(AccountMeta::new(treasury_b_acc, false));
    accounts.push(AccountMeta::new_readonly(treasurer, false));
    accounts.push(AccountMeta::new_readonly(splt_program, false));
    // Return
    Ok(Instruction {
      program_id,
      accounts,
      data,
    })
  }
  ///
  /// Initialize account
  ///
  pub fn swap(
    amount: u64,
    limit: u64,
    payer_acc: Pubkey,
    pool_acc: Pubkey,
    vault_acc: Pubkey,
    src_acc: Pubkey,
    treasury_bid_acc: Pubkey,
    dst_acc: Pubkey,
    treasury_ask_acc: Pubkey,
    treasury_sen_acc: Pubkey,
    treasurer: Pubkey,
    splt_program: Pubkey,
    program_id: Pubkey,
  ) -> Result<Instruction, ProgramError> {
    // Build data
    let mut data = Vec::with_capacity(size_of::<Self>());
    // Swap - Code 3
    data.push(3);
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&limit.to_le_bytes());
    // Build accounts
    let mut accounts = Vec::with_capacity(10);
    accounts.push(AccountMeta::new(payer_acc, true));
    accounts.push(AccountMeta::new(pool_acc, false));
    accounts.push(AccountMeta::new(vault_acc, false));
    accounts.push(AccountMeta::new(src_acc, false));
    accounts.push(AccountMeta::new(treasury_bid_acc, false));
    accounts.push(AccountMeta::new(dst_acc, false));
    accounts.push(AccountMeta::new(treasury_ask_acc, false));
    accounts.push(AccountMeta::new(treasury_sen_acc, false));
    accounts.push(AccountMeta::new_readonly(treasurer, false));
    accounts.push(AccountMeta::new_readonly(splt_program, false));
    // Return
    Ok(Instruction {
      program_id,
      accounts,
      data,
    })
  }
}
