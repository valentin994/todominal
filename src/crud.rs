use chrono::NaiveDate;
use rusqlite::{Connection, Error, Result};
use std::fmt;
use std::fmt::Display;

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

pub fn insert_todo(
    conn: Connection,
    todo_text: Option<String>,
    priority: String,
) -> Result<(), Error> {
    let text = match todo_text {
        Some(text) => text,
        None => panic!("Please provide a valid string"),
    };
    conn.execute(
        "INSERT into todo (todo_text, priority, date) VALUES (?1, ?2, DATE('now'))",
        (text, priority),
    )?;
    Ok(())
}

pub fn remove_todo(conn: Connection, id: Option<i32>) -> Result<(), Error> {
    let id = match id {
        Some(id) => id,
        None => panic!("Please provide a valid ID value"),
    };
    conn.execute("DELETE from todo where id = ?", [id])?;
    Ok(())
}

pub fn modify_todo(conn: Connection, id: Option<i32>, text: Option<String>) -> Result<(), Error> {
    let id = match id {
        Some(id) => id,
        None => panic!("ID does not exist"),
    };
    let text = match text {
        Some(text) => text,
        None => panic!("Please provide valid text"),
    };
    conn.execute(
        "UPDATE todo \
                      set todo_text = ? \
                      WHERE id = ?;",
        (text, id),
    )?;
    Ok(())
}

pub fn get_all_todos(conn: Connection) -> Result<(), Error> {
    let mut statement = conn.prepare("SELECT * FROM todo")?;
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
