use anchor_lang::prelude::*;

declare_id!("79bqZ54jqo2vMq1iB7HQtPZcfqawPoWdRRf4AgK96PdK");

#[program]
pub mod favorite {
    use super::*;
    pub fn set_favorite(
        ctx: Context<SetFavorite>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_pubkey = ctx.accounts.user.key();
        msg!(
            "user {} favorites number {}\ncolor {}\nhobbies {:#?}",
            user_pubkey,
            number,
            color,
            hobbies
        );

        ctx.accounts.favorite_account.set_inner(FavoriteStruct {
            number,
            color,
            hobbies,
        });
        Ok(())
    }

    pub fn get_favorite(ctx: Context<SetFavorite>) -> Result<()> {
        let number = &ctx.accounts.favorite_account.number;
        let color = &ctx.accounts.favorite_account.color;
        let hobbies = &ctx.accounts.favorite_account.hobbies;

        msg!("get favorite function i.e retriving and displaying account's data");
        msg!(
            "favorite number: {}, color: {}, hobbies {:#?}",
            number,
            color,
            hobbies
        );
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct FavoriteStruct {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorite<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + FavoriteStruct::INIT_SPACE,
        seeds = [b"favorite", user.key().as_ref()],
        bump
    )]
    pub favorite_account: Account<'info, FavoriteStruct>,

    pub system_program: Program<'info, System>,
}
