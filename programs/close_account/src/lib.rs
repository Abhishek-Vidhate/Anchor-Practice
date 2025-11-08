use anchor_lang::prelude::*;

mod instructions;
mod state;

use instructions::*; // to invoke/call create_user and close_user functions

declare_id!("9yezQFF1w2oNmqrUZwnmGjyss7F5czRUB93ftvqoFbtY");

#[program]
pub mod close_account {
    use super::*;

    pub fn create_user(ctx: Context<CreateUserAccount>, name: String) -> Result<()> {
        create_user::create_user(ctx, name)
    }

    pub fn close_user(ctx: Context<CloseUserAccount>) -> Result<()> {
        close_user::close_user(ctx)
    }
}
