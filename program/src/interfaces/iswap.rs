use solana_program::{
  instruction::{AccountMeta, Instruction},
  program_error::ProgramError,
  pubkey::Pubkey,
};
use std::mem::size_of;

pub struct ISwap {}

impl ISwap {
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
