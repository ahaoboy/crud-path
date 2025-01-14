use crate::exec;

pub fn add_path(path: &str) -> bool {
    let mode = if is_admin::is_admin() {
        "Machine"
    } else {
        "User"
    };
    let shell = format!(
        r#"$currentPath = [Environment]::GetEnvironmentVariable("Path", "{mode}");$newPath = "$currentPath;{path}"; [Environment]::SetEnvironmentVariable("Path", $newPath, "{mode}")"#,
    );

    exec("powershell", ["-c", &shell])
}

#[cfg(test)]
mod test {
    use super::add_path;

    #[test]
    fn test_add_path() {
        let s = "c:/xxx";
        let s = add_path(s);
        assert!(s);
    }
}
