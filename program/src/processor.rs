use crate::error::AppError;
use crate::helper::oracle::Oracle;
use crate::instruction::AppInstruction;
use crate::interfaces::{xsplata::XSPLATA, xswap::XSwap};
use crate::schema::{account::Account, pool::Pool};
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  program_pack::{IsInitialized, Pack},
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
        if !Self::is_rented_and_initialized_acc(&dst_acc)? {
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
        let second_pool_data = Pool::unpack(&second_pool_acc.data.borrow())?;
        if first_pool_data.mint_s != second_pool_data.mint_s {
          return Err(AppError::UnmatchedPrimaryMints.into());
        }
        // Initialize middle account just in case (usually being SEN)
        if !Self::is_rented_and_initialized_acc(&sen_acc)? {
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
        if !Self::is_rented_and_initialized_acc(&dst_acc)? {
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
        let bid_reserve = Self::parse_reserve(&first_pool_data, *mint_bid_acc.key)
          .ok_or(AppError::CannotFindReserves)?;
        let middle_reserve = Self::parse_reserve(&first_pool_data, *mint_sen_acc.key)
          .ok_or(AppError::CannotFindReserves)?;
        let new_bid_reserve = bid_reserve.checked_add(amount).ok_or(AppError::Overflow)?;
        let (new_middle_reserve, _, _) = Oracle::curve_in_fee(
          new_bid_reserve,
          bid_reserve,
          first_pool_data.reserve_s,
          true,
        )
        .ok_or(AppError::Overflow)?;
        let middle_amount = middle_reserve
          .checked_sub(new_middle_reserve)
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

      AppInstruction::AddLiquidity {
        delta_s,
        delta_a,
        delta_b,
      } => {
        msg!("Calling AddLiquidity function");
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;
        let pool_acc = next_account_info(accounts_iter)?;
        let lpt_acc = next_account_info(accounts_iter)?;
        let mint_lpt_acc = next_account_info(accounts_iter)?;
        let src_s_acc = next_account_info(accounts_iter)?;
        let treasury_s_acc = next_account_info(accounts_iter)?;
        let src_a_acc = next_account_info(accounts_iter)?;
        let treasury_a_acc = next_account_info(accounts_iter)?;
        let src_b_acc = next_account_info(accounts_iter)?;
        let treasury_b_acc = next_account_info(accounts_iter)?;
        let treasurer = next_account_info(accounts_iter)?;
        let splt_program = next_account_info(accounts_iter)?;
        let splata_program = next_account_info(accounts_iter)?;
        let swap_program = next_account_info(accounts_iter)?;
        let sysvar_rent_acc = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Initialize destination account just in case
        if !Self::is_rented_and_initialized_acc(&lpt_acc)? {
          XSPLATA::initialize_account(
            payer,
            lpt_acc,
            payer,
            mint_lpt_acc,
            system_program,
            splt_program,
            sysvar_rent_acc,
            splata_program,
            &[],
          )?;
        }
        // Add Liquidity
        XSwap::add_liquidity(
          delta_s,
          delta_a,
          delta_b,
          payer,
          pool_acc,
          lpt_acc,
          mint_lpt_acc,
          src_s_acc,
          treasury_s_acc,
          src_a_acc,
          treasury_a_acc,
          src_b_acc,
          treasury_b_acc,
          treasurer,
          splt_program,
          swap_program,
          &[],
        )?;

        Ok(())
      }

      AppInstruction::RemoveLiquidity { lpt } => {
        msg!("Calling RemoveLiquidity function");
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;
        let pool_acc = next_account_info(accounts_iter)?;
        let lpt_acc = next_account_info(accounts_iter)?;
        let mint_lpt_acc = next_account_info(accounts_iter)?;
        let dst_s_acc = next_account_info(accounts_iter)?;
        let mint_s_acc = next_account_info(accounts_iter)?;
        let treasury_s_acc = next_account_info(accounts_iter)?;
        let dst_a_acc = next_account_info(accounts_iter)?;
        let mint_a_acc = next_account_info(accounts_iter)?;
        let treasury_a_acc = next_account_info(accounts_iter)?;
        let dst_b_acc = next_account_info(accounts_iter)?;
        let mint_b_acc = next_account_info(accounts_iter)?;
        let treasury_b_acc = next_account_info(accounts_iter)?;
        let treasurer = next_account_info(accounts_iter)?;
        let splt_program = next_account_info(accounts_iter)?;
        let splata_program = next_account_info(accounts_iter)?;
        let swap_program = next_account_info(accounts_iter)?;
        let sysvar_rent_acc = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

        // Initialize destination account just in case
        if !Self::is_rented_and_initialized_acc(&dst_s_acc)? {
          XSPLATA::initialize_account(
            payer,
            dst_s_acc,
            payer,
            mint_s_acc,
            system_program,
            splt_program,
            sysvar_rent_acc,
            splata_program,
            &[],
          )?;
        }
        if !Self::is_rented_and_initialized_acc(&dst_a_acc)? {
          XSPLATA::initialize_account(
            payer,
            dst_a_acc,
            payer,
            mint_a_acc,
            system_program,
            splt_program,
            sysvar_rent_acc,
            splata_program,
            &[],
          )?;
        }
        if !Self::is_rented_and_initialized_acc(&dst_b_acc)? {
          XSPLATA::initialize_account(
            payer,
            dst_b_acc,
            payer,
            mint_b_acc,
            system_program,
            splt_program,
            sysvar_rent_acc,
            splata_program,
            &[],
          )?;
        }
        // Remove Liquidity
        XSwap::remove_liquidity(
          lpt,
          payer,
          pool_acc,
          lpt_acc,
          mint_lpt_acc,
          dst_s_acc,
          treasury_s_acc,
          dst_a_acc,
          treasury_a_acc,
          dst_b_acc,
          treasury_b_acc,
          treasurer,
          splt_program,
          swap_program,
          &[],
        )?;

        Ok(())
      }
    }
  }

  pub fn is_rented_and_initialized_acc(acc: &AccountInfo) -> Result<bool, ProgramError> {
    let is_initialized: bool;
    if (&acc.data.borrow()).len() == 0 {
      is_initialized = false
    } else {
      let acc_data = Account::unpack_unchecked(&acc.data.borrow())?;
      is_initialized = acc_data.is_initialized();
    }
    Ok(is_initialized)
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
}
