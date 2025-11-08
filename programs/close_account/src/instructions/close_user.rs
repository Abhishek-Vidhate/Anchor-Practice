use crate::state::user_state::UserState;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseUserAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        close = user,
        seeds = [b"user", user.key().as_ref()],
        bump,
        
        // here init, space and payer is not used; as account is already created (or init)
        // no need to mention space, as account is already created
        // no payer , as this account was already paid while creation, so here mentioned `close`
        // using close to send the rent to the mentioned account , here we mentioned user account 
    )]
    pub user_data_account: Account<'info, UserState>, 
        // as here we are using check 'close' this will close the account , close is having closing implemention , might be similar to macros 

    pub system_program: Program<'info, System>
}

// invoke this account ( with `close` check), by invoking the close check will be validated (or called) and this account will be closed 
pub fn close_user(_ctx: Context<CloseUserAccount>) -> Result<()> {
    Ok(())
}
