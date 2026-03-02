use crate::DELIMITER;
use path_clean::PathClean;
use std::ffi::OsStr;
use std::path::Path;

pub(crate) fn exec<S, I>(cmd: S, args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    std::process::Command::new(cmd)
        .envs(std::env::vars())
        .args(args)
        .output()
        .is_ok()
}

pub fn get_path() -> Vec<String> {
    let path = std::env::var("PATH").unwrap_or_default();
    path.split(DELIMITER)
        .filter(|s| !s.is_empty())
        .map(|s| s.replace("\\", "/"))
        .collect()
}

pub fn has_path(path: &str) -> bool {
    let path = normalize_for_comparison(&expand_path(path));
    get_path()
        .iter()
        .any(|entry| normalize_for_comparison(entry) == path)
}

/// Remove a path from the current process PATH environment variable.
pub(crate) fn remove_from_env_path(path: &str) {
    let target = normalize_for_comparison(path);
    let entries: Vec<String> = std::env::var("PATH")
        .unwrap_or_default()
        .split(DELIMITER)
        .filter(|s| !s.is_empty())
        .filter(|entry| normalize_for_comparison(entry) != target)
        .map(|s| s.to_string())
        .collect();
    let new_path = entries.join(&DELIMITER.to_string());
    // Safety: env mutation is not thread-safe, but this CLI is single-threaded.
    unsafe { std::env::set_var("PATH", new_path) };
}

/// Normalize a path for comparison.
/// On Windows, paths are case-insensitive, so we lowercase.
/// On Unix, paths are case-sensitive, so we return as-is.
/// Always normalizes slashes to forward slashes and trims trailing separators.
/// On Windows, also converts MSYS2-style paths (e.g., /c/Users) to c:/Users.
fn normalize_for_comparison(path: &str) -> String {
    let normalized = path.replace("\\", "/");
    let normalized = normalized.trim_end_matches('/');
    // Convert MSYS2-style drive paths: /c/... -> c:/...
    let normalized = if cfg!(windows)
        && normalized.len() >= 2
        && normalized.starts_with('/')
        && normalized.as_bytes()[1].is_ascii_alphabetic()
        && (normalized.len() == 2 || normalized.as_bytes()[2] == b'/')
    {
        let drive = normalized.as_bytes()[1] as char;
        format!("{}:{}", drive, &normalized[2..])
    } else {
        normalized.to_string()
    };
    if cfg!(windows) {
        normalized.to_ascii_lowercase()
    } else {
        normalized
    }
}

/// Convert a Unix-style path to a Windows-style path.
/// e.g. `c:/a/b` -> `C:\a\b`
pub fn to_win_path(path: &str) -> String {
    let mut path = path.replace("/", "\\");
    if path.len() >= 3
        && let Some(s) = path.get(1..3)
        && s == ":\\"
    {
        path[..1].make_ascii_uppercase();
    }
    path
}

pub fn is_msys() -> bool {
    std::env::var("MSYSTEM").is_ok()
}

/// Convert a Windows-style path to an MSYS-style path.
/// e.g. `C:\a\b` -> `/c/a/b`
pub fn to_msys_path(path: &str) -> String {
    let path = path.replace("\\", "/");
    if path.len() >= 3 && path.as_bytes().get(1) == Some(&b':') && path.as_bytes().get(2) == Some(&b'/') {
        let drive = path.as_bytes()[0].to_ascii_lowercase() as char;
        format!("/{drive}{}", &path[2..])
    } else {
        path
    }
}

pub(crate) fn expand_path(path: &str) -> String {
    let expanded = shellexpand::tilde(path);
    Path::new(&*expanded).clean().to_string_lossy().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_path() {
        let path = get_path();
        assert!(!path.is_empty());
    }

    #[test]
    fn test_to_msys_path() {
        assert_eq!(to_msys_path("C:\\Users\\test"), "/c/Users/test");
        assert_eq!(to_msys_path("D:/foo/bar"), "/d/foo/bar");
        assert_eq!(to_msys_path("/already/unix"), "/already/unix");
        assert_eq!(to_msys_path("relative/path"), "relative/path");
    }

    #[test]
    fn test_to_win_path() {
        assert_eq!(to_win_path("c:/a/b"), "C:\\a\\b");
        assert_eq!(to_win_path("d:/foo"), "D:\\foo");
        assert_eq!(to_win_path("/unix/path"), "\\unix\\path");
    }

    #[test]
    fn test_normalize_for_comparison() {
        let a = normalize_for_comparison("C:\\Users\\Test");
        let b = normalize_for_comparison("c:/users/test");
        if cfg!(windows) {
            assert_eq!(a, b);
        } else {
            assert_ne!(a, b);
        }
    }

    #[test]
    fn test_normalize_trailing_separator() {
        let a = normalize_for_comparison("C:\\Users\\Test\\");
        let b = normalize_for_comparison("C:\\Users\\Test");
        if cfg!(windows) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_normalize_msys_path() {
        let a = normalize_for_comparison("/c/Users/Test");
        let b = normalize_for_comparison("C:\\Users\\Test");
        if cfg!(windows) {
            assert_eq!(a, b);
        }
    }
}
