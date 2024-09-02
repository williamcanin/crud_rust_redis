use crate::{options::connection_data, users::controller::Users};

#[allow(dead_code)]
fn client_users() -> Users {
  // ------ Criando um novo cliente Redis para Users ------
  match Users::new(connection_data()) {
    Ok(client) => client,
    Err(e) => {
      panic!("Error connecting to Redis: {}", e);
    }
  }
}

#[test]
fn create() {
  let mut client = client_users();
  // ------ Criando um novo usuário ------
  let new_user = crate::users::models::User {
    name: "William".to_string(),
    email: "william@example.com".to_string(),
    country: "Brasil".to_string(),
    site: "https://williamcanin.github.io".to_string(),
    graduation: true,
  };

  // ------ Adicionando um valor ------
  match client.add(&new_user) {
    Ok(ret) => {
      let id = ret.split_once(":").unwrap_or(("", "")).1;
      println!("Usuário adicionado com ID: {}", id);
    }
    Err(e) => {
      crate::utils::errors::handle_error(Box::new(e));
      panic!("Erro ao adicionar usuário.");
    }
  };
}

#[test]
fn read() {
  let user_id = "1";
  let mut client = client_users();
  match client.get(&format!("user:{}", user_id)) {
    Ok(user) => {
      println!("Usuário recuperado: {:?}", user);
    }
    Err(e) => {
      crate::utils::errors::handle_error(Box::new(e));
      panic!("Erro ao obter usuário ou inexistente.");
    }
  };
}

#[test]
fn update() {
  let user_id = "1";

  let mut client = client_users();
  // ------ Atualizando o valor ------
  let updated_user = crate::users::models::User {
    name: "William Canin".to_string(),
    email: "william.canin@example.com".to_string(),
    country: "United States".to_string(),
    site: "https://williamcanin.github.io".to_string(),
    graduation: true,
  };
  match client.update(&format!("user:{}", user_id), &updated_user) {
    Ok(_) => {
      println!("Usuário atualizado!");
    }
    Err(e) => {
      crate::utils::errors::handle_error(Box::new(e));
      panic!("Erro ao atualizar usuário.");
    }
  };
}

#[test]
fn delete() {
  let user_id = "1";

  let mut client = client_users();
  // ------ Deletando o valor ------
  match client.delete(&format!("user:{}", user_id)) {
    Ok(_) => {
      println!("Usuário ID {} deletado!", user_id);
    }
    Err(e) => {
      crate::utils::errors::handle_error(Box::new(e));
      panic!("Erro ao deletar usuário.");
    }
  };
}
