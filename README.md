# To-Do List CLI

Concepts Covered: Vectors, strings, enums, basic file I/O (std::fs)

What You’ll Build: A command-line application that lets users add tasks, list all tasks, and mark them as complete. Tasks persist between sessions using a simple text file.

This introduces you to Rust’s owned types (String vs &str), working with collections, and basic file operations. You’ll define your first custom types and learn how data ownership works when passing values between functions.

Tip: Start with an in-memory implementation, then add file persistence.

## Brainstorm

- [x] Need a menu:
  - [x] Add a todo
  - [x] Delete exisiting
  - [x] Edit exisiting
  - [x] Safe exit out of program

After base todo works on memory, then start working on implementing file read/write to save the todos

- [x] read lines from file
  - [x] add each line to a vector of todos
- [ ] On exit overwrite exisiting file of todos
