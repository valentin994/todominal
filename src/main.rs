use ::clap::{Parser, ValueEnum};
use rusqlite::{params, Connection, Error, Result};
use std::fmt;
use std::fmt::Display;

//TODO A way to update the todo
//TODO Change code to use Subcommands
//TODO A way to check on which operating system I am and how to store the todo
//TODO A way to reorganize the todos
//TODO Add colors for priorities
//TODO Add a config to be able to change the colors, specify file location for the sqlite
//TODO implement ratatui into this

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    operation: Operation,
    #[arg(short, long, default_value = " ")]
    text: String,
    #[arg(short, long, default_value = "medium")]
    priority: String,
    #[arg(short, long, default_value = None)]
    id: Option<i32>,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operation {
    Add,
    Remove,
    Modify,
    Ls,
}

fn create_table() -> Result<(Connection), Error> {
    let path = "./my_db.db3";
    let db = Connection::open(path)?;
    // Use the database somehow...
    /* db.execute(
        "CREATE TABLE todo (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            todo_text  TEXT NOT NULL,
            priority  TEXT
        )",
        (), // empty list of parameters.
    )?;*/

    Ok(db)
}

fn insert_todo(conn: Connection, todo_text: String, priority: String) -> Result<(), Error> {
    conn.execute(
        "INSERT into todo (todo_text, priority) VALUES (?1, ?2)",
        (todo_text, priority),
    )?;
    Ok(())
}

fn remove_todo(conn: Connection, id: i32) -> Result<(), Error> {
    conn.execute("DELETE from todo where id = ?", ([&id]))?;
    Ok(())
}

fn modify_todo(conn: Connection, todo_text: String, priority: String) -> Result<(), Error> {
    let mut stms = conn.prepare("DELETE from todo where id = VALUES (?1)")?;
    Ok(())
}

fn get_all_todos(conn: Connection) -> Result<(), Error> {
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
            "○ {} \t\tID: {} \n  ->priority: {}\n",
            self.todo_text, self.id, self.priority
        )
    }
}

fn main() {
    let conn = create_table().expect("Failed to connect to DB");
    let args = Args::parse();
    match args.operation {
        Operation::Add => {
            insert_todo(conn, args.text, args.priority);
        }
        Operation::Remove => {
            match remove_todo(conn, args.id.unwrap()) {
                Ok(()) => (),
                Err(err) => println!("{err}"),
            };
        }
        Operation::Modify => {
            modify_todo(conn, args.text, args.priority);
        }
        Operation::Ls => {
            get_all_todos(conn);
        }
    }
}
