use super::models::ConnectionData;
use redis::{Connection, RedisError, RedisResult};

pub struct Database {
  conn: Option<Connection>,
}

impl Database {
  /// Método para realizar a conexão com Redis.
  pub fn connection(data: ConnectionData) -> RedisResult<Self> {
    let url = match data.production {
      true => data.production_url,
      false => data.development_url,
    };

    let client = redis::Client::open(url)?;
    let con = client.get_connection()?;

    // // Autenticação::DEPRECATED
    // if data.production {
    //   use crate::utils::catch::url_password;
    //   let password = url_password(&data.production_url);
    //   let _: () = redis::cmd("AUTH").arg(password).query(&mut con)?;
    // }

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
