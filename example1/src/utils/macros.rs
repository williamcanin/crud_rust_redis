#[macro_export]
macro_rules! string_to_bool {
  ($s:expr) => {
    matches!($s.to_lowercase().as_str(), "true")
  };
}
