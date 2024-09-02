use redis::RedisError;
use std::error::Error;
use std::io;

// Função para tratar erros
pub fn handle_error(error: Box<dyn Error>) {
  if let Some(redis_error) = error.downcast_ref::<RedisError>() {
    if let Some(io_error) = redis_error
      .source()
      .and_then(|e| e.downcast_ref::<io::Error>())
    {
      match io_error.kind() {
        io::ErrorKind::TimedOut => {
          println!("Erro de conexão: O servidor Redis não respondeu dentro do tempo limite.");
        }
        _ => {
          println!("Erro de I/O: {}", io_error);
        }
      }
    } else {
      println!("Erro Redis: {}", redis_error);
    }
  } else {
    println!("Erro desconhecido: {}", error);
  }
}
