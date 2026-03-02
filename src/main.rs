use std::path::Path;

use crud_path::{add_github_path, add_path, get_path, has_path, is_github, remove_path};

/// Resolve a path: expand `~` via shellexpand, then convert relative paths to absolute.
fn resolve_path(path: &str) -> String {
    let expanded = shellexpand::tilde(path).to_string();
    let p = Path::new(&expanded);
    if p.is_absolute() {
        expanded
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(p).to_string_lossy().to_string(),
            Err(_) => expanded,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let Some(cmd) = args.get(1) else {
        print_usage();
        return;
    };

    match cmd.as_str() {
        "shell" => {
            println!("{:?}", which_shell::which_shell());
        }
        "add" => {
            let Some(input) = args.get(2) else {
                eprintln!("Error: 'add' requires a PATH argument");
                eprintln!("Usage: crud-path add <PATH>");
                std::process::exit(1);
            };
            let path = resolve_path(input);
            if let Some(sh) = add_path(&path) {
                println!("add {path} to {sh}");
            } else if !has_path(&path) {
                eprintln!("failed to add {path} to $PATH");
                std::process::exit(1);
            }
        }
        "remove" => {
            let Some(input) = args.get(2) else {
                eprintln!("Error: 'remove' requires a PATH argument");
                eprintln!("Usage: crud-path remove <PATH>");
                std::process::exit(1);
            };
            let path = resolve_path(input);
            if remove_path(&path) {
                println!("removed {path} from PATH");
            } else {
                println!("{path} is not in PATH");
            }
        }
        "has" => {
            let Some(input) = args.get(2) else {
                eprintln!("Error: 'has' requires a PATH argument");
                eprintln!("Usage: crud-path has <PATH>");
                std::process::exit(1);
            };
            let path = resolve_path(input);
            println!("{}", has_path(&path));
        }
        "get" => {
            println!("{}", get_path().join("\n"));
        }
        "is_github" => {
            println!("{}", is_github());
        }
        "add_github_path" => {
            let Some(input) = args.get(2) else {
                eprintln!("Error: 'add_github_path' requires a PATH argument");
                eprintln!("Usage: crud-path add_github_path <PATH>");
                std::process::exit(1);
            };
            let path = resolve_path(input);
            if is_github() {
                match add_github_path(&path) {
                    Some(new_path) => println!("{new_path}"),
                    None => {
                        eprintln!("failed to add {path} to GitHub PATH");
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("not in GitHub Actions environment");
                std::process::exit(1);
            }
        }
        other => {
            eprintln!("Unknown command: {other}");
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage: crud-path <COMMAND> [PATH]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  get              List all entries in PATH");
    eprintln!("  has <PATH>       Check if PATH contains an entry");
    eprintln!("  add <PATH>       Add an entry to PATH");
    eprintln!("  remove <PATH>    Remove an entry from PATH");
    eprintln!("  shell            Show detected shell");
    eprintln!("  is_github        Check if running in GitHub Actions");
    eprintln!("  add_github_path  Add an entry to GitHub Actions PATH");
}
