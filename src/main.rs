use crud_path::{add_github_path, add_path, get_path, has_path, is_github};

fn main() {
    if let Some(cmd) = std::env::args().nth(1) {
        match cmd.as_str() {
            "add" => {
                if let Some(path) = std::env::args().nth(2) {
                    println!("{}", add_path(&path).is_some());
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
                    }
                }
                return;
            }
            _ => {}
        }
    }
    println!("Usage: curd-path get/has/add <PATH>");
}
