use crate::error::AppError;
use crate::instruction::AppInstruction;
use crate::interfaces::{xsplata::XSPLATA, xswap::XSwap};
use crate::schema::{
  account::{Account, AccountState},
  pool::Pool,
};
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program_pack::Pack,
  pubkey::Pubkey,
};

pub struct Processor {}

impl Processor {
  pub fn process(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
  ) -> ProgramResult {
    let instruction = AppInstruction::unpack(instruction_data)?;
    match instruction {
      AppInstruction::Swap { amount, limit } => {
        msg!("Calling Swap function");
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;
        let pool_acc = next_account_info(accounts_iter)?;
        let vault_acc = next_account_info(accounts_iter)?;
        let src_acc = next_account_info(accounts_iter)?;
        let treasury_bid_acc = next_account_info(accounts_iter)?;
        let dst_acc = next_account_info(accounts_iter)?;
        let mint_bid_acc = next_account_info(accounts_iter)?;
        let treasury_ask_acc = next_account_info(accounts_iter)?;
        let treasury_sen_acc = next_account_info(accounts_iter)?;
        let treasurer = next_account_info(accounts_iter)?;
        let splt_program = next_account_info(accounts_iter)?;
        let splata_program = next_account_info(accounts_iter)?;
        let swap_program = next_account_info(accounts_iter)?;
        let sysvar_rent_acc = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Initialize destination account just in case
        let dst_acc_data = Account::unpack_unchecked(&dst_acc.data.borrow())?;
        if dst_acc_data.state == AccountState::Uninitialized {
          XSPLATA::initialize_account(
            payer,
            dst_acc,
            payer,
            mint_bid_acc,
            system_program,
            splt_program,
            sysvar_rent_acc,
            splata_program,
            &[],
          )?;
        }
        // Swap
        XSwap::swap(
          amount,
          limit,
          payer,
          pool_acc,
          vault_acc,
          src_acc,
          treasury_bid_acc,
          dst_acc,
          treasury_ask_acc,
          treasury_sen_acc,
          treasurer,
          splt_program,
          swap_program,
          &[],
        )?;

        Ok(())
      }

      AppInstruction::Route {
        amount,
        first_limit,
        second_limit,
      } => {
        msg!("Calling Route function");
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;

        let first_pool_acc = next_account_info(accounts_iter)?;
        let first_vault_acc = next_account_info(accounts_iter)?;
        let src_acc = next_account_info(accounts_iter)?;
        let mint_bid_acc = next_account_info(accounts_iter)?;
        let treasury_bid_acc = next_account_info(accounts_iter)?;
        let first_treasury_sen_acc = next_account_info(accounts_iter)?;
        let first_treasurer = next_account_info(accounts_iter)?;

        let second_pool_acc = next_account_info(accounts_iter)?;
        let second_vault_acc = next_account_info(accounts_iter)?;
        let dst_acc = next_account_info(accounts_iter)?;
        let mint_ask_acc = next_account_info(accounts_iter)?;
        let treasury_ask_acc = next_account_info(accounts_iter)?;
        let second_treasury_sen_acc = next_account_info(accounts_iter)?;
        let second_treasurer = next_account_info(accounts_iter)?;

        let sen_acc = next_account_info(accounts_iter)?;
        let mint_sen_acc = next_account_info(accounts_iter)?;
        let splt_program = next_account_info(accounts_iter)?;
        let splata_program = next_account_info(accounts_iter)?;
        let swap_program = next_account_info(accounts_iter)?;
        let sysvar_rent_acc = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        let first_pool_data = Pool::unpack(&first_pool_acc.data.borrow())?;
        let second_pool_data = Pool::unpack(&first_pool_acc.data.borrow())?;
        if first_pool_data.mint_s != second_pool_data.mint_s {
          return Err(AppError::UnmatchedPrimaryMints.into());
        }
        // Initialize middle account just in case (usually being SEN)
        let sen_acc_data = Account::unpack_unchecked(&sen_acc.data.borrow())?;
        if sen_acc_data.state == AccountState::Uninitialized {
          XSPLATA::initialize_account(
            payer,
            sen_acc,
            payer,
            mint_sen_acc,
            system_program,
            splt_program,
            sysvar_rent_acc,
            splata_program,
            &[],
          )?;
        }
        // Initialize end account just in case
        let dst_acc_data = Account::unpack_unchecked(&dst_acc.data.borrow())?;
        if dst_acc_data.state == AccountState::Uninitialized {
          XSPLATA::initialize_account(
            payer,
            dst_acc,
            payer,
            mint_ask_acc,
            system_program,
            splt_program,
            sysvar_rent_acc,
            splata_program,
            &[],
          )?;
        }
        // Estimate middle amount
        let middle_amount = Self::compute_return_in_swapping(
          amount,
          Self::parse_reserve(&first_pool_data, *mint_bid_acc.key)
            .ok_or(AppError::CannotFindReserves)?,
          first_pool_data.reserve_s,
          true,
        )
        .ok_or(AppError::Overflow)?;
        // Routing #1
        XSwap::swap(
          amount,
          first_limit,
          payer,
          first_pool_acc,
          first_vault_acc,
          src_acc,
          treasury_bid_acc,
          sen_acc,
          first_treasury_sen_acc,
          first_treasury_sen_acc,
          first_treasurer,
          splt_program,
          swap_program,
          &[],
        )?;
        // Routing #2
        XSwap::swap(
          middle_amount,
          second_limit,
          payer,
          second_pool_acc,
          second_vault_acc,
          sen_acc,
          second_treasury_sen_acc,
          dst_acc,
          treasury_ask_acc,
          second_treasury_sen_acc,
          second_treasurer,
          splt_program,
          swap_program,
          &[],
        )?;

        Ok(())
      }
    }
  }

  pub fn parse_reserve(pool_data: &Pool, mint: Pubkey) -> Option<u64> {
    if pool_data.mint_a == mint {
      return Some(pool_data.reserve_a);
    } else if pool_data.mint_b == mint {
      return Some(pool_data.reserve_b);
    } else if pool_data.mint_s == mint {
      return Some(pool_data.reserve_s);
    } else {
      return None;
    }
  }

  pub fn compute_return_in_swapping(
    bid_delta: u64,
    bid_reserve: u64,
    ask_reserve: u64,
    is_discounted: bool,
  ) -> Option<u64> {
    let decimals: u128 = 1000000000;
    let fee: u128 = 2500000;
    let earn: u128 = 500000;

    let new_bid_reserve = bid_delta.checked_add(bid_reserve)?;
    let new_ask_reserve = (bid_reserve as u128)
      .checked_mul(ask_reserve as u128)?
      .checked_div(new_bid_reserve as u128)?;
    let ask_delta = (ask_reserve as u128).checked_sub(new_ask_reserve)?;
    if is_discounted {
      return Some(
        decimals
          .checked_sub(fee)?
          .checked_mul(ask_delta)?
          .checked_div(decimals)? as u64,
      );
    } else {
      return Some(
        decimals
          .checked_sub(fee)?
          .checked_sub(earn)?
          .checked_mul(ask_delta)?
          .checked_div(decimals)? as u64,
      );
    }
  }
}
