use anchor_lang::prelude::*;

declare_id!("GigwQcJY9cZaxfT5J295psuwzzBuADivAYYBpUes6Fwd");

#[program]
pub mod check_account {
    use super::*;

    pub fn check_accounts(_ctx: Context<CheckAccount>) -> Result<()> {
        Ok(())
    }
}

// In anchor account validation is done using types & constraints specified in #[derive(Accounts)] struct , this Account (CheckAccount) don't include all available constraints
#[derive(Accounts)]
pub struct CheckAccount<'info> {
    payer: Signer<'info>, //checks if account is Signer

    // CHECK : no check performed, eg of unchecked account
    #[account(mut)]
    account_to_create: UncheckedAccount<'info>,

    // CHECK : perform owner check using constraint
    #[account(
        mut,
        owner = id()
    )]
    account_to_change: UncheckedAccount<'info>,

    system_program: Program<'info, System>, // the `System` checks if this accont is sys program
                                            // i.e this account check i.e system_program checks if account 'CheckAccount' is executable and is it a system program
}
