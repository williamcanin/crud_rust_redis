use crate::{
  database::{controller::Database, models::ConnectionData},
  users::models::User,
};
use redis::RedisResult;
use std::collections::HashMap;

pub struct Users {
  conn: Database,
}

fn fields(user: &User) -> Vec<(&str, String)> {
  // Cria um vetor de tuplas, preservando a ordem dos campos do struct User
  vec![
    ("name", user.name.to_string()),
    ("email", user.email.to_string()),
    ("country", user.country.to_string()),
    ("site", user.site.to_string()),
    ("graduation", user.graduation.to_string()),
  ]
}

impl Users {
  // Cria um novo RedisClient usando a conexão fornecida
  pub fn new(data: ConnectionData) -> Result<Self, Box<dyn std::error::Error>> {
    // Realiza a conexão com o banco de dados
    let conn = Database::connection(data)?;
    // Retorna a conexão
    Ok(Self { conn })
  }

  // Fecha a conexão Redis
  fn close(&mut self) -> RedisResult<()> {
    // Chama o fechamento da conexão
    self.conn.close()?;
    Ok(())
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
    // Gera ID para o usuário adicionado
    let user_id = self.generate_id("user")?;

    // Resgata os campos dos dados
    let user_fields = fields(user);

    // Usa redis::cmd para executar o HMSET com os campos na ordem correta
    let _: () = redis::cmd("HMSET")
      .arg(&user_id)
      .arg(user_fields)
      .query(self.conn.get_connection()?)?;

    // Fecha a conexão
    self.close()?;

    Ok(user_id)
  }

  // Método para pegar o usuário no Redis por ID
  pub fn get(&mut self, user_id: &str) -> RedisResult<HashMap<String, String>> {
    use std::collections::HashMap;
    // Garantir que a conexão está ativa
    let con = self.conn.get_connection()?;

    // Definir os campos esperados e garantir que estão presentes
    let fields = vec!["name", "email", "country", "graduation"];

    // Cria um HashMap<String, String> vazio
    let mut values: HashMap<String, String> = HashMap::new();

    // Percorre os campos e insere no HashMap<String, String>
    for field in &fields {
      let value: String = redis::cmd("HGET").arg(user_id).arg(field).query(con)?;
      values.insert(field.to_string(), value);
    }

    // Fecha a conexão
    self.close()?;

    // Retornar o usuário com os dados recuperados
    Ok(values)
  }

  // Método para deletar um valor do Redis
  #[allow(dead_code)]
  pub fn delete(&mut self, key: &str) -> RedisResult<()> {
    // Estabelece conexão
    let con = self.conn.get_connection()?;

    // Envia comando ao Redis para deletar
    redis::cmd("DEL").arg(key).query(con)?;

    // Fecha a conexão
    self.close()?;

    Ok(())
  }

  // Método para atualizar um valor no Redis
  pub fn update(&mut self, user_id: &str, user: &User) -> RedisResult<()> {
    // Estabelece conexão
    let con = self.conn.get_connection()?;

    // Resgata os campos dos dados
    let user_fields = fields(user);

    // Passando todos os campos para o comando HMSET
    let mut cmd = redis::cmd("HMSET");
    cmd.arg(user_id);

    // Percorre os campos atribuindo os mesmo para o comando Redis
    for (field, value) in user_fields {
      cmd.arg(field).arg(value);
    }

    cmd.query(con)?;

    // Fecha a conexão
    self.close()?;

    Ok(())
  }
}
