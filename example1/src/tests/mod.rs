#[cfg(test)]
mod database {
  use crate::{database::controller::Database, options::connection_data};
  use std::env;

  #[tokio::test]
  async fn connection() {
    match Database::connection(connection_data()) {
      Ok(client) => {
        let host = env::var("REDIS_HOSTNAME").unwrap();
        println!("{}", host);
        client
      }
      Err(e) => {
        eprintln!("Error creating Redis client: {}", e);
        panic!("Failed to create Redis client");
      }
    };
  }
}
