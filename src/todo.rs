use crate::database::Database;

#[derive(Debug, Clone)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
}

impl Todo {
    pub fn new(title: String, description: Option<String>, done: Option<bool>) -> Todo {
        Todo {
            id: None,
            title,
            description,
            done: done.unwrap_or(false),
        }
    }

    pub fn to_md(&self) -> String {
        format!(
            "# [{}] {}\n{}",
            if self.done { "X" } else { " " },
            self.title,
            self.description.as_ref().unwrap_or(&"".to_string())
        )
    }
}

pub struct TodoRepository {
    pub db: Database,
}

impl TodoRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub fn add(&self, todo: &Todo) {
        let sql =
            "INSERT INTO todos (title, description, done) VALUES (:title, :description, :done)";
        self.db
            .conn
            .execute(sql, (&todo.title, &todo.description, &todo.done))
            .unwrap();
    }

    pub fn get_all(&self) -> Vec<Todo> {
        let sql = "SELECT id, title, description, done FROM todos WHERE date(created_at) = strftime('%Y-%m-%d', 'now')";
        let mut stmt = self.db.conn.prepare(sql).unwrap();
        stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                done: row.get(3)?,
            })
        })
        .unwrap()
        .map(|todo| todo.unwrap())
        .collect::<Vec<Todo>>()
    }

    pub fn get_all_undone(&self) -> Vec<Todo> {
        let sql = "SELECT id, title, description, done FROM todos WHERE date(created_at) = strftime('%Y-%m-%d', 'now') AND done = 0";
        let mut stmt = self.db.conn.prepare(sql).unwrap();
        stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                done: row.get(3)?,
            })
        })
        .unwrap()
        .map(|todo| todo.unwrap())
        .collect::<Vec<Todo>>()
    }

    pub fn get_all_done(&self) -> Vec<Todo> {
        let sql = "SELECT id, title, description, done FROM todos WHERE date(created_at) = strftime('%Y-%m-%d', 'now') AND done = 1";
        let mut stmt = self.db.conn.prepare(sql).unwrap();
        stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                done: row.get(3)?,
            })
        })
        .unwrap()
        .map(|todo| todo.unwrap())
        .collect::<Vec<Todo>>()
    }

    pub fn remove_all(&self) {
        let sql = "DELETE FROM todos WHERE date(created_at) = strftime('%Y-%m-%d', 'now')";
        self.db
            .conn
            .execute(sql, [])
            .expect("Failed to remove all the todos");
    }

    pub fn done(&self, id: i32) {
        let sql = "UPDATE todos SET done = :done WHERE id = :id";
        self.db
            .conn
            .execute(sql, (&1, &id))
            .expect("Failed to update the todo");
    }

    pub fn undone(&self, id: i32) {
        let sql = "UPDATE todos SET done = :done WHERE id = :id";
        self.db
            .conn
            .execute(sql, (&0, &id))
            .expect("Failed to update the todo");
    }
}
