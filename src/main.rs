use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;

const PATH: &str = "todos.txt";

fn clear_console() {
    print!("\x1B[2J")
}

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}

fn clear_delay_message(message: &str, secs: u64) {
    clear_console();
    println!("{}", message);
    sleep(secs);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_file_todos() -> Vec<String> {
    let mut file_todos: Vec<String> = Vec::new();

    if let Ok(lines) = read_lines(PATH) {
        for line in lines.map_while(Result::ok) {
            file_todos.push(line);
        }
    }

    file_todos
}

fn write_file_todos(todos: String) {
    let _ = fs::write(PATH, todos);
}

fn join_todos(todos: &[String]) -> String {
    let mut file_todo_string = String::new();
    for (i, todo) in todos.iter().enumerate() {
        if i + 1 == todos.len() {
            file_todo_string.push_str(todo);
        } else {
            file_todo_string.push_str(todo);
            file_todo_string.push('\n');
        }
    }

    file_todo_string
}

fn no_todos() {
    clear_delay_message("There is no todos!", 2);
}

fn menu_selection() -> String {
    let mut user_input = String::new();
    clear_console();
    println!("Make a selection:\n1) Add Todo\n2) Edit Todo\n3) Delete Todo\n4) Display Todos\n5) Exit Program");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Did not read line");

    user_input.trim().to_lowercase()
}

fn get_todo(first_string: &str) -> String {
    let mut user_input = String::new();

    loop {
        clear_console();
        println!("Enter todo:");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                println!("{}", first_string);
                sleep(1);
                return user_input.trim().to_string();
            }
            Err(_) => continue,
        }
    }
}

fn get_valid_idx(todos: &[String], option: &str) -> usize {
    loop {
        let mut user_input = String::new();

        display_todos(todos, true, false);
        println!("Which Todo do you want to {}?:", option);

        io::stdin().read_line(&mut user_input).expect("Cooked");

        match user_input.trim().parse::<usize>() {
            Ok(num) => {
                if num == 0 || num > todos.len() {
                    clear_console();
                    println!("{} is not a valid selection!", num);
                    sleep(2);
                    continue;
                } else {
                    return num - 1;
                }
            }
            Err(_) => continue,
        }
    }
}

fn add_todo(todos: &mut Vec<String>) {
    let new_todo = get_todo("Added new todo!");
    todos.push(new_todo);
}

fn display_todos(todos: &[String], clear: bool, delay: bool) {
    if todos.is_empty() {
        no_todos();
        return;
    }
    if clear {
        clear_console();
    }
    for (i, todo) in todos.iter().enumerate() {
        println!("{}) {}", i + 1, todo);
    }
    if delay {
        sleep(5);
    }
}

fn edit_todo(todos: &mut [String]) {
    if todos.is_empty() {
        no_todos();
        return;
    }

    let idx = get_valid_idx(todos, "edit");
    let edit_todo = get_todo("Edit Complete!");
    todos[idx] = edit_todo;
}

fn delete_todo(todos: &mut Vec<String>) {
    if todos.is_empty() {
        no_todos();
        return;
    }

    let idx = get_valid_idx(todos, "delete");
    todos.remove(idx);
}

fn program_loop() {
    let mut todos: Vec<String> = get_file_todos();

    loop {
        let user_input = menu_selection();

        match user_input.as_str() {
            "1" | "add" | "a" => add_todo(&mut todos),
            "2" | "edit" | "e" => edit_todo(&mut todos),
            "3" | "delete" | "del" => delete_todo(&mut todos),
            "4" | "display" | "dis" => display_todos(&todos, true, true),
            "5" => {
                write_file_todos(join_todos(&todos));
                clear_console();
                break;
            }
            _ => {
                clear_delay_message("You entered \" {} \"\nThis is not a valid option!", 2);
                continue;
            }
        }
    }
}

fn main() {
    clear_delay_message("Welcome to Todo list!", 2);
    program_loop();
}
