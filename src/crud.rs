use rusqlite::{Connection, Error, Result};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct Todo {
    id: u16,
    todo_text: String,
    priority: String,
}

impl Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ID: {}\n○ {}\n  ->priority: {}\n",
            self.id, self.todo_text, self.priority
        )
    }
}

pub fn insert_todo(conn: Connection, todo_text: String, priority: String) -> Result<(), Error> {
    conn.execute(
        "INSERT into todo (todo_text, priority) VALUES (?1, ?2)",
        (todo_text, priority),
    )?;
    Ok(())
}

pub fn remove_todo(conn: Connection, id: i32) -> Result<(), Error> {
    conn.execute("DELETE from todo where id = ?", [&id])?;
    Ok(())
}

pub fn modify_todo(conn: Connection) -> Result<(), Error> {
    conn.prepare("DELETE from todo where id = VALUES (?1)")?;
    Ok(())
}

pub fn get_all_todos(conn: Connection) -> Result<(), Error> {
    let mut statement = conn.prepare("SELECT * FROM todo")?;
    let todo_iter = statement.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            todo_text: row.get(1)?,
            priority: row.get(2)?,
        })
    })?;
    for todo in todo_iter {
        println!("{}", todo.unwrap());
    }
    Ok(())
}