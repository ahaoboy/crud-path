use std::path::Path;

use crate::{DELIMITER, exec, expand_path, has_path, to_win_path, unix};
use which_shell::Shell;

#[allow(dead_code)]
pub fn add_path(path: &str) -> Option<Shell> {
    let abs_path = expand_path(path);

    // Validate that the path is absolute
    if !Path::new(&abs_path).is_absolute() {
        eprintln!("Error: '{abs_path}' is not an absolute path");
        return None;
    }

    // Warn if the path does not exist, but continue
    if !Path::new(&abs_path).exists() {
        eprintln!("Warning: '{abs_path}' does not exist");
    }

    // Skip if already in PATH
    if has_path(&abs_path) {
        eprintln!("{abs_path} is already in PATH");
        return None;
    }

    let path_win = to_win_path(&abs_path);
    let mode = if is_admin::is_admin() {
        "Machine"
    } else {
        "User"
    };

    // Escape single quotes in path for PowerShell safety
    let escaped = path_win.replace("'", "''");
    // Check if the path already exists in the persistent (registry) PATH before appending,
    // to prevent duplicates when the shell hasn't been restarted.
    let shell = format!(
        r#"$p = [Environment]::GetEnvironmentVariable('Path', '{mode}'); if (($p -split ';') -notcontains '{escaped}') {{ [Environment]::SetEnvironmentVariable('Path', "$p;{escaped}", '{mode}') }}"#,
    );

    if exec("powershell", ["-c", &shell]) {
        let mut env_path = std::env::var("PATH").unwrap_or_default();
        env_path.push(DELIMITER);
        env_path.push_str(&abs_path);

        // Safety: env mutation is not thread-safe, but this CLI is single-threaded.
        unsafe { std::env::set_var("PATH", env_path) };
        // Also add to shell config (bash/zsh/fish) for MSYS2/Git Bash.
        // Call add_path_to_shell directly to avoid duplicate validation.
        if let Some(shell) = which_shell::which_shell()
            && unix::add_path_to_shell(shell.shell, &abs_path) {
                return Some(shell.shell);
            }
        if unix::add_path_to_shell(Shell::Bash, &abs_path) {
            return Some(Shell::Bash);
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
