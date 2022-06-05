use std::error::Error;
use crate::data::db::TABLE_RATES;
use crate::data::models::Rate;
use crate::DBPool;

pub async fn read_current_rate(currency: &str, foreign_currency: &str, db_pool: &DBPool) -> Result<f64, Box<dyn Error>> {
    let rate: Vec<Rate> = sqlx::query_as::<_, Rate>(
        format!("
select
    r.currency,
    r.foreign_currency,
    r.exchange_date,
    r.rate
from
    {} r
where
    r.currency = $1
    and r.foreign_currency = $2
order by
    r.exchange_date desc
limit 1", TABLE_RATES).as_str()
    )
        .bind(currency)
        .bind(foreign_currency)
        .fetch_all(db_pool).await?;

    match rate.len() {
        0 => Ok(0.0),
        _ => Ok(rate[0].rate)
    }
}
