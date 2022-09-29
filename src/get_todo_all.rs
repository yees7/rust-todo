use std::fs;
use colored::Colorize;

use crate::todo::TodoItem;

pub fn get_todo_all() {
    let data = fs::read_to_string("./data.json")
        .unwrap_or("[]".to_string());
    let todos: Vec<TodoItem> = serde_json::from_str(&data)
        .expect("data.json does not have correct format");

    for i in 0..todos.len() {
        let checkbox: char = match todos[i].done {
            true => '✅',
            false => ' ',
        };

        print!("{} {}{}{} {}   ", checkbox, todos[i].name.bold(), " ".repeat(50-todos[i].name.chars().count()),"ID:".bold(), i.to_string());
        for j in 0..todos[i].tags.len() {
            match todos[i].tags[j].as_str() {
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
    println!();
}