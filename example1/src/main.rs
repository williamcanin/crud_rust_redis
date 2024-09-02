//
// Recomenta-se usar o https://upstash.com para servidor remoto com Redis.
//
mod database;
mod options;
mod tests;
mod users;

use crate::users::controller::Users;
use crate::users::models::User;
use options::connection_data;
use users::errors::handle_error;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  // ------ Criando um novo cliente Redis para Users ------
  let mut redis_client = match Users::new(connection_data()) {
    Ok(client) => client,
    Err(e) => {
      eprintln!("Error connecting to Redis: {}", e);
      std::process::exit(1);
    }
  };

  // ------ Criando um novo usuário ------
  let new_user = User {
    name: "William".to_string(),
    email: "william@example.com".to_string(),
    country: "Brasil".to_string(),
    site: "https://williamcanin.dev".to_string(),
    graduation: true,
  };

  // ------ Adicionando um valor ------
  #[allow(unused_variables)]
  let mut user_id = String::new();
  #[allow(unused_assignments)]
  match redis_client.add(&new_user) {
    Ok(ret) => {
      let id = ret.split_once(":").unwrap_or(("", "")).1;
      println!("Usuário adicionado com ID: {}", id);
      user_id = ret;
    }
    Err(e) => handle_error(Box::new(e)),
  };

  // ------ Lendo o valor ------
  match redis_client.get(&user_id) {
    Ok(user) => println!("Usuário recuperado: {:?}", user),
    Err(e) => handle_error(Box::new(e)),
  };

  // ------ Atualizando o valor ------
  let updated_user = User {
    name: "William Canin".to_string(),
    email: "william.canin@example.com".to_string(),
    country: "United States".to_string(),
    site: "https://williamcanin.github.io".to_string(),
    graduation: true,
  };
  match redis_client.update(&user_id, &updated_user) {
    Ok(_) => println!("Usuário atualizado!"),
    Err(e) => handle_error(Box::new(e)),
  };

  match redis_client.get(&user_id) {
    Ok(user) => println!("Usuário atualizado recuperado: {:?}", user),
    Err(e) => handle_error(Box::new(e)),
  };

  // // ------ Deletando o valor ------
  // match redis_client.delete(&user_id) {
  //   Ok(_) => println!("Usuário ID {} deletado!", &user_id),
  //   Err(e) => handle_error(Box::new(e)),
  // };

  // ------ Fecha a conexão no final ------
  redis_client.close()?;

  Ok(())
}
