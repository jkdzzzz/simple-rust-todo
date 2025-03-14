use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

pub fn task_menu(tasks: Vec<String>) -> String {
    let mut command = String::new();
    let command_list = ["n", "c", "q", "r"];

    for i in 0..tasks.len() {
        let task = &tasks[i];
        println!("[{i}] {task}");
    }

    print!("\n");
    println!("-- Command List --");

    for command in command_list {
        println!("{command}");
    }

    print!("$> ");
    io::stdout().flush().expect("Failed to flush stdout!");
    io::stdin().read_line(&mut command).expect("The input is invalid!");

    return command;
}

pub fn add_task(mut file: fs::File) {
    print!("$/new_task/> ");
    io::stdout().flush().expect("Failed to flush stdout!");

    let mut new_task = String::new();
    io::stdin().read_line(&mut new_task).expect("Invalid input!");
    new_task = new_task.trim().parse().expect("Invalid Task input!");

    if let Err(e) = writeln!(file, "{new_task}") {
        eprintln!("Error writing to file: {e}");
        process::exit(1);
    }
}

pub fn clear_file() {
    match fs::remove_file("list.txt") {
        Ok(()) => println!("Cleared the file!"),
        Err(e) => {
            eprintln!("Error clearing file: {e}");
            process::exit(1);
        }
    }
}

pub fn remove_task(mut file: fs::File, mut tasks: Vec<String>) -> io::Result<()> {
    let mut task = String::new();

    print!("$/remove_task/> ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut task).expect("Invalid input!");
    let task: usize = task.trim().parse().expect("Not a valid ID!");

    let task_line = &tasks.clone()[task];
    tasks.remove(task);

    let reader = io::BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(|line| {
        let line = line.ok()?;
        if &line != task_line {
            Some(line)
        } else {
            None
        }
    });

    Ok(())
}

pub fn handle_command(command: String, file: fs::File, tasks: Vec<String>) {
    match command.trim() {
        "n" => add_task(file),
        "q" => {
            println!("Quitting...");
            process::exit(0);
        },
        "r" => {
            remove_task(file, tasks);
        }
        "c" => clear_file(),
        _ => {
            println!("Invalid command!");
            return;
        }
    }
}