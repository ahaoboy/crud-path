# crud-path

This Rust project provides a simple way to manage the system's `PATH` variable
programmatically. It includes the following functionalities:

## Installation

```bash
cargo binstall crud-path

crud-path get
crud-path add /a/b/c
source ~/.bashrc
crud-path has /a/b/c
crud-path is_github
crud-path add_github_path /x/y
```

```bash
npm i crud-path -g
```

## Features

### 1. `get_path`

- **Description:** Retrieves the current system `PATH` variable.
- **Returns:** A list of strings representing the paths in the `PATH` variable.
- **Note:** Since terminal-specific modifications to `PATH` may not match
  system-wide settings, the result could differ from configuration files.

### 2. `has_path`

- **Description:** Checks if a specific string exists in the system's `PATH`
  variable.
- **Returns:** A boolean indicating whether the given path is in `PATH`.
- **Note:** Changes may require restarting the terminal to take effect.

### 3. `add_path`

- **Description:** Adds a string to the `PATH` variable.
  - On Windows: Uses PowerShell to modify the `PATH` variable.
  - On Unix-like systems: Updates the corresponding shell configuration file
    based on the terminal in use.
- **Note:** Changes may require restarting the terminal to take effect.
