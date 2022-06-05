use std::error::Error;
use crate::data::models::{CurrentRate, RateSubscription};
use crate::DBPool;
use crate::data;
use crate::logic::util::get_multiplier;

pub(crate) mod util;


pub(crate) async fn send_notifications(main_currencies: [&str; 3], server_api_key: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {
    let firebase_tokens: Vec<String> = data::db::read_firebase_tokens(db_pool).await?;

    for token in firebase_tokens {
        let rates: Vec<CurrentRate> = get_subscription_current_rates(main_currencies, token.as_str(), db_pool).await?;
    }

    Ok(())
}

async fn get_subscription_current_rates(main_currencies: [&str; 3], firebase_token: &str, db_pool: &DBPool) -> Result<Vec<CurrentRate>, Box<dyn Error>> {

    let subscriptions: Vec<RateSubscription> = data::db::read_rate_subscriptions(firebase_token, db_pool).await?;
    let rates: Vec<CurrentRate> = subscriptions.into_iter().map(|subscription: RateSubscription| {
        let rate = if main_currencies.contains(&subscription.from_currency_id.as_str()) {
            let current = data::db::rates::read_current_rate(&subscription.from_currency_id, &subscription.to_currency_id, db_pool).await?;
            let multiplier = get_multiplier(current);

            CurrentRate {
                from_currency_id: subscription.from_currency_id,
                to_currency_id: subscription.to_currency_id,
                rate: current * (multiplier as f64),
                multiplier,
            }
        } else if main_currencies.contains(&subscription.to_currency_id.as_str()) {
            let current = data::db::rates::read_current_rate(&subscription.to_currency_id, &subscription.from_currency_id, db_pool).await?;
            let current = if current == 0.0 {
                0.0
            } else {
                1.0 / current
            };
            let multiplier = get_multiplier(current);

            CurrentRate {
                from_currency_id: subscription.to_currency_id,
                to_currency_id: subscription.from_currency_id,
                rate: current * (multiplier as f64),
                multiplier,
            }
        } else { 0.0 };

        Ok(rate)
    }).collect::<Result<Vec<CurrentRate>, Box<dyn Error>>>()?;

    Ok(rates)
}

async fn send_notification(
    rates: Vec<CurrentRate>,
    server_api_key: &String,
    token: &String,
) -> Result<(), Box<dyn Error>> {
    let client = fcm::Client::new();

    let mut notification_builder = fcm::NotificationBuilder::new();
    notification_builder.title("title");
    notification_builder.body("description");

    let notification = notification_builder.finalize();
    let mut message_builder = fcm::MessageBuilder::new(server_api_key, token);
    message_builder.notification(notification);

    let _response = client
        .send(message_builder.finalize())
        .await?;

    Ok(())
}
