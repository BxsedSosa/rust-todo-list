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

fn menu_selection() {
    clear_console();
    println!("Make a selection:\n1) Add Todo\n2) Edit Todo\n3) Delete Todo\n4) Exit Program")
}

fn main() {
    welcome();
    menu_selection();
}
