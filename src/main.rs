use std::{
    process::{self, Command},
    str::FromStr,
};

use database::Database;
use strum::{Display, EnumString};
use todo::TodoRepository;
use todo_markdown::TodoMarkdown;

use crate::todo::Todo;

mod args;
mod database;
mod todo;
mod todo_markdown;

extern crate pretty_env_logger;

extern crate log;

fn main() {
    pretty_env_logger::init();

    let db = Database::new("todos.db");
    db.test_connection();
    db.create_basic_schema();

    let todo_repository = TodoRepository::new(db);

    let args = args::load_args();
    let action: Action;
    match Action::from_str(&args.action) {
        Ok(v) => action = v,
        Err(_) => {
            eprintln!("\"{}\" is not a valid action!", args.action);
            eprintln!(
                "Valid actions: {}, {}, {}, {}, {} and {}",
                Action::Add,
                Action::Done,
                Action::Undone,
                Action::List,
                Action::Edit,
                Action::Help
            );
            process::exit(1)
        }
    }

    match action {
        Action::Add => add_todo(&args.params, &todo_repository),
        Action::Done => done_todo(&args.params, &todo_repository),
        Action::Undone => undone_todo(&args.params, &todo_repository),
        Action::List => list_todo(&args.params, &todo_repository),
        Action::Edit => edit_todo(&args.params, &todo_repository),
        _ => unimplemented!("This action was not implemented yet"),
    }
}

fn add_todo(params: &Vec<String>, todo_repository: &TodoRepository) {
    if params.len() < 1 {
        eprintln!("Usage: todos add <title> <description?>");
        process::exit(1)
    }
    let title = params[0].to_string();
    let description = match params.get(1) {
        Some(v) => Some(v.to_string()),
        None => None,
    };

    let todo = Todo::new(title, description, None);
    todo_repository.add(&todo);
    println!("{}", todo.to_md());
}

fn list_todo(_params: &Vec<String>, todo_repository: &TodoRepository) {
    let todos = todo_repository.get_all();
    let todo_markdown = TodoMarkdown::save("todos.md", &todos);

    Command::new("less")
        .arg(todo_markdown.filepath.as_str())
        .status()
        .expect("Failed to open the todo list");

    todo_markdown.remove_file();
}

fn done_todo(params: &Vec<String>, todo_repository: &TodoRepository) {
    if params.len() < 1 {
        let todos = todo_repository.get_all_undone();
        if todos.is_empty() {
            eprintln!("No TODO created yet");
            process::exit(0)
        }

        println!("Here are your TODOs:");
        for todo in todos {
            println!("{}: {}", todo.id.unwrap(), todo.title);
        }

        println!("Which TODO do you want to mark as done?");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let id = input.trim().parse::<i32>().unwrap();
        todo_repository.done(id);
    } else {
        let id = params[0].parse::<i32>().unwrap();
        todo_repository.done(id);
    }
}

fn undone_todo(params: &Vec<String>, todo_repository: &TodoRepository) {
    if params.len() < 1 {
        let todos = todo_repository.get_all_done();
        if todos.is_empty() {
            eprintln!("No TODO created yet");
            process::exit(0)
        }

        println!("Here are your finished TODOs:");
        for todo in todos {
            println!("{}: {}", todo.id.unwrap(), todo.title);
        }

        println!("Which TODO do you want to mark as undone?");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let id = input.trim().parse::<i32>().unwrap();
        todo_repository.undone(id);
    } else {
        let id = params[0].parse::<i32>().unwrap();
        todo_repository.undone(id);
    }
}

fn edit_todo(_params: &Vec<String>, todo_repository: &TodoRepository) {
    let todos = todo_repository.get_all();
    let mut todo_markdown = TodoMarkdown::save("todos.md", &todos);

    Command::new("nvim")
        .arg(todo_markdown.filepath.as_str())
        .status()
        .expect("Failed to open the todo list");

    todo_markdown.save_edit();
    todo_repository.remove_all();
    for todo in &todo_markdown.todos {
        todo_repository.add(&todo);
    }
    todo_markdown.remove_file();
}

#[derive(EnumString, Display)]
#[strum(ascii_case_insensitive)]
enum Action {
    #[strum(to_string = "add")]
    Add,

    #[strum(to_string = "done")]
    Done,

    #[strum(to_string = "undone")]
    Undone,

    #[strum(to_string = "list")]
    List,

    #[strum(to_string = "edit")]
    Edit,

    #[strum(to_string = "help")]
    Help,
}
