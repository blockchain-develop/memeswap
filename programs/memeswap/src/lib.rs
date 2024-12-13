use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("DKHwy7ZDKC85w9FRBEnpk11EXSimVxA2NPfFbGSBJ9v");

#[program]
pub mod memeswap {
    use super::*;

    pub fn execute(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
        Swap::execute(ctx, args)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwapArgs {
    pub user_id: u64,
    pub buy: u8,
    pub amount: u64,
    pub data: Vec<u8>,
}

#[derive(Accounts)]
#[instruction(args: SwapArgs)]
pub struct Swap<'info> {
    #[account(
        mut,
        seeds = [b"wallet", &u64::to_le_bytes(args.user_id)],
        bump
    )]
    /// CHECK:
    pub user_wallet: AccountInfo<'info>,

    #[account(mut)]
    pub from: Account<'info, TokenAccount>,

    #[account(mut)]
    pub to: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    /// CHECK:
    pub jupiter_program: AccountInfo<'info>,

    /// The creator of the multisig.
    #[account(mut)]
    pub payer: Signer<'info>,
}

impl Swap<'_> {
    fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Creates a multisig.
    #[access_control(ctx.accounts.validate())]
    pub fn execute(ctx: Context<Self>, args: SwapArgs) -> Result<()> {
        msg!("execute");
        //  parameters
        //
        msg!("parameter user id: {}", args.user_id);
        msg!("parameter buy: {}", args.buy);
        msg!("parameter amount: {}", args.amount);
        let args_data = to_hex_string(&args.data);
        msg!("parameter data: {}", args_data);

        Ok(())
    }
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("")
}
