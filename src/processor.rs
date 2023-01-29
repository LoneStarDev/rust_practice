use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{error::EscrowError, instructions::MetaTicket_EscrowInstructions, state::Escrow};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = MetaTicket_EscrowInstructions::unpack_instructions(instruction_data)?;

        match instruction {
            MetaTicket_EscrowInstructions::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_metaticket_escrow(accounts, amount, program_id)
            }
        }
    }

    fn process_init_metaticket_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        // here we check if the MetaTicket user account is the present to initialize the escrow
        let account_info_iterator = &mut accounts.iter();
        let initializer = next_account_info(account_info_iterator)?;

        if !initializer.is_signer {
            //if initializer is not signer we return false. An error is returned.
            return Err(ProgramError::MissingRequiredSignature);
        }

        //temp token account needs to be writeable, but transaction will automatically fail if its not anyways
        // we don't need to check if this temp tocken account is owned by the token program as we will be asking the
        // token program to transfer its ownership to a PDA or publically dirived address
        let temp_token_account = next_account_info(account_info_iterator)?;

        // in this account it will hold the metaticket NFT, but the owner will be the token program.
        // no changes will be made to this account, but we will save it into the escrow data so that when
        // MetaTicket accepts the trade the escrow program will know where to send the NFT ticket.
        let metaticket_user_ticket_receive_account = next_account_info(account_info_iterator)?;
        if *metaticket_user_ticket_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account = next_account_info(account_info_iterator)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iterator)?)?;

        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(())
    }
}
