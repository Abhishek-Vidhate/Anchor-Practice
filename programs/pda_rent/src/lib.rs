use anchor_lang::prelude::*;
use anchor_lang::system_program::{create_account, CreateAccount};
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("3mcxkRPpoLRYJ9cN7v2QsTa7ecd9678MFhcEaZbKaRza");

#[program]
pub mod pda_rent {
    use super::*;

    // When lamports are transferred to a new address (without and existing account),
    // An account owned by the system program is created by default
    pub fn init_rent_vault(ctx: Context<InitRentVault>, fund_lamport: u64) -> Result<()> {
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer.to_account_info(),
                    to: ctx.accounts.rent_vault.to_account_info(),
                },
            ),
            fund_lamport,
        )?;

        Ok(())
    }

    pub fn create_new_account(ctx: Context<CreateNewAccount>) -> Result<()> {
        // pda signer seeds
        let signer_seeds: &[&[&[u8]]] = &[&[b"rent_vault", &[ctx.bumps.rent_vault]]];

        // mimimum lamports for rent exemption
        let lamports = (Rent::get()?).minimum_balance(0);

        // create the new account, transfering lamports from the rent vault to new account
        create_account(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                CreateAccount {
                    from: ctx.accounts.rent_vault.to_account_info(),
                    to: ctx.accounts.new_account.to_account_info(),
                },
            )
            .with_signer(signer_seeds),
            lamports,
            0,
            &ctx.accounts.system_program.key(),
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitRentVault<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"rent_vault"],
        bump,
    )]
    pub rent_vault: SystemAccount<'info>, // this account will be having address using seed `rent_vault` i.e this account is having PDA.
    // also this account is SystemAccount , so owned by system program (111111....)
    // for this example we are letting rent_vault be owned by system program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateNewAccount<'info> {
    #[account(mut)]
    pub new_account: Signer<'info>,
    #[account(
        mut,
        seeds = [b"rent_vault"],
        bump,
    )]
    pub rent_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
