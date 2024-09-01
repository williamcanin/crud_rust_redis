use crate::options::{DEVELOPMENT_URL, TLS};
use crate::users::models::ConnectionData;
use redis::{Connection, RedisError, RedisResult};
pub struct Database {
  conn: Option<Connection>,
}

impl Database {
  /// Método para realizar a conexão com Redis.
  pub fn connection(data: ConnectionData) -> RedisResult<Self> {
    let uri_schema = match TLS {
      true => match data.development {
        true => "redis",
        false => "rediss",
      },
      false => "redis",
    };
    let url = match data.development {
      true => DEVELOPMENT_URL.to_string(),
      false => format!(
        "{}://{}:{}@{}:{}/",
        uri_schema, data.username, data.password, data.hostname, data.port
      ),
    };

    let client = redis::Client::open(url)?;
    let mut con = client.get_connection()?;

    // Autenticação
    if !data.development {
      let _: () = redis::cmd("AUTH").arg(data.password).query(&mut con)?;
    }

    Ok(Self { conn: Some(con) })
  }

  // Obtém a conexão atual
  pub fn get_connection(&mut self) -> Result<&mut Connection, RedisError> {
    self
      .conn
      .as_mut()
      .ok_or_else(|| RedisError::from((redis::ErrorKind::IoError, "Connection is closed")))
  }

  // Fecha a conexão
  pub fn close(&mut self) -> RedisResult<()> {
    // Define a conexão como None para indicar que está fechada
    self.conn = None;
    println!("Redis connection closed.");
    Ok(())
  }
}
