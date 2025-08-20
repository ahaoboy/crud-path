use std::env;
use std::fs;
use std::io::Write;
use crate::expand_path;

#[cfg(target_os = "windows")]
const DELIMITER: &str = ";";

#[cfg(not(target_os = "windows"))]
const DELIMITER: &str = ":";

pub fn is_github() -> bool {
    std::env::var("GITHUB_ACTIONS") == Ok("true".to_string())
}

pub fn add_github_path(input_path: &str) -> Option<String> {
    let input_path = &expand_path(input_path);
    if let Ok(file_path) = env::var("GITHUB_PATH")
        && !file_path.is_empty() {
            issue_file_command("PATH", input_path);
        }

    let current_path = env::var("PATH").ok()?;
    let new_path = format!("{input_path}{DELIMITER}{current_path}");
    unsafe { env::set_var("PATH", &new_path) };
    Some(new_path)
}

fn issue_file_command(command: &str, message: &str) {
    let env_var_name = format!("GITHUB_{command}");
    if let Ok(file_path) = env::var(&env_var_name) {
        if fs::metadata(&file_path).is_err() {
            panic!("Missing file at path: {file_path}");
        }

        let mut file = fs::OpenOptions::new()
            .append(true)
            .open(&file_path)
            .expect("Failed to open file");

        writeln!(file, "{message}").expect("write file error");
    } else {
        panic!(
            "Unable to find environment variable for file command {command}"
        );
    }
}
