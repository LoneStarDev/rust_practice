use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    sysvar::{self, rent::Rent, Sysvar},
};

use crate::processor::Processor;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Decode and dispatch instructions here.
    Processor::process(program_id, accounts, instruction_data);

    //Rent Determination
    {
        msg!("Rent identifier:");
        sysvar::rent::id().log();
        let rent = Rent::from_account_info(&accounts[7]).unwrap();
        assert_eq!(rent, Rent::default());
        let got_rent = Rent::get()?;
        assert_eq!(rent, got_rent);
    }

    Ok(())
}
