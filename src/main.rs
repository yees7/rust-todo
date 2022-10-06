use std::{env, fs};
use colored::Colorize;

pub mod todo;
pub extern crate colored;
pub extern crate serde;
pub extern crate serde_json;

mod get_todo;
use get_todo::get_todo;

mod get_todo_all;
use get_todo_all::get_todo_all;

mod new_todo;
use new_todo::new_todo;

mod remove_todo;
use remove_todo::remove_todo;

mod tag_todo;
use tag_todo::tag_todo;

mod finish_todo;
use finish_todo::finish_todo;

fn main() {
    let args: Vec<String> = env::args().collect();

    match &*args[1] {
        "--help" | "-h" => print_help(),
        "--get" | "-g" => get_todo(args[2].parse().unwrap()),
        "--list" | "-l" => get_todo_all(),
        "--new" | "-n" => new_todo(args[2].clone()),
        "--clear" | "-c" => {fs::write("data.json", "[]").unwrap();println!("Todo list cleared.")},
        "--remove" | "-r" => remove_todo(args[2].parse().unwrap()),
        "--tag" | "-t" => tag_todo(args[2].parse().expect(&format!("Index '{}' invalid", args[2])), args[3].parse().unwrap()),
        "--done" | "-d" => finish_todo(args[2].parse().unwrap()),
        "--version" | "-v" => println!("rust-todo v{}", env!("CARGO_PKG_VERSION")),
        _ => println!("Command not found"),
    }
}

fn print_help() {
    println!("A todo-list CLI tool written in Rust");
    println!();
    println!("Usage:");
    println!("    rtodo [OPTIONS]");
    println!("    rtodo [OPTIONS] [ARGUMENTS]");
    println!();
    println!("Options:");
    println!("    --help, -h                     Prints this help message");
    println!("    --get, -g     [INDEX]          Prints todo info at index");
    println!("    --list, -l                     Prints all todos");
    println!("    --new, -n     [NAME]           Creates new todo with name");
    println!("    --clear, -c                    Clears all todos");
    println!("    --remove, -r  [INDEX]          Removes todo at index");
    println!("    --tag, -t     [INDEX] [COLOR]  Tags todo at index with color");
    println!("    --done, d     [INDEX]          Makes todo finished/unfinished");
    println!("    --version, -v                  Prints version info");
    println!();
    println!("Tag Colors:");
    println!("    {}", "red ⦿".red());
    println!("    {}", "blue ⦿".blue());
    println!("    {}", "yellow ⦿".yellow());
    println!("    {}", "green ⦿".green());
    println!("    {}", "cyan ⦿".cyan());
    println!("    {}", "purple, magenta ⦿".purple());
}
