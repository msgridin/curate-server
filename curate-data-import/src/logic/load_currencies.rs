use std::error::Error;
use crate::DBPool;
use crate::data;

pub(crate) async fn invoke(currencies_path: &str, crypto_path: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {

    println!("LOADING CURRENCIES");

    let currencies = data::fs::read_currencies(currencies_path)?;
    println!("{} CURRENCIES", currencies.len());
    for currency in currencies {
        data::db::create_currency(currency, db_pool).await?;
    }

    let currencies = data::fs::read_currencies(crypto_path)?;
    println!("{} CRYPTO", currencies.len());
    for currency in currencies {
        data::db::create_currency(currency, db_pool).await?;
    }

    Ok(())
}
