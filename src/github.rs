use crate::expand_path;
use std::env;
use std::fs;
use std::io::Write;

pub fn is_github() -> bool {
    env::var("GITHUB_ACTIONS").as_deref() == Ok("true")
}

pub fn add_github_path(input_path: &str) -> Option<String> {
    let input_path = expand_path(input_path);

    if let Ok(file_path) = env::var("GITHUB_PATH")
        && !file_path.is_empty()
        && let Err(e) = issue_file_command("PATH", &input_path)
    {
        log::warn!("failed to write to GITHUB_PATH file: {e}");
    }

    let current_path = env::var("PATH").unwrap_or_default();
    let delimiter = crate::DELIMITER;
    let new_path = format!("{input_path}{delimiter}{current_path}");
    // Safety: env mutation is not thread-safe, but this CLI is single-threaded.
    unsafe { env::set_var("PATH", &new_path) };
    Some(new_path)
}

fn issue_file_command(command: &str, message: &str) -> Result<(), String> {
    let env_var_name = format!("GITHUB_{command}");
    let file_path = env::var(&env_var_name)
        .map_err(|_| format!("Missing environment variable: {env_var_name}"))?;

    if !std::path::Path::new(&file_path).exists() {
        return Err(format!("Missing file at path: {file_path}"));
    }

    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&file_path)
        .map_err(|e| format!("Failed to open {file_path}: {e}"))?;

    writeln!(file, "{message}").map_err(|e| format!("Failed to write to {file_path}: {e}"))
}
