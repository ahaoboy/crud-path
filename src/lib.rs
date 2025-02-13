mod common;
pub use common::*;

#[cfg(not(target_os = "windows"))]
mod unix;
#[cfg(not(target_os = "windows"))]
pub use unix::*;

#[cfg(target_os = "windows")]
pub mod windows;
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
    fn test() {
        add_path("abcd");
        assert!(has_path("abcd"))
    }
}
