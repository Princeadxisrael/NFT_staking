use anchor_lang::prelude::*;

// create a pda for user_account
#[account]
pub struct UserAccount{
   pub points:u32,
   pub amount_staked:u8,
    pub bump: u8,
}

impl Space for UserAccount {
    const INIT_SPACE:usize= 8 + 4 +1 +1;   
}