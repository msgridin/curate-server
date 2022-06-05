use std::error::Error;
use crate::data::models::{CurrentRate, RateSubscription};
use crate::DBPool;
use crate::data;
use crate::logic::util::get_multiplier;

pub(crate) mod util;


pub(crate) async fn send_notifications(main_currencies: [&str; 3], server_api_key: &str, db_pool: &DBPool) -> Result<(), Box<dyn Error>> {
    let firebase_tokens: Vec<String> = data::db::rates::read_firebase_tokens(db_pool).await?;

    for token in firebase_tokens {
        let rates: Vec<CurrentRate> = get_subscription_current_rates(main_currencies, token.as_str(), db_pool).await?;
        for rate in rates {
            send_notification(rate, server_api_key, token.as_str()).await?;
        }
    }

    Ok(())
}

async fn get_subscription_current_rates(main_currencies: [&str; 3], firebase_token: &str, db_pool: &DBPool) -> Result<Vec<CurrentRate>, Box<dyn Error>> {
    let subscriptions: Vec<RateSubscription> = data::db::rates::read_rate_subscriptions(firebase_token, db_pool).await?;
    let mut rates: Vec<CurrentRate> = vec![];
    for subscription in subscriptions {
        let current_rate = get_current_rate(main_currencies, subscription, db_pool).await?;
        rates.push(current_rate);
    }

    Ok(rates)
}

async fn get_current_rate(main_currencies: [&str; 3], subscription: RateSubscription, db_pool: &DBPool) -> Result<CurrentRate, Box<dyn Error>> {
    let rate = if main_currencies.contains(&subscription.from_currency.as_str()) {
        let current = data::db::rates::read_current_rate(&subscription.from_currency, &subscription.to_currency, db_pool).await?;
        let multiplier = get_multiplier(current);

        CurrentRate {
            from_currency: subscription.from_currency,
            to_currency: subscription.to_currency,
            rate: current * (multiplier as f64),
            multiplier,
        }
    } else if main_currencies.contains(&subscription.to_currency.as_str()) {
        let current = data::db::rates::read_current_rate(&subscription.to_currency, &subscription.from_currency, db_pool).await?;
        let current = if current == 0.0 {
            0.0
        } else {
            1.0 / current
        };
        let multiplier = get_multiplier(current);

        CurrentRate {
            from_currency: subscription.from_currency,
            to_currency: subscription.to_currency,
            rate: current * (multiplier as f64),
            multiplier,
        }
    } else {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Currency not found")));
    };

    Ok(rate)
}

async fn send_notification(
    rate: CurrentRate,
    server_api_key: &str,
    token: &str,
) -> Result<(), Box<dyn Error>> {
    let client = fcm::Client::new();

    let title = format!("{} {} = {} {:.3}", rate.from_currency, rate.multiplier, rate.to_currency, rate.rate);

    let mut notification_builder = fcm::NotificationBuilder::new();
    notification_builder.title(title.as_str());

    let notification = notification_builder.finalize();
    let mut message_builder = fcm::MessageBuilder::new(server_api_key, token);
    message_builder.notification(notification);

    let _response = client
        .send(message_builder.finalize())
        .await?;

    Ok(())
}
