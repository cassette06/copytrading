use anchor_lang::prelude::*;

declare_id!("21111111111111111111111111111111111111111111"); // 临时 Program ID，占位

#[program]
pub mod order_management {
    use super::*;

    pub fn create_order(ctx: Context<CreateOrder>, order_id: u64, amount: u64) -> Result<()> {
        let order = &mut ctx.accounts.order;
        order.order_id = order_id;
        order.amount = amount;
        order.status = String::from("Pending");
        order.owner = *ctx.accounts.user.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(init, payer = user, space = 8 + 64 + 64 + 32 + 32)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Order {
    pub order_id: u64,
    pub amount: u64,
    pub status: String,
    pub owner: Pubkey,
}
