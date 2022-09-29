use std::fs;
use colored::Colorize;

use crate::todo::TodoItem;

pub fn get_todo(index: usize) {
    let data = fs::read_to_string("./data.json")
        .unwrap_or("[]".to_string());
    let todos: Vec<TodoItem> = serde_json::from_str(&data)
        .expect("data.json does not have correct format");

    if todos.get(index).is_none() {
        println!("Todo with index '{}' not found", index);
        return;
    };

    let checkbox: char = match todos[index].done {
        true => '✅',
        false => ' ',
    };

    println!("{} {}", checkbox, todos[index].name.as_str().bold());
    println!();
    println!("{} {}", "ID: ".bold(), index);
    println!("{}", "Tags:".bold());

    for i in 0..todos[index].tags.len() {
        match todos[index].tags[i].as_str() {
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