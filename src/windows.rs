use std::path::{Path, PathBuf};

use crate::{
    DELIMITER, append_line_once, exec, expand_path, has_path, remove_from_env_path, to_win_path,
    unix,
};
use which_shell::Shell;

#[allow(dead_code)]
pub fn add_path(path: &str, force: bool) -> Option<Shell> {
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

    // Skip if already in PATH (unless forced)
    if !force && has_path(&abs_path) {
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
        // `add_path_to_shell` dispatches PowerShell to `$PROFILE` and the rest
        // to their Unix-style config files.
        if let Some(shell) = which_shell::which_shell()
            && add_path_to_shell(shell.shell, &abs_path).is_some()
        {
            return Some(shell.shell);
        }
        if add_path_to_shell(Shell::Bash, &abs_path).is_some() {
            return Some(Shell::Bash);
        }
        return Some(Shell::PowerShell);
    }
    None
}

/// Add `path` to the config file of `shell`.
///
/// Windows has no single login shell: PowerShell keeps `PATH` in a profile
/// script (resolved with `$PROFILE`, so OneDrive-redirected Documents work),
/// while MSYS2/Git Bash and the other shells use the same files as on Unix.
/// Returns the file that was updated.
pub fn add_path_to_shell(shell: Shell, path: &str) -> Option<PathBuf> {
    match shell {
        Shell::PowerShell | Shell::Pwsh => add_to_powershell_profile(shell, path),
        _ => unix::add_path_to_shell(shell, path),
    }
}

fn add_to_powershell_profile(shell: Shell, path: &str) -> Option<PathBuf> {
    // `Shell::config_file` resolves `$PROFILE` for PowerShell on Windows.
    let file = shell.config_file()?;
    let line = shell.path_line(path)?;
    append_line_once(&file, &line).then_some(file)
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

    /// Mutates the real user `PATH` (registry) and shell profile, so it is not
    /// part of the automated suite. Run manually with
    /// `cargo test -- --ignored`.
    #[test]
    #[cfg_attr(windows, ignore = "writes to the real user PATH and the shell profile")]
    fn test_add_path() {
        let s = "c:/xxx";
        let s = add_path(s, false);
        assert!(s.is_some());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The profile line must be written once, however often the install runs.
    #[test]
    fn append_line_once_is_idempotent() {
        let dir = std::env::temp_dir().join(format!("crud-path-profile-{}", std::process::id()));
        let file = dir.join("Microsoft.PowerShell_profile.ps1");
        let line = r#"$env:Path += [IO.Path]::PathSeparator + "C:/x""#;

        assert!(append_line_once(&file, line));
        assert!(append_line_once(&file, line));

        let text = std::fs::read_to_string(&file).unwrap();
        assert_eq!(text.lines().count(), 1, "{text}");
        assert_eq!(text.trim(), line);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
