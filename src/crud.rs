use chrono::NaiveDate;
use directories::ProjectDirs;
use rusqlite::{Connection, Error, Result};
use std::fmt;
use std::fmt::Display;
use std::fs;
use std::path::Path;
#[derive(Debug)]
struct Todo {
    id: u16,
    todo_text: String,
    priority: String,
    date: String,
}

impl Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {id}\n○ {text:<30} {date:>30}\n  ->priority: {priority}\n",
            id = self.id,
            text = self.todo_text,
            date = NaiveDate::parse_from_str(&self.date, "%Y-%m-%d")
                .unwrap()
                .format("%d-%m-%Y")
                .to_string(),
            priority = self.priority,
        )
    }
}

pub fn insert_todo(conn: Connection, todo_text: String, priority: String) -> Result<String, Error> {
    conn.execute(
        "INSERT into todo (todo_text, priority, date) VALUES (?1, ?2, DATE('now'))",
        (&todo_text, priority),
    )?;
    Ok(String::from(format!("{todo_text}")))
}

pub fn remove_todo(conn: Connection, id: i32) -> Result<String, Error> {
    conn.execute("DELETE from todo where id = ?", [id])?;
    Ok(String::from("Done"))
}

pub fn modify_todo(conn: Connection, id: i32, text: String) -> Result<String, Error> {
    conn.execute(
        "UPDATE todo \
                      set todo_text = ? \
                      WHERE id = ?;",
        (&text, &id),
    )?;
    Ok(String::from(format!("Changed Todo: {id}, to be: {text}")))
}

pub fn get_all_todos(conn: Connection, filter: Option<String>) -> Result<(), Error> {
    let filter_query = filter
        .map(|i| format!(" WHERE priority = '{}'", i))
        .unwrap_or_default();
    let query = format!(
        "SELECT id, todo_text, priority, date FROM todo{}",
        filter_query
    );
    let mut statement = conn.prepare(&query)?;
    let todo_iter = statement.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            todo_text: row.get(1)?,
            priority: row.get(2)?,
            date: row.get(3)?,
        })
    })?;
    for todo in todo_iter {
        println!("{}", todo.unwrap());
    }
    Ok(())
}

pub fn create_table() -> Result<Connection, rusqlite::Error> {
    let proj_dirs = ProjectDirs::from("", "", "todominal");
    let path = proj_dirs
        .clone()
        .unwrap()
        .config_dir()
        .join(Path::new("todominal.db3"));
    return if path.exists() {
        Ok(Connection::open(path)?)
    } else {
        let _ = fs::create_dir_all(proj_dirs.clone().unwrap().config_dir());
        let db = Connection::open(path)?;
        let _ = db.execute(
            "CREATE TABLE todo (
         id INTEGER PRIMARY KEY AUTOINCREMENT,
         todo_text  TEXT NOT NULL,
         priority  TEXT,
         date      TEXT
     )",
            (),
        )?;
        return Ok(db);
    };
}
