# Todominal

The goal of this project is to create a todo application for the terminal.
The application uses sqlite for storing the data.
![render1702041883355.gif](assets%2Frender1702041883355.gif)

Location for the database:
- Windows: `C:\Users\username\AppData\Roaming\todominal\config\todominal.db3`
- Linux: `/home/user/.config/todominal/todominal.db3` 

To build and run the application
- ` cargo install --path . && todominal`

## Operations
### Help
Running 
- `todominal --help`

Will provide you with
```Usage: todominal.exe <COMMAND>

Commands:
add     Add a new todo to the database
remove  Remove a todo from the database
modify  Modify a todo from the database
list    List all todos
help    Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
```
You can run help on any subcommand as well.
- `todominal add --help`
```
Usage: todominal.exe add [OPTIONS] <TEXT>

Arguments:
  <TEXT>  The todo text

Options:
  -p, --priority <PRIORITY>  The priority of the todo [default: medium]
  -h, --help                 Print help

```
### Add
This adds a todo with a provided text to the sqlite database. 
The priority is set to medium by but you can provide any arbitrary value to that flag. 

- `todominal add My new todo --priority "high"`
- `todominal a My new todo --priority "high"`
### Modify
Modify a Todo by providing its ID.
- `todominal modify 1 Modified text`
- `todominal md 1 Modified text`
### Remove
Delete a Todo by providing the ID.
- `todominal remove 1`
- `todominal rm 1`
### List
List all todos. Supports priority filter.
- `todominal list --priority "medium`
- `todominal ls`
