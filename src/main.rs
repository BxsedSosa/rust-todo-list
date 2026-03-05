use std::{env::join_paths, io};

fn clear_console() {
    print!("\x1B[2J")
}

fn sleep(secs: u64) {
    std::thread::sleep(std::time::Duration::from_secs(secs));
}

fn welcome() {
    clear_console();
    println!("Welcome to Todo List");
    sleep(2);
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

fn get_new_todo() -> String {
    let mut user_input = String::new();

    loop {
        clear_console();
        println!("Enter in new todo:");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                println!("New Todo added!");
                sleep(1);
                return user_input.trim().to_string();
            }
            Err(_) => continue,
        }
    }
}

fn add_todo(todos: &mut Vec<String>) {
    let new_todo = get_new_todo();
    todos.push(new_todo);
}

fn display_todos(todos: &Vec<String>) {
    for todo in todos {
        println!("{}", todo);
    }
    sleep(5);
}

fn edit_todo() {
    println!("edit");
    sleep(1);
}
fn delete_todo() {
    println!("delete");
    sleep(1);
}

fn program_loop() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        let user_input = menu_selection();

        match user_input.as_str() {
            "1" | "add" | "a" => add_todo(&mut todos),
            "2" | "edit" | "e" => edit_todo(),
            "3" | "delete" | "del" => delete_todo(),
            "4" | "display" | "dis" => display_todos(&todos),
            "5" => {
                clear_console();
                break;
            }
            _ => {
                clear_console();
                println!(
                    "You entered \" {} \"\nThis is not a valid option!",
                    user_input
                );
                sleep(2);
                continue;
            }
        }
    }
}

fn main() {
    welcome();
    program_loop();
}
