use std::fs;
use colored::Colorize;

use crate::todo::TodoItem;

pub fn tag_todo(index: usize, tag: String) {
    let data = fs::read_to_string("./data.json")
        .unwrap_or("[]".to_string());
    let mut todos: Vec<TodoItem> = serde_json::from_str(&data)
        .expect("data.json does not have correct format");

    if todos.get(index).is_none() {
        println!("Todo with index '{}' not found", index);
        return;
    };

    if todos[index].tags.contains(&tag) {
        todos[index].tags.retain(|i| i != &tag);
    } else {
        todos[index].tags.push(tag);
    }
    
    let serialized = serde_json::to_string_pretty(&todos);
    fs::write("data.json", serialized.unwrap()).expect("Writing to JSON failed.");

    let checkbox: char = match todos[index].done {
        true => '✅',
        false => ' ',
    };

    print!("{} {}{}{} {}   ", checkbox, todos[index].name.bold(), " ".repeat(50-todos[index].name.chars().count()),"ID:".bold(), index.to_string());
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
}