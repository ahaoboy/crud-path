use std::path::{Path, PathBuf};

use crate::{
    append_line_once, config_contains, expand_path, has_path, is_msys, remove_from_env_path,
    to_msys_path,
};
use which_shell::Shell;

/// Absolute path of `shell`'s config file, when this crate can edit it.
pub fn config_file(shell: Shell) -> Option<PathBuf> {
    shell
        .path_config()
        .map(|config| PathBuf::from(expand_path(&format!("~/{}", config.file))))
}

/// Append `path` to `shell`'s config file.
///
/// Returns the file that was (or already had been) updated, so callers can
/// show the user exactly where the entry went.
pub fn add_path_to_shell(shell: Shell, path: &str) -> Option<PathBuf> {
    let path = if cfg!(windows) || is_msys() {
        to_msys_path(path)
    } else {
        path.to_string()
    };
    let line = shell.path_line(&path)?;
    let file = config_file(shell)?;

    if !append_line_once(&file, &line) {
        return None;
    }

    // Running as root: make it available to every user as well.
    #[cfg(not(windows))]
    if is_admin::is_admin() {
        let _ = append_line_once(Path::new("/etc/profile"), &line);
    }

    Some(file)
}

#[allow(dead_code)]
pub fn add_path(path: &str, force: bool) -> Option<Shell> {
    let path = &expand_path(path);

    // Validate that the path is absolute
    if !Path::new(path).is_absolute() {
        log::error!("'{path}' is not an absolute path");
        return None;
    }

    // Warn if the path does not exist, but continue
    if !Path::new(path).exists() {
        log::warn!("'{path}' does not exist");
    } else if !Path::new(path).is_dir() {
        log::warn!("'{path}' is not a directory");
    }

    // Skip if already in PATH (unless forced)
    if !force && has_path(path) {
        return None;
    }

    // Try the detected shell first, fall back to bash
    if let Some(shell) = which_shell::which_shell()
        && add_path_to_shell(shell.shell, path).is_some()
    {
        return Some(shell.shell);
    }
    if add_path_to_shell(Shell::Bash, path).is_some() {
        return Some(Shell::Bash);
    }
    None
}

pub fn remove_path_from_shell(shell: Shell, path: &str) -> bool {
    let path = if cfg!(windows) || is_msys() {
        to_msys_path(path)
    } else {
        path.to_string()
    };

    let Some(export_line) = shell.path_line(&path) else {
        return false;
    };
    let Some(config_path) = config_file(shell) else {
        return false;
    };

    if let Ok(content) = std::fs::read_to_string(&config_path)
        && config_contains(&content, &export_line)
    {
        let new_content = content
            .lines()
            .filter(|line| !config_contains(line, &export_line))
            .collect::<Vec<_>>()
            .join("\n");
        if std::fs::write(&config_path, new_content).is_ok() {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
pub fn remove_path(path: &str) -> bool {
    let path = &expand_path(path);

    if !has_path(path) {
        return false;
    }

    remove_from_env_path(path);

    // Try to remove from all known shell configs
    if let Some(shell) = which_shell::which_shell() {
        remove_path_from_shell(shell.shell, path);
    }
    remove_path_from_shell(Shell::Bash, path);
    remove_path_from_shell(Shell::Zsh, path);
    remove_path_from_shell(Shell::Fish, path);

    true
}

#[cfg(all(test, unix))]
mod test {
    use super::add_path;

    /// Unix-only: `/xxx` is not an absolute path on Windows (no drive prefix),
    /// so `add_path` rejects it there before touching any config file.
    #[test]
    fn test_add_path() {
        let s = "/xxx";
        let s = add_path(s, false);
        assert!(s.is_some());
    }
}
