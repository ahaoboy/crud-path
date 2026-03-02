mod common;
pub use common::*;

pub(crate) mod unix;
#[cfg(not(target_os = "windows"))]
pub use unix::*;

pub(crate) mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

mod github;
pub use github::*;

#[cfg(target_os = "windows")]
pub const DELIMITER: char = ';';

#[cfg(not(target_os = "windows"))]
pub const DELIMITER: char = ':';

#[cfg(test)]
mod test {
    use crate::{add_path, has_path};

    #[test]
    fn test_add_and_has_path() {
        #[cfg(target_os = "windows")]
        let path = "C:/test_crud_path_abcd";
        #[cfg(not(target_os = "windows"))]
        let path = "/test_crud_path_abcd";

        add_path(path);
        assert!(has_path(path));
    }

    #[test]
    fn test_add_relative_path_returns_none() {
        let result = add_path("relative_path");
        assert!(result.is_none());
    }
}
