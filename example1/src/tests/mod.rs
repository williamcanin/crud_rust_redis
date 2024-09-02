#[cfg(test)]
mod database {
  use crate::{database::controller::Database, options::connection_data};

  #[tokio::test]
  async fn connection() {
    match Database::connection(connection_data()) {
      Ok(mut client) => {
        // Caso tenha conexão, feche a mesma.
        client.close().unwrap_or(())
      }
      Err(e) => {
        eprintln!("Error creating Redis client: {}", e);
        panic!("Failed to create Redis client");
      }
    };
  }
}
