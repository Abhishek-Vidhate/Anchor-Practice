use anchor_lang::prelude::*;
use anchor_spl::metadata::{
    create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
    Metadata,
};
use anchor_spl::token::{Mint, Token};

declare_id!("678ZcdUGNPqdwjDNLCGTSSEe9wKMWFDjgew3KL2Qic6i");

#[program]

pub mod create_mint {

    use super::*;

    pub fn create_token_mint(
        ctx: Context<CreateTokenMint>,
        token_name: String,
        _token_decimal: u8,
        token_symbol: String,
        token_uri: String,
    ) -> Result<()> {
        msg!("Hello from Aves with program_id: {}", ctx.program_id.key());
        msg!("creating metadata account for token: {}", &token_name);
        msg!(
            "metadata account address: {}",
            &ctx.accounts.metadata_account.key()
        );

        // CPI , as to create token , this program or program's instruction have to invoke instruction from `token program`

        // invoking the create_metadata_account_v3 instruction on the `token_metadata_program`
        create_metadata_accounts_v3(
            // field no.1 of create_metadata_accounts_v3 i.e CTX
            CpiContext::new(
                // regular fields of CpiContext method
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint_account.to_account_info(),
                    mint_authority: ctx.accounts.payer.to_account_info(),
                    update_authority: ctx.accounts.payer.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
            ),
            // field no.2 of create_metadata_accounts_v3
            DataV2 {
                name: token_name,
                symbol: token_symbol,
                uri: token_uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            false, // no.4 ; is mutable or not
            true,  // no.5 is the update_authority field (check above) the signer for this creation
            None,  // no.6 ; collection detail
        )?;

        msg!("Token Mint created successfully",);
        msg!("Mint address: {}", ctx.accounts.mint_account.key());
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_token_decimals:u8)]
pub struct CreateTokenMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // CHECK : validate account by deriving PDA
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(), mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    // create new mint account
    #[account(
        init,
        payer = payer,
        mint::decimals = _token_decimals,
        mint::authority = payer.key(),
    )]
    pub mint_account: Account<'info, Mint>,

    // now provide all required program account or later calling/invoking
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub token_program: Program<'info, Token>,

    // read more on this account , what is this ? is this related to InitSpace for rent calculation
    pub rent: Sysvar<'info, Rent>,
}
