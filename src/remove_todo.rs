use std::fs;
use colored::Colorize;

use crate::todo::TodoItem;

pub fn remove_todo(index: usize) {
    let data = fs::read_to_string("./data.json")
        .unwrap_or("[]".to_string());
    let mut todos: Vec<TodoItem> = serde_json::from_str(&data)
        .expect("data.json does not have correct format");

    print!("{}{}{} {}   ", todos[index].name.bold(), " ".repeat(50-todos[index].name.chars().count()),"ID:".bold(), index.to_string());
    for j in 0..todos[index].tags.len() {
        match todos[index].tags[j].as_str() {
            "red" => print!("{} ", "⦿".red()),
            "yellow" => print!("{} ", "⦿".yellow()),
            "green" => print!("{} ", "⦿".green()),
            "blue" => print!("{} ", "⦿".blue()),
            "cyan" => print!("{} ", "⦿".cyan()),
            "magenta" => print!("{} ", "⦿".magenta()),
            "purple" => print!("{} ", "⦿".purple()),
            _ => print!("?"),
        }
    }
    println!();
    
    todos.remove(index);
    let serialized = serde_json::to_string_pretty(&todos);
    fs::write("data.json", serialized.unwrap()).expect("Writing to JSON failed.");
}