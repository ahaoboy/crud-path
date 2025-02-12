use crate::{exec, DELIMITER};
use which_shell::Shell;

pub fn add_path(path: &str) -> Option<Shell> {
    let mode = if is_admin::is_admin() {
        "Machine"
    } else {
        "User"
    };
    let shell = format!(
        r#"$currentPath = [Environment]::GetEnvironmentVariable("Path", "{mode}");$newPath = "$currentPath;{path}"; [Environment]::SetEnvironmentVariable("Path", $newPath, "{mode}")"#,
    );

    if exec("powershell", ["-c", &shell]) {
        let mut env_path = std::env::var("PATH").unwrap_or_default();
        env_path.push(DELIMITER);
        env_path.push_str(&path);

        std::env::set_var("PATH", env_path);
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
