pub fn get_path() -> Vec<String> {
    let path = std::env::var("PATH")
        .expect("Failed to get PATH")
        .to_string();
    path.split(';').map(|s| s.to_string()).collect()
}

pub fn has_path(path: &str) -> bool {
    get_path().contains(&path.to_string())
}

pub fn add_path(path: &str) -> bool {
    let mode = if is_admin::is_admin() {
        "Machine"
    } else {
        "User"
    };
    let shell = format!(
        r#"$currentPath = [Environment]::GetEnvironmentVariable("Path", "{mode}");$newPath = "$currentPath;{path}"; [Environment]::SetEnvironmentVariable("Path", $newPath, "{mode}")"#,
    );
    std::process::Command::new("powershell")
        .args(["-c", &shell])
        .output()
        .is_ok()
}

#[cfg(test)]
mod test {
    use super::{add_path, get_path};

    #[test]
    fn test_get_path() {
        let path = get_path();
        assert!(path.len() > 0)
    }

    #[test]
    fn test_add_path() {
        let s = "./xxx";
        let s = add_path(s);
        assert!(s);
    }
}
