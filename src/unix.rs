use crate::{exec, is_msys, to_msys_path};
use which_shell::Shell;

pub fn add_path_to_shell(shell: Shell, path: &str) -> bool {
    let path = if cfg!(windows) || is_msys() {
        &to_msys_path(path)
    } else {
        path
    };
    let (cmd, args) = match shell {
        which_shell::Shell::Fish => (
            "fish",
            [
                "-c",
                &format!(
                    r#"echo '
set -gx PATH "{path}" $PATH
' >> ~/.config/fish/config.fish"#
                ),
            ],
        ),
        which_shell::Shell::Zsh => (
            "bash",
            [
                "-c",
                &format!(
                    r#"echo '
export PATH="{path}:$PATH"
' >> ~/.zshrc"#
                ),
            ],
        ),
        which_shell::Shell::Bash => (
            "bash",
            [
                "-c",
                &format!(
                    r#"echo '
export PATH="{path}:$PATH"
' >> ~/.bashrc"#
                ),
            ],
        ),
        _ => return false,
    };

    exec(cmd, args)
}

pub fn add_path(path: &str) -> Option<Shell> {
    // By default, bash is used as a fallback
    if let Some(shell) = which_shell::which_shell() {
        if add_path_to_shell(shell.shell, path) {
            return Some(shell.shell);
        }
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
