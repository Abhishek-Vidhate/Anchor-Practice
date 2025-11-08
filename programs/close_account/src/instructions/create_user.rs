use crate::state::UserState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserState::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserState>,

    pub system_program: Program<'info, System>, // this account `system_program` checks (or validates) if the account 'CreateUser' is executable and owned by system_progrm
}

pub fn create_user(ctx: Context<CreateUserAccount>, name: String) -> Result<()> {
    *ctx.accounts.user_account = UserState {
        bump: ctx.bumps.user_account,
        user: ctx.accounts.user.key(),
        name,
    };
    Ok(())
}
