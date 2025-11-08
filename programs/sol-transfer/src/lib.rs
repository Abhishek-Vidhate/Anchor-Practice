use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("8fk5B1hKBADS9mF5BBcGRuuY4Zzw9GGAkpVTRdTnSnT7");

//in this example we are transfering to system owned account ( can be a system owned acc , system program id , etc )

#[program]
pub mod sol_transfer {
    use super::*;

    pub fn transfer_cpi(ctx: Context<TransferWithCPI>, amount: u64) -> Result<()> {
        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.recipient.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

    pub fn transfer_program_id(ctx: Context<TransferWithProgramId>, amount: u64) -> Result<()> {
        **ctx.accounts.payer.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.payer.try_borrow_mut_lamports()? += amount;
        Ok(())

        // doubt dereference to point to the stored location and allow mutation of value.
    }
}

//SOL transfers via CPI from a user-specified payer (signer wallet address) to a user-specified recipient (system-owned account), invoking the System Program for validation.
#[derive(Accounts)]
pub struct TransferWithCPI<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>, // system owned account

    system_program: Program<'info, System>, // to provide reference, to enable option to CPI
}

// SOL transfers directly by modifying lamports from a program-owned payer account (e.g., a PDA tied to the program's ID, not a random wallet) to a user-specified recipient (system-owned account), without CPI.
#[derive(Accounts)]
pub struct TransferWithProgramId<'info> {
    // validation or check to verify if this account "TransferWithProgramId" is owned by ProgramID
    #[account(
        mut,
        owner = id() // the value in declare_id
    )]
    pub payer: UncheckedAccount<'info>,

    #[account(mut)]
    pub recipient: SystemAccount<'info>, // system owned account
}
