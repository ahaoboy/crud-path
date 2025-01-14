pub fn get_path() -> Vec<String> {
  let path = std::env::var("PATH")
      .expect("Failed to get PATH")
      .to_string();
  path.split(':').map(|s| s.to_string()).collect()
}


#[cfg(test)]
mod test {
    use super::{add_path, get_path};

    #[test]
    fn test_get_path() {
        let path = get_path();
        assert!(path.len() > 0)
    }

    #[test]
    fn test_add_path() {
        let s = "./xxx";
        let s = add_path(s);
        assert!(s);
    }
}
