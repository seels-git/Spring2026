use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),            // Directory path
    Display(String),         // File path
    Create(String, String),  // File path and content
    Remove(String),          // File path
    Pwd,                     // Print working directory
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(dir_path) => {
            let status = Command::new("ls")
                .arg(&dir_path)
                .status()
                .expect("Failed to execute ls");
            if !status.success() {
                eprintln!("Error: Failed to list directory '{}'", dir_path);
            }
        }

        FileOperation::Display(file_path) => {
            let status = Command::new("cat")
                .arg(&file_path)
                .status()
                .expect("Failed to execute cat");
            if !status.success() {
                eprintln!("Error: Failed to display file '{}'", file_path);
            }
        }

        FileOperation::Create(file_path, content) => {
            let user_command = format!("echo '{}' > {}", content, file_path);
            let output = Command::new("sh")
                .arg("-c")
                .arg(&user_command)
                .output()
                .expect("Failed to execute command");

            if output.status.success() {
                println!("File '{}' created successfully.", file_path);
            } else {
                eprintln!("Error: Failed to create file '{}'", file_path);
            }
        }

        FileOperation::Remove(file_path) => {
            let status = Command::new("rm")
                .arg(&file_path)
                .status()
                .expect("Failed to execute rm");
            if status.success() {
                println!("File '{}' removed successfully.", file_path);
            } else {
                eprintln!("Error: Failed to remove file '{}'", file_path);
            }
        }

        FileOperation::Pwd => {
            print!("Current working directory: ");
            io::stdout().flush().unwrap();
            let status = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
            if !status.success() {
                eprintln!("Error: Failed to get working directory");
            }
        }
    }
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn print_menu() {
    println!("\nFile Operations Menu:");
    println!("1. List files in a directory");
    println!("2. Display file contents");
    println!("3. Create a new file");
    println!("4. Remove a file");
    println!("5. Print working directory");
    println!("0. Exit");
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        print_menu();
        let choice = read_line("\nEnter your choice (0-5): ");

        let operation = match choice.as_str() {
            "0" => {
                println!("\nGoodbye!");
                break;
            }
            "1" => {
                let dir = read_line("Enter directory path: ");
                FileOperation::List(dir)
            }
            "2" => {
                let path = read_line("Enter file path: ");
                FileOperation::Display(path)
            }
            "3" => {
                let path = read_line("Enter file path: ");
                let content = read_line("Enter content: ");
                FileOperation::Create(path, content)
            }
            "4" => {
                let path = read_line("Enter file path: ");
                FileOperation::Remove(path)
            }
            "5" => FileOperation::Pwd,
            _ => {
                println!("Invalid option '{}'. Please enter a number between 0 and 5.", choice);
                continue;
            }
        };

        println!();
        perform_operation(operation);
    }
}