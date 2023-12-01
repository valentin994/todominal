use::clap::{Parser, ValueEnum};
use rusqlite::{Connection, Result, Error};

//TODO Add a way to add a todo
//TODO A way to remove the todo
//TODO A way to check on which operating system I am and how to store the todo
//TODO A way to reorganize the todos
//TODO Add colors for priorities
//TODO Add a config to be able to change the colors, specify file location for the sqlite

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    operation: Operation,
    #[arg(short, long)]
    text: String,
    #[arg(short, long, default_value = "medium")]
    priority: String,
}
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Operation {
    Add,
    Remove,
    Modify
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

fn insert_todo(conn: Connection, todo_text: String, priority: String) -> Result<(), Error>{
    conn.execute("INSERT into todo (todo_text, priority) VALUES (?1, ?2)", (todo_text, priority))?;
    Ok(())
}

fn remove_todo(conn: Connection, todo_text: String, priority: String) -> Result<(), Error> {
    conn.execute("INSERT into todo (todo_text, priority) VALUES (?1, ?2)", (todo_text, priority))?;
    Ok(())
}

fn modify_todo(conn: Connection, todo_text: String, priority: String) -> Result<(), Error> {
    conn.execute("INSERT into todo (todo_text, priority) VALUES (?1, ?2)", (todo_text, priority))?;
    Ok(())
}

fn main() {
    let conn = create_table().expect("Failed to connect to DB");
    let args = Args::parse();
    match args.operation {
        add => {
            insert_todo(conn, args.text, args.priority);
        },
        remove => {
            remove_todo(conn, args.text, args.priority);
        },
        modify => {
            modify_todo(conn,args.text, args.priority);
        },
    }

}
