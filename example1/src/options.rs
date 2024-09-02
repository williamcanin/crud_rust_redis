use crate::{database::models::ConnectionData, string_to_bool};
use dotenv::dotenv;
use std::env;

// Dados de conexão remota.
// Nota: Se o "PRODUCTION=false" no arquivo .env, o TLS será desabilitado automaticamente, ou seja,
// mesmo que "TLS=true" no arquivo .env, ele não irá surgir efeito.
pub fn connection_data() -> ConnectionData {
  // Carrega as variáveis do arquivo .env
  dotenv().ok();
  ConnectionData {
    production: string_to_bool!(env::var("PRODUCTION").unwrap()),
    tls: string_to_bool!(env::var("TLS").unwrap()),
    development_url: env::var("DEVELOPMENT_URL").unwrap(),
    username: env::var("REDIS_USERNAME").unwrap(),
    hostname: env::var("REDIS_HOSTNAME").unwrap(),
    password: env::var("REDIS_PASSWORD").unwrap(),
    port: env::var("REDIS_PORT").unwrap(),
  }
}
