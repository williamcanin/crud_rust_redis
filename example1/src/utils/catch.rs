#[allow(dead_code)]
// UNUSED
pub fn url_password(url: &str) -> Option<&str> {
  // Verifica se a URL contém as partes "://", "@" e ":"
  if let Some(start) = url
    .find("://")
    .and_then(|pos| url[pos + 3..].find(':').map(|p| pos + 3 + p + 1))
  {
    if let Some(end) = url[start..].find('@') {
      return Some(&url[start..start + end]);
    }
  }
  None
}
