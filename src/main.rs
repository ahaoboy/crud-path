use crud_path::{add_path, get_path, has_path};

fn main() {
    if let Some(cmd) = std::env::args().nth(1) {
        match cmd.as_str() {
            "add" => {
                if let Some(path) = std::env::args().nth(2) {
                    println!("{}", add_path(&path));
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
            _ => {}
        }
    }
    println!("Usage: curd-path get/has/add <PATH>");
}
