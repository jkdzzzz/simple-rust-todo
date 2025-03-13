use std::fs::File;
use std::io::{self, Write};
use std::process;

fn main() {
    let mut new_task = String::new();
    let mut file = match File::create("output.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating the file: {e}");
            process::exit(1);
        }
    };

    println!("Simple Rust ToDo-List");
    print!("New Task: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin().read_line(&mut new_task).expect("The input is invalid!");

    if let Err(e) = file.write_all(new_task.as_bytes()) {
        eprintln!("Error writing to file: {e}");
        process::exit(1);
    }

    println!("Successfully written Task to output.txt");
}
