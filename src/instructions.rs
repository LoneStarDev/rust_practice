use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::EscrowError::InvalidInstruction;

pub enum MetaTicket_EscrowInstructions {
    ///  Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Account expected:
    ///
    /// 0. '[signer] The account of the MetaTicket user that will initialize the escrow.
    /// 1. '[writable] Temporary token account of the MetaTicket user, which will store USDC or SOL for the transaaction.
    /// 2. '[read only] the MetaTicket user's token account for the MetaTicket they will receive if the trade goas through.
    /// 3. '[writable]' the escrow account that will hold all necessary information about the trade.
    /// 4. '[read only]' The rent sysvar
    /// 5. '[read only]' The Token Program.
    ///
    /// API endpoint
    InitEscrow {
        /// How many MetaTickets (Token Y) to be expect by MetaTicket user.
        amount: u64,
    },
}

impl MetaTicket_EscrowInstructions {
    ///unpack first byte with a tag, then use match to determine how to decode the rest of the bytes as a slice
    pub fn unpack_instructions(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    pub fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
