use crate::database::Database;
use crate::users::models::User;
use redis::RedisResult;
use std::collections::HashMap;

use super::models::ConnectionData;
pub struct Users {
  conn: Database,
}

impl Users {
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

    // Converte o campo booleano graduation para string
    let graduation = user.graduation.to_string();

    // Cria um vetor de tuplas, preservando a ordem dos campos do struct User
    let user_fields = vec![
      ("name", &user.name),
      ("email", &user.email),
      ("country", &user.country),
      ("site", &user.site),
      ("graduation", &graduation),
    ];

    // Usa redis::cmd para executar o HMSET com os campos na ordem correta
    let _: () = redis::cmd("HMSET")
      .arg(&user_id)
      .arg(user_fields)
      .query(self.conn.get_connection()?)?;

    Ok(user_id)
  }

  // Método para pegar o usuário no Redis por ID
  pub fn get(&mut self, user_id: &str) -> RedisResult<HashMap<String, String>> {
    use std::collections::HashMap;
    // Garantir que a conexão está ativa
    let con = self.conn.get_connection()?;

    // Definir os campos esperados e garantir que estão presentes
    let fields = vec!["name", "email", "country", "graduation"];

    let mut values: HashMap<String, String> = HashMap::new();

    for field in &fields {
      let value: String = redis::cmd("HGET").arg(user_id).arg(field).query(con)?;
      values.insert(field.to_string(), value);
    }

    // Retornar o usuário com os dados recuperados
    Ok(values)
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

    // Converte o campo booleano graduation para string
    let graduation = user.graduation.to_string();

    // Cria um vetor de tuplas, preservando a ordem dos campos do struct User
    let user_fields = vec![
      ("name", &user.name),
      ("email", &user.email),
      ("country", &user.country),
      ("site", &user.site),
      ("graduation", &graduation),
    ];

    // Passando todos os campos para o comando HMSET
    let mut cmd = redis::cmd("HMSET");
    cmd.arg(user_id);

    for (field, value) in user_fields {
      cmd.arg(field).arg(value);
    }

    cmd.query(con)?;

    Ok(())
  }

  // Fecha a conexão Redis
  pub fn close(&mut self) -> RedisResult<()> {
    self.conn.close()?;
    Ok(())
  }
}
