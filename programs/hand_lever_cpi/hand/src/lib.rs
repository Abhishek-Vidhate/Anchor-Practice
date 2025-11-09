use anchor_lang::prelude::*;

declare_id!("TeWrUdRCukHVF6ZsTKbrKXATNmMS6KpCxUhyBa9wVy6");

declare_program!(lever);
use lever::accounts::PowerStatus;
use lever::cpi::accounts::SwitchPower;
use lever::cpi::switch_power;

use crate::lever::program::Lever;

#[program]
pub mod hand {
    use super::*;

    pub fn pull_lever(ctx: Context<PullLever>, name: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        // create CPI instance to call/invoke 'switch_power' instruction from lever program
        let cpi_ctx = CpiContext::new(
            ctx.accounts.lever_power.to_account_info(),
            SwitchPower {
                power: ctx.accounts.power.to_account_info(),
            },
        );
        switch_power(cpi_ctx, name)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullLever<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
    pub lever_power: Program<'info, Lever>,
}
