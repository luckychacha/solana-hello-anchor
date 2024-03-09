use anchor_lang::prelude::*;

declare_id!("DJ24usMaxgzhmpNdLJHzMCnZ2kayoYv7gBoMGwbHWqK3");

#[program]
pub mod solana_hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Counter Account Created");
        msg!("Current Count: { }", counter.count);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer_user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub payer_user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Counter {
    pub count: u64,
}

pub fn increment(ctx: Context<Increment>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;
    msg!("Previous Count: { }", counter.count);
    counter.count = counter.count.checked_add(1).unwrap();
    msg!("Counter Incremented");
    msg!("Current Count: { }", counter.count);
    Ok(())
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}