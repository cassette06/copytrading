use anchor_lang::prelude::*;

declare_id!("7KUQnjdVyEhoLCC9tNG9rxjQ4GP98TbzvCHytdZ3u1ZD");

#[program]
pub mod copytrading {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
