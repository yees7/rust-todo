use std::{env, fs};

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
    // let path = Path::new("data.json");

    // if !path.exists() {
    //     File::create(&path).unwrap();
    // }

    let args: Vec<String> = env::args().collect();

    match &*args[1] {
        "get" => get_todo(args[2].parse().unwrap()),
        "list" => get_todo_all(),
        "new" => new_todo(args[2].clone()),
        "clear" => {fs::write("data.json", "[]").unwrap();println!("Todo list cleared.")},
        "remove" => remove_todo(args[2].parse().unwrap()),
        "tag" => tag_todo(args[2].parse().expect(&format!("Index '{}' invalid", args[2])), args[3].parse().unwrap()),
        "done" => finish_todo(args[2].parse().unwrap()),
        _ => println!("Command not found"),
    }
}