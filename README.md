# rust-todo

A to-do list CLI tool written in Rust.

## Usage

```shell
A todo-list CLI tool written in Rust

Usage:
    rtodo [OPTIONS]
    rtodo [OPTIONS] [ARGUMENTS]

Options:
    --help, -h                     Prints this help message
    --get, -g     [INDEX]          Prints todo info at index
    --list, -l                     Prints all todos
    --new, -n     [NAME]           Creates new todo with name
    --clear, -c                    Clears all todos
    --remove, -r  [INDEX]          Removes todo at index
    --tag, -t     [INDEX] [COLOR]  Tags todo at index with color
    --done, d     [INDEX]          Makes todo finished/unfinished
    --version, -v                  Prints version info

Tag Colors:
    red ⦿
    blue ⦿
    yellow ⦿
    green ⦿
    cyan ⦿
    purple, magenta ⦿
```

## Installation

Install Rust and Cargo using [`rustup.rs`](https://rustup.rs/)

Check if it's properly installed:

```
rustc --version
cargo --version
```

### With crates.io

```
cargo install rust-todo
```

### Building from source



1. Clone the repository:
    ```
    git clone https://github.com/yees7/rust-todo
    ```
1. `cd` into the directory and build with `release` flag:
    ```
    cd rust-todo
    cargo build --release
    ```
1. `rust-todo` executable will be available in `target/release/rust-todo`
