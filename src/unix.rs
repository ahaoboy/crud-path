use std::path::Path;

use crate::{exec, expand_path, has_path, is_msys, to_msys_path};
use which_shell::Shell;

/// Returns the shell config file path and export syntax for the given shell.
fn shell_config(shell: Shell) -> Option<(&'static str, &'static str)> {
    match shell {
        Shell::Fish => Some(("~/.config/fish/config.fish", "set -gx PATH \"{}\" $PATH")),
        Shell::Zsh => Some(("~/.zshrc", "export PATH=\"{}:$PATH\"")),
        Shell::Bash | Shell::Sh => Some(("~/.bashrc", "export PATH=\"{}:$PATH\"")),
        _ => None,
    }
}

/// Check if the content contains the given line.
/// On Windows/MSYS, comparison is case-insensitive.
fn config_contains(content: &str, line: &str) -> bool {
    if cfg!(windows) {
        content.to_ascii_lowercase().contains(&line.to_ascii_lowercase())
    } else {
        content.contains(line)
    }
}

pub fn add_path_to_shell(shell: Shell, path: &str) -> bool {
    let path = if cfg!(windows) || is_msys() {
        &to_msys_path(path)
    } else {
        path
    };

    let (config_file, template) = match shell_config(shell) {
        Some(c) => c,
        None => return false,
    };

    let export_line = template.replace("{}", path);

    // Check if the export line already exists in the config file
    let config_path = expand_path(config_file);
    if let Ok(content) = std::fs::read_to_string(&config_path)
        && config_contains(&content, &export_line) {
            eprintln!("{path} is already in {config_file}");
            return true;
        }

    let cmd_str = format!("echo '\n{export_line}\n' >> {config_file}");
    let shell_cmd = if matches!(shell, Shell::Fish) {
        "fish"
    } else {
        "sh"
    };

    if is_admin::is_admin() {
        // For admin, also write to /etc/profile using the appropriate syntax
        let admin_export = if matches!(shell, Shell::Fish) {
            format!("set -gx PATH \"{path}\" $PATH")
        } else {
            format!("export PATH=\"{path}:$PATH\"")
        };
        // Only append if not already present
        if let Ok(content) = std::fs::read_to_string("/etc/profile")
            && config_contains(&content, &admin_export) {
                return exec(shell_cmd, ["-c", &cmd_str]);
            }
        exec(
            "sh",
            ["-c", &format!("echo '\n{admin_export}\n' >> /etc/profile")],
        );
    }

    exec(shell_cmd, ["-c", &cmd_str])
}

#[allow(dead_code)]
pub fn add_path(path: &str) -> Option<Shell> {
    let path = &expand_path(path);

    // Validate that the path is absolute
    if !Path::new(path).is_absolute() {
        eprintln!("Error: '{path}' is not an absolute path");
        return None;
    }

    // Warn if the path does not exist, but continue
    if !Path::new(path).exists() {
        eprintln!("Warning: '{path}' does not exist");
    }

    // Skip if already in PATH
    if has_path(path) {
        return None;
    }

    // Try the detected shell first, fall back to bash
    if let Some(shell) = which_shell::which_shell()
        && add_path_to_shell(shell.shell, path)
    {
        return Some(shell.shell);
    }
    if add_path_to_shell(Shell::Bash, path) {
        return Some(Shell::Bash);
    }
    None
}

#[cfg(test)]
mod test {
    use super::add_path;

    #[test]
    fn test_add_path() {
        let s = "/xxx";
        let s = add_path(s);
        assert!(s.is_some());
    }
}
