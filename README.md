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
### Add
This adds a todo with a provided text to the sqlite database. 
The priority is set to medium by but you can provide any arbitrary value to that flag. 
`todominal add "My new todo --priority "high"`