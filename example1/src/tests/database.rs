#[test]
fn connection() {
  let data = crate::options::connection_data();
  match crate::database::controller::Database::connection(data) {
    Ok(mut client) => {
      // Caso tenha conexão, feche a mesma.
      client.close().unwrap_or(());
    }
    Err(e) => {
      panic!("Failed to create Redis client: {}", e);
    }
  };
}
