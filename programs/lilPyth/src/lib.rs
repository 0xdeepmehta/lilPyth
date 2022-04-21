use anchor_lang::prelude::*;
use pyth_client::{self, load_price, load_product};

declare_id!("Hgs7P8y4RUJ82H64twuv8vMWcvH3dN4A24o9xYJm2ktS");

#[program]
pub mod lil_pyth {
    use super::*;

    pub fn gib_price(ctx: Context<GibPrice>) -> Result<()> {

        let pyth_product_data = &ctx.accounts.pyth_poduct.try_borrow_data()?;

        let product_account = *load_product(pyth_product_data).unwrap();
        let pyth_price_pubkey = Pubkey::new(&product_account.px_acc.val);

        // Sanity checks for pyth_price account, 
        // Validation on pyth_poduct account is done by pyth_client itslef, wen we invoke load_product(), load_price()
        require!(product_account.px_acc.is_valid(), LilError::InvalidPriceAccount);
        require_keys_neq!(pyth_price_pubkey, ctx.accounts.pyth_price.key(), LilError::InvalidPriceAccount);

        let pyth_price_data = &ctx.accounts.pyth_price.try_borrow_data()?;
        let price_account = *load_price(pyth_price_data).unwrap();

        msg!("price :: {}", price_account.agg.price);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct GibPrice<'info> {
    /// CHECK: 
    pub pyth_price: AccountInfo<'info>,
    /// CHECK: 
    pub pyth_poduct: AccountInfo<'info>,

}

#[error_code]
pub enum LilError {
    #[msg("Invalid Price product account")]
    InvalidPriceAccount,
}