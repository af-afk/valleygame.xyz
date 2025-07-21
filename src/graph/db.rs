use postgres_native_tls::MakeTlsConnector;

use native_tls::TlsConnector;

use deadpool_postgres::{Config, Pool, Runtime};

use tokio_postgres::config::Host;

use crate::consts::ENV_DATABASE_URI;

use std::env;

pub fn create_db() -> Pool {
    let pg_config = env::var(ENV_DATABASE_URI)
        .unwrap()
        .parse::<tokio_postgres::Config>()
        .unwrap();
    let tls_connector = MakeTlsConnector::new(
        TlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    );
    let mut pool_config = Config::new();
    pool_config.host = pg_config.get_hosts().get(0).map(|h| match h {
        Host::Tcp(s) => s.clone(),
        Host::Unix(s) => s.to_string_lossy().to_string(),
    });
    pool_config.port = pg_config.get_ports().get(0).copied();
    pool_config.user = pg_config.get_user().map(|u| u.to_string());
    pool_config.password = pg_config
        .get_password()
        .map(|p| std::str::from_utf8(p).unwrap().to_string());
    pool_config.dbname = pg_config.get_dbname().map(|d| d.to_string());
    pool_config.create_pool(Some(Runtime::Tokio1), tls_connector).unwrap()
}
