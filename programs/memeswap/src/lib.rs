use anchor_lang::prelude::*;

declare_id!("DKHwy7ZDKC85w9FRBEnpk11EXSimVxA2NPfFbGSBJ9v");

#[program]
pub mod memeswap {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}