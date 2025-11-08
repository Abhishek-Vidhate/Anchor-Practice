use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub bump: u8,     //1 byte
    pub user: Pubkey, //32 bytes
    #[max_len(50)]
    pub name: String, // 4 byte + 50 byte
}
