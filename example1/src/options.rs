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
    development_url: env::var("DEVELOPMENT_URL").unwrap(),
    production_url: env::var("PRODUCTION_URL").unwrap(),
  }
}
