use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Cryptofestival {
    // data
    #[max_len(40)] //  nombre del evento max 40 caracteres
    pub name: String,
    pub ticket_price: u64,
    pub active: bool,
    // accounts
    pub authority: Pubkey,
    pub accepted_mint: Pubkey,
    // bumps
    pub event_bump: u8,
    pub event_mint_bump: u8,
    pub treasury_vault_bump: u8,
    pub gain_vault_bump: u8,
}

impl Cryptofestival {
    // custom seeds
    pub const SEED_EVENT: &'static str = "festival";
    pub const SEED_EVENT_MINT: &'static str = "festival_mint";
    pub const SEED_TREASURY_VAULT: &'static str = "treasury_vault";
    pub const SEED_GAIN_VAULT: &'static str = "gain_vault";
}