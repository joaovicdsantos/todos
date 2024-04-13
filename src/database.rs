use log::info;
use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(filename: &str) -> Self {
        let full_path = Self::create_database_folder();
        let database_name = format!("{}{}", full_path, filename);
        let connection = match Connection::open(database_name) {
            Ok(v) => v,
            Err(e) => panic!("Could not open the todos database! {}", e),
        };
        Self { conn: connection }
    }

    pub fn test_connection(&self) {
        match self
            .conn
            .query_row("SELECT 1", (), |row| row.get::<usize, u8>(0))
        {
            Ok(_) => info!("The connection test was successful!"),
            Err(e) => panic!("The connection test was failed! {}", e),
        }
    }

    pub fn create_basic_schema(&self) {
        match self.conn.execute(DB_SCHEMA, ()) {
            Ok(_) => info!("The basic schema was created!"),
            Err(_) => panic!("The basic schema was not created!"),
        }
    }

    fn create_database_folder() -> String {
        let home_path = std::env::var("HOME").unwrap();
        let full_path = format!("{}/.cache/todos/", home_path);
        match std::fs::create_dir_all(&full_path) {
            Ok(_) => info!("The database folder was created!"),
            Err(_) => panic!("The database folder was not created!"),
        }
        full_path
    }
}

const DB_SCHEMA: &str = include_str!("static/schema.sql");
