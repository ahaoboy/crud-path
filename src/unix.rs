use crate::exec;

pub fn add_path(path: &str) -> bool {
    if let Some(which_shell::ShellVersion { shell, version: _ }) = which_shell::which_shell() {
        match shell {
            which_shell::Shell::Fish => {
                let shell = format!(
                    r#"echo '
set -gx PATH "{path}" $PATH
' >> ~/.config/fish/config.fish"#
                );
                println!("shell: {}",shell);
                return exec("fish", ["-c", &shell]);
            }
            which_shell::Shell::Zsh => {
                let shell = format!(
                    r#"echo '
export PATH="{path}:$PATH"
' >> ~/.zshrc"#
                );
                return exec("bash", ["-c", &shell]);
            }
            which_shell::Shell::Bash => {
                let shell = format!(
                    r#"echo '
export PATH="{path}:$PATH"
' >> ~/.bashrc"#
                );
                return exec("bash", ["-c", &shell]);
            }
            _ => {
                return false;
            }
        }
    }

    false
}
#[cfg(test)]
mod test {
    use super::add_path;

    #[test]
    fn test_add_path() {
        let s = "/xxx";
        let s = add_path(s);
        assert!(s);
    }
}
