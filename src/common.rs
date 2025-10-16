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
    let path = std::env::var("PATH")
        .expect("Failed to get PATH")
        .to_string();
    path.split(DELIMITER)
        .map(|s| s.replace("\\", "/").to_string())
        .collect()
}

pub fn has_path(path: &str) -> bool {
    let path = &expand_path(path);
    #[cfg(windows)]
    let path = to_win_path(path).replace("\\", "/");
    get_path().iter().any(|i| i.eq_ignore_ascii_case(&path))
}

// c:/a/b -> C:\a\b
pub fn to_win_path(path: &str) -> String {
    let mut path = path.replace("/", "\\");
    if let Some(s) = path.as_mut_str().get_mut(0..3)
        && s.ends_with(":\\")
    {
        s.make_ascii_uppercase();
    }
    path
}

pub fn is_msys() -> bool {
    std::env::var("MSYSTEM").is_ok()
}

// C:\a\b -> /c/a/b
pub fn to_msys_path(path: &str) -> String {
    let mut path = path.replace("\\", "/");
    if let Some(s) = path.as_mut_str().get_mut(0..3)
        && s.len() == 3
        && s.ends_with(":/")
    {
        unsafe {
            let p = s.as_mut_ptr();
            let name = (*p).to_ascii_lowercase();
            *p = b'/';
            *(p.wrapping_add(1)) = name;
            *(p.wrapping_add(2)) = b'/';
        };
    }
    path
}

pub(crate) fn expand_path(path: &str) -> String {
    let expanded = shellexpand::tilde(path);
    Path::new(&*expanded).clean().to_string_lossy().to_string()
}

#[cfg(test)]
mod test {
    use super::get_path;

    #[test]
    fn test_get_path() {
        let path = get_path();
        assert!(!path.is_empty())
    }
}
