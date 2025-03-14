use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::process;

mod utils;

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let path = "list.txt";

    loop {
        let file = match OpenOptions::new().append(true).write(true).read(true).create(true).open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open / create {path}: {e}");
                process::exit(1);
            }
        };

        let mut tasks: Vec<String> = Vec::new();
        let reader = io::BufReader::new(&file);
        let lines = reader.lines();
        for line in lines {
            match line {
                Ok(l) => tasks.push(l),
                Err(e) => {
                    eprintln!("Error reading file: {e}");
                    break;
                },
            }
        }

        clear_console();
        println!("Simple Rust ToDo-List");

        let command = utils::task_menu(tasks);
        utils::handle_command(command, file);
    }
}
