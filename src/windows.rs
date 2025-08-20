use crate::{exec, expand_path, to_win_path, unix, DELIMITER};
use which_shell::Shell;

pub fn add_path(path: &str) -> Option<Shell> {
    let abs_path = expand_path(path);
    let path_win = to_win_path(&abs_path);
    let mode = if is_admin::is_admin() {
        "Machine"
    } else {
        "User"
    };
    let shell = format!(
        r#"$currentPath = [Environment]::GetEnvironmentVariable("Path", "{mode}");$newPath = "$currentPath;{path_win}"; [Environment]::SetEnvironmentVariable("Path", $newPath, "{mode}")"#,
    );

    if exec("powershell", ["-c", &shell]) {
        let mut env_path = std::env::var("PATH").unwrap_or_default();
        env_path.push(DELIMITER);
        env_path.push_str(&abs_path);

        unsafe { std::env::set_var("PATH", env_path) };
        if let Some(sh) = unix::add_path(&abs_path) {
            return Some(sh);
        }
        return Some(Shell::PowerShell);
    }
    None
}

#[cfg(test)]
mod test {
    use super::add_path;

    #[test]
    fn test_add_path() {
        let s = "c:/xxx";
        let s = add_path(s);
        assert!(s.is_some());
    }
}
