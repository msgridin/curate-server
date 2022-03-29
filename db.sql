CREATE TABLE IF NOT EXISTS currencies
(
    id VARCHAR(3) PRIMARY KEY NOT NULL,
    name VARCHAR(50),
    country_id VARCHAR(2),
    country_name VARCHAR(50)
);

CREATE TABLE IF NOT EXISTS rates
(
    currency VARCHAR(3) NOT NULL,
    foreign_currency VARCHAR(3) NOT NULL,
    rate DOUBLE PRECISION NOT NULL,
    exchange_date TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY(currency, foreign_currency, exchange_date)
);

