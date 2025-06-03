# freSH

**freSH** (FRiendly Easy SHell) is a simple, interactive command-line shell written in Rust. It aims to provide a user-friendly shell experience with essential features such as command execution, history management, and support for pipelines, all in a lightweight and modern Rust codebase.

---

## Overview

freSH is designed as a minimal yet functional shell for Unix-like systems. It provides a familiar shell prompt, supports running external commands, handles basic shell built-ins like `cd` and `exit`, and allows users to chain commands using pipelines (`|`). Command history is persisted across sessions for convenience.

---

## Features

- **Interactive Shell Prompt**
  Presents a prompt for user input, with a welcoming ASCII art banner.

- **Command Execution**
  Runs external commands with arguments, just like a typical shell.

- **Built-in Commands**
  - `cd <dir>`: Change the current working directory.
  - `exit`: Exit the shell gracefully.

- **Pipelines**
  Supports chaining commands using the pipe (`|`) operator, passing the output of one command as the input to the next.

- **Command History**
  Remembers previously entered commands across sessions using a persistent history file (`/tmp/.fresh_history`).

- **Graceful Exit**
  Handles `Ctrl-C` and `Ctrl-D` to exit cleanly, saving command history.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)

### Build and Run

```sh
git clone https://github.com/vrn21/fresh
cd fresh
cargo run
```

---

## Usage

- Type any command and press Enter to execute.
- Use `cd <directory>` to change directories.
- Use `exit` or press `Ctrl-D`/`Ctrl-C` to quit.
- Chain commands with `|` for pipelines, e.g.:
  ```
  ls -l | grep ".rs" | sort
  ```

---

## Architecture

### Main Components

- **Entry Point (`main.rs`)**
  The shell starts by displaying an ASCII art banner and a welcome message. It initializes a line editor using the [`rustyline`](https://crates.io/crates/rustyline) crate for interactive input and history management.

- **Event Loop**
  The shell enters a loop, prompting the user for input. For each line:
  - If the input is empty, it prompts again.
  - If the input is `exit`, the shell exits.
  - If the input is `cd <dir>`, it changes the current directory.
  - Otherwise, it parses the input for pipelines (`|`), splits the command, and executes each segment, connecting their input/output as needed.

- **Command Execution**
  Uses Rust's `std::process::Command` to spawn child processes for each command. Handles standard input/output redirection for pipelines.

- **History Management**
  Loads command history from `/tmp/.fresh_history` at startup and saves it on exit or interruption.

- **Error Handling**
  Prints errors to standard error if command execution fails or if built-in commands encounter issues.

---

## Dependencies

- [`rustyline`](https://crates.io/crates/rustyline) — For interactive input and history.


---

## License

This project is licensed under the MIT License.

---

**freSH** is a learning project and a foundation for building more advanced shell features in Rust. Contributions and suggestions are welcome!
