use cfg_if::cfg_if;

cfg_if!(
    if #[cfg(feature = "prod")] {
        pub(crate) const ENVIRONMENT: &str = "PROD";
        pub(crate) const SERVER_REST_API_PORT: u16 = 5080;
        pub(crate) const DATABASE_CONNECTION_STRING: &str = "postgres://postgres:!firebase@127.0.0.1:7878/curate_prod";
    } else if #[cfg(feature = "dev")] {

        pub(crate) const ENVIRONMENT: &str = "DEV";
        pub(crate) const SERVER_REST_API_PORT: u16 = 4080;
        pub(crate) const DATABASE_CONNECTION_STRING: &str = "postgres://postgres:!firebase@127.0.0.1:7878/curate_dev";
    }
);
