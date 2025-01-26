use crate::exec;
use which_shell::Shell;

pub fn add_path(path: &str) -> Option<Shell> {
    if let Some(which_shell::ShellVersion { shell, version: _ }) = which_shell::which_shell() {
        match shell {
            which_shell::Shell::Fish => {
                let cmd = format!(
                    r#"echo '
set -gx PATH "{path}" $PATH
' >> ~/.config/fish/config.fish"#
                );
                if exec("fish", ["-c", &cmd]) {
                    return Some(shell);
                } else {
                    return None;
                }
            }
            which_shell::Shell::Zsh => {
                let cmd = format!(
                    r#"echo '
export PATH="{path}:$PATH"
' >> ~/.zshrc"#
                );
                if exec("bash", ["-c", &cmd]) {
                    return Some(shell);
                } else {
                    return None;
                }
            }
            which_shell::Shell::Bash => {
                let cmd = format!(
                    r#"echo '
export PATH="{path}:$PATH"
' >> ~/.bashrc"#
                );
                if exec("bash", ["-c", &cmd]) {
                    return Some(shell);
                } else {
                    return None;
                }
            }
            _ => {
                return None;
            }
        }
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
