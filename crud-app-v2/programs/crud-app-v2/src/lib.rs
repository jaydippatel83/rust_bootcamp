use anchor_lang::prelude::*;

declare_id!("5fAUYdYqQ24ckyvypccCWSjYXRrg4toRoSKQ6MHMA1wk");

#[program]
pub mod crud_app_v2 {
    use super::*; 
}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState{
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(500)]
    pub message: String,
}