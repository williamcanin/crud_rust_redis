use crate::database::Database;
// use crate::options::{DEVELOPMENT_URL, TLS};
use crate::users::models::User;
use redis::RedisResult;

use super::models::ConnectionData;
pub struct Users {
  conn: Database,
}

// TODO: Fazer um módulo apenas para conexão.
impl Users {
  // /// Método para realizar a conexão com Redis.
  // pub fn new(data: ConnectionData) -> RedisResult<Self> {
  //   let uri_schema = match TLS {
  //     true => match data.development {
  //       true => "redis",
  //       false => "rediss",
  //     },
  //     false => "redis",
  //   };
  //   let url = match data.development {
  //     true => DEVELOPMENT_URL.to_string(),
  //     false => format!(
  //       "{}://{}:{}@{}:{}/",
  //       uri_schema, data.username, data.password, data.hostname, data.port
  //     ),
  //   };

  //   let client = redis::Client::open(url)?;
  //   let mut con = client.get_connection()?;

  //   // Autenticação
  //   if !data.development {
  //     let _: () = redis::cmd("AUTH").arg(data.password).query(&mut con)?;
  //   }

  //   Ok(Self { con: Some(con) })
  // }

  // Cria um novo RedisClient usando a conexão fornecida
  pub fn new(data: ConnectionData) -> Result<Self, Box<dyn std::error::Error>> {
    let conn = Database::connection(data)?;
    Ok(Self { conn })
  }

  // Método para gerar ID para cada registro
  fn generate_id(&mut self, key_prefix: &str) -> RedisResult<String> {
    let key = format!("{}:id", key_prefix);
    let con = self.conn.get_connection()?;
    let id: u64 = redis::cmd("INCR").arg(&key).query(con)?;
    Ok(format!("{}:{}", key_prefix, id))
  }

  // Método para adicionar um usuário ao Redis
  pub fn add(&mut self, user: &User) -> RedisResult<String> {
    let user_id = self.generate_id("user")?;
    let _: () = redis::cmd("HMSET")
      .arg(&user_id)
      .arg("name")
      .arg(&user.name)
      .arg("email")
      .arg(&user.email)
      .arg("country")
      .arg(&user.country)
      .query(self.conn.get_connection()?)?;

    Ok(user_id)
  }

  // Método para pegar o usuário no Redis por ID
  pub fn get(&mut self, user_id: &str) -> RedisResult<User> {
    // Garantir que a conexão está ativa
    let con = self.conn.get_connection()?;
    // Recuperar os dados do Redis
    let name: String = redis::cmd("HGET").arg(user_id).arg("name").query(con)?;
    let email: String = redis::cmd("HGET").arg(user_id).arg("email").query(con)?;
    let country: String = redis::cmd("HGET").arg(user_id).arg("country").query(con)?;

    // Retornar o usuário com os dados recuperados
    Ok(User {
      name,
      email,
      country,
    })
  }

  // Método para deletar um valor do Redis
  #[allow(dead_code)]
  pub fn delete(&mut self, key: &str) -> RedisResult<()> {
    let con = self.conn.get_connection()?;
    redis::cmd("DEL").arg(key).query(con)?;
    Ok(())
  }

  // Método para atualizar um valor no Redis
  pub fn update(&mut self, user_id: &str, user: &User) -> RedisResult<()> {
    let con = self.conn.get_connection()?;
    let _: () = redis::cmd("HMSET")
      .arg(user_id)
      .arg("name")
      .arg(&user.name)
      .arg("email")
      .arg(&user.email)
      .arg("country")
      .arg(&user.country)
      .query(con)?;
    Ok(())
  }

  // Fecha a conexão Redis
  pub fn close(&mut self) -> RedisResult<()> {
    self.conn.close()?;
    Ok(())
  }
}
