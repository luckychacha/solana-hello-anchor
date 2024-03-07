use anchor_lang::prelude::*;

declare_id!("DJ24usMaxgzhmpNdLJHzMCnZ2kayoYv7gBoMGwbHWqK3");

#[program]
pub mod solana_hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
