use std::ffi::OsStr;

pub(crate) fn exec<S, I>(cmd: S, args: I) -> bool
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    std::process::Command::new(cmd).args(args).output().is_ok()
}

pub fn get_path() -> Vec<String> {
    let path = std::env::var("PATH")
        .expect("Failed to get PATH")
        .to_string();
    let delimiter = if cfg!(target_os = "windows") {
        ';'
    } else {
        ':'
    };
    path.split(delimiter).map(|s| s.to_string()).collect()
}

pub fn has_path(path: &str) -> bool {
    get_path().contains(&path.to_string())
}

#[cfg(test)]
mod test {
    use super::get_path;

    #[test]
    fn test_get_path() {
        let path = get_path();
        assert!(path.len() > 0)
    }
}
