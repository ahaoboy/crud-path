use crate::{exec, is_msys, to_msys_path};
use which_shell::Shell;

pub fn add_path(path: &str) -> Option<Shell> {
    let path = if cfg!(windows) || is_msys() {
        &to_msys_path(path)
    } else {
        path
    };

    // By default, bash is used as a fallback
    let shell = which_shell::which_shell().map_or(Shell::Bash, |i| i.shell);

    match shell {
        which_shell::Shell::Fish => {
            let cmd = format!(
                r#"echo '
set -gx PATH "{path}" $PATH
' >> ~/.config/fish/config.fish"#
            );
            if exec("fish", ["-c", &cmd]) {
                Some(shell)
            } else {
                None
            }
        }
        which_shell::Shell::Zsh => {
            let cmd = format!(
                r#"echo '
export PATH="{path}:$PATH"
' >> ~/.zshrc"#
            );
            if exec("bash", ["-c", &cmd]) {
                Some(shell)
            } else {
                None
            }
        }
        which_shell::Shell::Bash => {
            let cmd = format!(
                r#"echo '
export PATH="{path}:$PATH"
' >> ~/.bashrc"#
            );
            if exec("bash", ["-c", &cmd]) {
                Some(shell)
            } else {
                None
            }
        }
        _ => {
            None
        }
    }
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
