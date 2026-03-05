use std::io;

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
    println!("Make a selection:\n1) Add Todo\n2) Edit Todo\n3) Delete Todo\n4) Exit Program");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Did not read line");

    user_input.trim().to_lowercase()
}

fn add_todo() {
    println!("add");
    sleep(1);
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
    loop {
        let user_input = menu_selection();

        match user_input.as_str() {
            "1" => add_todo(),
            "2" => edit_todo(),
            "3" => delete_todo(),
            "4" => break,
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
