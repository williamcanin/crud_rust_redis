use crate::users::models::ConnectionData;
use dotenv::dotenv;
use std::env;

// Url de conexão local (localhost).
pub const DEVELOPMENT_URL: &str = "redis://127.0.0.1:6379/";

// Dados de conexão remota.
// pub const CONNECTION_DATA: ConnectionData = ConnectionData {
//   development: false,
//   username: env::var("REDIS_USERNAME").unwrap(),
//   hostname: env::var("REDIS_HOSTNAME").unwrap(),
//   password: env::var("REDIS_PASSWORD").unwrap(),
//   port: env::var("REDIS_PORT").unwrap(),
// };

// Define se a conexão será em modo TLS ou não.
// Nota: Se o "development: true" O TLS será desabilitado automaticamente, ou seja,
// mesmo que "TLS = true", não irá surgir efeito.
pub const TLS: bool = true;

pub fn connection_data() -> ConnectionData {
  // Carrega as variáveis do arquivo .env
  dotenv().ok();
  // Set as variáveis do arquivo .env
  ConnectionData {
    development: false,
    username: env::var("REDIS_USERNAME").unwrap(),
    hostname: env::var("REDIS_HOSTNAME").unwrap(),
    password: env::var("REDIS_PASSWORD").unwrap(),
    port: env::var("REDIS_PORT").unwrap(),
  }
}
