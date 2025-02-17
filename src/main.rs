use crud_path::{add_github_path, add_path, get_path, has_path, is_github};

fn main() {
    if let Some(cmd) = std::env::args().nth(1) {
        match cmd.as_str() {
            "shell" => {
                println!("{:?}", which_shell::which_shell());
                return;
            }
            "add" => {
                if let Some(path) = std::env::args().nth(2) {
                    if let Some(sh) = add_path(&path) {
                        println!("add {} to {}", path, sh);
                    } else {
                        println!("failed to add {} to $PATH", path);
                    }
                    return;
                }
            }
            "has" => {
                if let Some(path) = std::env::args().nth(2) {
                    println!("{}", has_path(&path));
                    return;
                }
            }
            "get" => {
                println!("{}", get_path().join("\n"));
                return;
            }
            "is_github" => {
                println!("{}", is_github());
                return;
            }
            "add_github_path" => {
                if let Some(path) = std::env::args().nth(2) {
                    if is_github() {
                        println!("{}", add_github_path(&path).unwrap());
                    } else {
                        println!("not in github action")
                    }
                }
                return;
            }
            _ => {}
        }
    }
    println!("Usage: curd-path get/has/add/is_github/add_github_path/shell <PATH>");
}
