use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// The state.rs file will be responsible for defining state objects the processor can use and also
/// serialize and deserialize such objects from and into arrays of u8 respectively.

// here we create a struct that checks the state of the escrow
// is the trade initialized? what is the initializers pubkey? what is the temporary token accounts
// pubkey so that that account holding the USDC/SOL can send to metaticket's USDC/SOL account or accounts.
// also we check the pubkey of the account created to hold the metaticket NFT
//we also check to see if MetaTicket is sending the correct amount of tickets to the user.

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
    pub temp_token_account_pubkey: Pubkey,
    pub initializer_token_to_receive_acount_pubkey: Pubkey,
    pub expected_amount: u64,
}

// Solana's verson of Rust's Sized trait
impl Sealed for Escrow {}

// check if programs state is initialized
impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
