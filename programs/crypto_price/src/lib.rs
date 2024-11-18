use anchor_lang::prelude::*;

declare_id!("31111111111111111111111111111111111111111111"); // 临时 Program ID，占位

#[program]
pub mod crypto_price {
    use super::*;

    pub fn add_numbers(_ctx: Context<AddNumbers>, left: u64, right: u64) -> Result<u64> {
        Ok(left + right)
    }
}

#[derive(Accounts)]
pub struct AddNumbers {}
