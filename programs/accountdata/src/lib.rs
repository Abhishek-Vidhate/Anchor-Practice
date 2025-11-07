use anchor_lang::prelude::*;

declare_id!("5jrMyoYU7J7nFETNWQxfNNN8W12Fw7pEzXtS3YVeTu6f");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod accountdata {
    use super::*;
    pub fn create_address_info(
        ctx: Context<CreateAddressInfo>,
        name: String,
        house_number: u8,
        street: String,
        city: String,
    ) -> Result<()> {
        *ctx.accounts.address_info = AddressInfo {
            name,
            house_number,
            street,
            city,
        };
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAddressInfo<'info> {
    #[account(mut)]
    pub payer_or_user: Signer<'info>,

    // AddressInfo account where data ( data's state to be precise ) will be stored on-chain
    #[account(
        init,
        payer = payer_or_user,
        space = ANCHOR_DISCRIMINATOR_SIZE + AddressInfo::INIT_SPACE, //to allow fixed space for rent management
    )]
    pub address_info: Account<'info, AddressInfo>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct AddressInfo {
    #[max_len(50)]
    pub name: String, // 4 byte + 50 byte is space allocated max
    pub house_number: u8, // 1 byte is allocated as no max_len is mentioned

    #[max_len(50)]
    pub street: String, // 4 byte + 50 byte
    #[max_len(50)]
    pub city: String,
}
