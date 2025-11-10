use anchor_lang::prelude::*;

declare_id!("EC8xD3gt9i9f7MrHVwZ6bdhY31QC4pKbKpVfrfmBg8oh");

#[program]
pub mod realloc_space {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, input: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.message_account.message = input; // add input string to pub message, this is not stored on chain
        Ok(())
    }

    pub fn update(ctx: Context<Update>, input: String) -> Result<()> {
        ctx.accounts.message_account.message = input;
        // now the old message is updated with this new string , and the size of account is calculated again with our impl message { required_space }, as this string can have smaller or larger size or input_len
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(input: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = Message::required_space(input.len()),

    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(input : String)]
pub struct Update<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        realloc = Message::required_space(input.len()),
        realloc::payer = payer,
        realloc::zero = true,
    )]
    pub message_account: Account<'info, Message>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Message {
    pub message: String,
}

// can also use #[derive(InitSpace)] instead of the below impl function; `InitSpace` automatically calculates for 8 ( for discriminator), 4 (for string) and here no need to mention input_len as "pub message: String" itself is the input_len data struct here.

// or i might be wrong on this take

// i think this impl message is needed and is main logic for changing size of account , as size will change based on input_len

// yes, InitSpace can't be used here, as with init space , once it is calculated during account creation , it is a fixed number, and it will be the this same number which will be use, So the impl required_space method is correct here

// MAIN LOGIC OF REALLOC OF ACCOUNT SPACE PROGRAM
impl Message {
    pub fn required_space(input_len: usize) -> usize {
        8 + // 8 byte for discriminator
        4 + // for string data type 
        input_len
    }
}
