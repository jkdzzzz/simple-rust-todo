use std::fs::OpenOptions;
use std::io::{self, Write, BufRead};
use std::process;

fn main() {
    let mut new_task = String::new();
    let path = "list.txt";
    let mut file = match OpenOptions::new().append(true).write(true).read(true).create(true).open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open / create {path}: {e}");
            process::exit(1);
        }
    };

    let reader = io::BufReader::new(&file);
    let mut tasks: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(l) => tasks.push(l),
            Err(e) => {
                eprintln!("Error reading file: {e}");
                break;
            },
        }
    }

    println!("Simple Rust ToDo-List");
    
    for task in tasks {
        println!("{task}");
    }

    print!("New Task: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin().read_line(&mut new_task).expect("The input is invalid!");

    new_task = new_task.trim().parse().expect("The input is not a valid string!");

    if let Err(e) = writeln!(file, "{new_task}") {
        eprintln!("Error writing to file: {e}");
        process::exit(1);
    }

    println!("Successfully written Task to {path}");
}
