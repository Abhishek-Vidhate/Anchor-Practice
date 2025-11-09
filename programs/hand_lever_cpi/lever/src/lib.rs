use anchor_lang::prelude::*;

declare_id!("CYuSu4niMAy8vtiC48u2Wmy2F72ddkD9tbJvfD6MLroz");

#[program]
pub mod lever {
    use super::*;

    pub fn initialize(ctx: Context<InitializeLever>) -> Result<()> {
        msg!(
            "Lever Program initialized, programId: {:#?}",
            ctx.program_id
        );
        Ok(())
    }

    pub fn switch_power(ctx: Context<SetPowerStatus>, name: String) -> Result<()> {
        let power = &mut ctx.accounts.power;

        power.is_on = !power.is_on; // main level logic , to switch values to opposite as level is called or pulled

        msg!("{} is pulling level", &name);

        match power.is_on {
            true => msg!("Power is ON"),
            false => msg!("Power is OFF"),
        };

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLever<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
    init,
    space = 8+8,
    payer = user,
)]
    pub power: Account<'info, PowerStatus>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPowerStatus<'info> {
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
}
#[account]
pub struct PowerStatus {
    pub is_on: bool,
}
