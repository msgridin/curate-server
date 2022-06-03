CREATE TABLE IF NOT EXISTS currencies
(
    id VARCHAR(7) PRIMARY KEY NOT NULL,
    name VARCHAR(50),
    country_id VARCHAR(2),
    country_name VARCHAR(50),
    is_crypto BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS rates
(
    currency VARCHAR(7) NOT NULL,
    foreign_currency VARCHAR(3) NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    exchange_date TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY(currency, foreign_currency, exchange_date)
);

CREATE TABLE IF NOT EXISTS rate_subscriptions
(
    from_currency VARCHAR(7) NOT NULL,
    to_currency VARCHAR(7) NOT NULL,
    firebase_token VARCHAR(255) NOT NULL,
    PRIMARY KEY(from_currency, to_currency, firebase_token)
);

