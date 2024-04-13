use std::fs::{self, File, OpenOptions};
use std::io::Write;

use regex::Regex;

use crate::todo::Todo;

pub struct TodoMarkdown {
    pub filepath: String,
    pub todos: Vec<Todo>,
}

const DIVIDER: &str = "\n\n<!---->\n\n";
const FILEPATH: &str = "/tmp/";

impl TodoMarkdown {
    pub fn save(filename: &str, todos: &Vec<Todo>) -> Self {
        let filepath = format!("{}{}", FILEPATH, filename);
        let mut file = TodoMarkdown::open_file(&filepath, true);
        let mut content = String::new();
        for (i, todo) in todos.iter().enumerate() {
            content.push_str(&todo.to_md());
            if i != todos.len() - 1 {
                content.push_str(DIVIDER);
            }
        }
        file.write(content.as_bytes())
            .expect("Failed to save the TODO");
        Self {
            filepath: filepath.to_string(),
            todos: todos.clone(),
        }
    }

    pub fn remove_file(&self) {
        std::fs::remove_file(&self.filepath).expect("Failed to remove the TODO file");
    }

    pub fn save_edit(&mut self) {
        let todo_md = fs::read_to_string(&self.filepath).unwrap();
        let edited_todos = todo_md.split(DIVIDER).collect::<Vec<&str>>();
        self.todos.clear();
        for edited_todo in edited_todos {
            let segments = edited_todo
                .split("\n")
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();

            let is_done_txt: &str;
            let title: &str;
            let description: &str;

            let full_title = segments[0];
            let re = Regex::new(r"#\s*\[([^\]]*)\]\s*(.*)").unwrap();
            if let Some(captures) = re.captures(full_title) {
                is_done_txt = captures.get(1).map_or("", |m| m.as_str());
                title = captures.get(2).map_or("", |m| m.as_str());
            } else {
                panic!("Invalid TODO: {}", full_title);
            }

            let description_segments_joined = &segments[1..].join("\n");
            description = &description_segments_joined[..].trim();

            let todo = Todo::new(
                title.to_string(),
                if description.is_empty() {
                    None
                } else {
                    Some(description.to_string())
                },
                Some(is_done_txt == "X"),
            );
            self.todos.push(todo);
        }
    }

    fn open_file(filepath: &str, create: bool) -> File {
        match OpenOptions::new()
            .append(true)
            .create(create)
            .open(filepath)
        {
            Ok(v) => v,
            Err(e) => panic!("Failed to open the temporary file {}", e),
        }
    }
}
