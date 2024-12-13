use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::instruction::Instruction;
use solana_program::program::invoke_signed;

declare_id!("DKHwy7ZDKC85w9FRBEnpk11EXSimVxA2NPfFbGSBJ9v");

#[program]
pub mod memeswap {
    use super::*;

    pub fn execute(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
        Swap::execute(ctx, args)
    }
}

pub const SEED_WALLET: &[u8] = b"wallet";

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
        seeds = [SEED_WALLET, &u64::to_le_bytes(0)],
        bump
    )]
    /// CHECK:
    pub contract_wallet: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_WALLET, &u64::to_le_bytes(args.user_id)],
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
        msg!("parameter user id: {}", args.user_id);
        msg!("parameter buy: {}", args.buy);
        msg!("parameter amount: {}", args.amount);
        let args_data = to_hex_string(&args.data);
        msg!("parameter data: {}", args_data);

        // user wallet address
        let seed_user_id = u64::to_le_bytes(args.user_id);
        let user_wallet_seeds = &[SEED_WALLET, &seed_user_id];
        let (_user_wallet_address, _) =
            Pubkey::find_program_address(user_wallet_seeds, ctx.program_id);
        // smart contract wallet address
        let seed_smart_wallet = u64::to_le_bytes(0);
        let smart_wallet_seeds = &[SEED_WALLET, &seed_smart_wallet];
        let (_smart_wallet_address, _) =
            Pubkey::find_program_address(smart_wallet_seeds, ctx.program_id);

        if args.buy == 1 {
            // transfer usdt to user wallet
            let cpi_accounts = Transfer {
                from: ctx.accounts.from.to_account_info().clone(),
                to: ctx.accounts.to.to_account_info().clone(),
                authority: ctx.accounts.contract_wallet.clone(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            token::transfer(
                CpiContext::new_with_signer(cpi_program, cpi_accounts, &[smart_wallet_seeds]),
                args.amount,
            )?;
        }

        // execute
        let instruction_accounts: Vec<AccountMeta> = ctx
            .remaining_accounts
            .iter()
            .map(|acc| AccountMeta {
                pubkey: *acc.key,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            })
            .collect();
        let account_infos: Vec<AccountInfo> = ctx
            .remaining_accounts
            .iter()
            .map(|acc| AccountInfo { ..acc.clone() })
            .collect();

        let instruction = Instruction {
            program_id: *ctx.accounts.jupiter_program.key,
            accounts: instruction_accounts,
            data: args.data,
        };
        invoke_signed(
            &instruction,
            &account_infos[..],
            &[user_wallet_seeds, smart_wallet_seeds],
        )?;

        if args.buy == 0 {
            // transfer usdt to smart wallet
            let cpi_accounts = Transfer {
                from: ctx.accounts.from.to_account_info().clone(),
                to: ctx.accounts.to.to_account_info().clone(),
                authority: ctx.accounts.user_wallet.clone(),
            };
            let cpi_program = ctx.accounts.token_program.to_account_info();
            // refresh use wallet & get usdt balance
            ctx.accounts.from.reload()?;
            let balance = ctx.accounts.from.amount;
            token::transfer(
                CpiContext::new_with_signer(cpi_program, cpi_accounts, &[user_wallet_seeds]),
                balance,
            )?;
        }

        Ok(())
    }
}

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join("")
}
