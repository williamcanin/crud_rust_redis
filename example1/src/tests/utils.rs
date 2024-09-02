#[cfg(test)]
mod catch {
  #[test]
  fn url_password() {
    use crate::utils::catch::url_password;
    let url = "rediss://default:123456789@localhost:6379";
    let pass = url_password(url);
    println!("Password: {}", pass.unwrap_or(""));
    assert_eq!("123456789", pass.unwrap_or(""));
  }
}
