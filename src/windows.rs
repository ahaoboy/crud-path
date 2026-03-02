use std::path::Path;

use crate::{DELIMITER, exec, expand_path, has_path, remove_from_env_path, to_win_path, unix};
use which_shell::Shell;

#[allow(dead_code)]
pub fn add_path(path: &str) -> Option<Shell> {
    let abs_path = expand_path(path);

    // Validate that the path is absolute
    if !Path::new(&abs_path).is_absolute() {
        log::error!("'{abs_path}' is not an absolute path");
        return None;
    }

    // Warn if the path does not exist, but continue
    if !Path::new(&abs_path).exists() {
        log::warn!("'{abs_path}' does not exist");
    } else if !Path::new(&abs_path).is_dir() {
        log::warn!("'{abs_path}' is not a directory");
    }

    // Skip if already in PATH
    if has_path(&abs_path) {
        log::info!("{abs_path} is already in PATH");
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

#[allow(dead_code)]
pub fn remove_path(path: &str) -> bool {
    let abs_path = expand_path(path);
    let path_win = to_win_path(&abs_path);
    let mode = if is_admin::is_admin() {
        "Machine"
    } else {
        "User"
    };

    // Read the persistent (registry) PATH directly via PowerShell,
    // because the current process env var may be stale if another
    // terminal modified PATH after this shell was started.
    let escaped = path_win.replace("'", "''");
    let check_and_remove = format!(
        r#"$p = [Environment]::GetEnvironmentVariable('Path', '{mode}'); $entries = ($p -split ';') | Where-Object {{ $_.Trim() -ne '' }}; $found = $entries | Where-Object {{ $_ -ieq '{escaped}' }}; if ($found) {{ $remaining = $entries | Where-Object {{ $_ -ine '{escaped}' }}; [Environment]::SetEnvironmentVariable('Path', ($remaining -join ';'), '{mode}'); Write-Output 'REMOVED' }} else {{ Write-Output 'NOT_FOUND' }}"#,
    );

    let output = std::process::Command::new("powershell")
        .args(["-c", &check_and_remove])
        .envs(std::env::vars())
        .output();

    let removed_from_registry = match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            stdout.trim() == "REMOVED"
        }
        Err(_) => false,
    };

    // Also check the current process env var (covers MSYS2 paths)
    let in_env = has_path(&abs_path);

    if !removed_from_registry && !in_env {
        return false;
    }

    // Clean up the current process env var
    if in_env {
        remove_from_env_path(&abs_path);
    }

    // Also remove from shell config (bash/zsh/fish) for MSYS2/Git Bash.
    unix::remove_path_from_shell(which_shell::Shell::Bash, &abs_path);
    unix::remove_path_from_shell(which_shell::Shell::Zsh, &abs_path);
    unix::remove_path_from_shell(which_shell::Shell::Fish, &abs_path);
    true
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
