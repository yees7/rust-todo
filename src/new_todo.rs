use std::fs;
use colored::Colorize;

use crate::todo::TodoItem;

pub fn new_todo(name: String) {
    let data = fs::read_to_string("./data.json")
        .unwrap_or("[]".to_string());
    let mut todos: Vec<TodoItem> = serde_json::from_str(&data)
        .expect("data.json does not have correct format");
    todos.push(TodoItem {
        done: false,
        name: name,
        tags: vec![],
    });
    let serialized = serde_json::to_string_pretty(&todos);
    fs::write("data.json", serialized.unwrap()).expect("Writing to JSON failed.");

    print!("{}{}{} {}   ", todos[todos.len()-1].name.bold(), " ".repeat(50-todos[todos.len()-1].name.chars().count()),"ID:".bold(), (todos.len()-1).to_string());
    println!();
}