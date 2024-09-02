#[derive(Debug)]
pub struct User {
  pub name: String,
  pub email: String,
  pub country: String,
  pub site: String,
  pub graduation: bool,
}

pub struct ConnectionData {
  pub development: bool,
  pub username: String,
  pub password: String,
  pub hostname: String,
  pub port: String,
}
