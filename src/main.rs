use std::io::{self, Write};
use std::path::Path;
use std::fs;

fn main() {
    let mut current_dir = Path::new("C:/");
    'exec_loop: loop {
        print!("User$ ");
        io::stdout().flush().unwrap();
        let mut cmd_buff = String::new();
        match io::stdin().read_line(&mut cmd_buff) {
            Ok(_) => {
                let cmd: Vec<&str> = cmd_buff.trim().split_whitespace().collect();

                match cmd.get(0).unwrap() {
                    &"echo" | &"Echo" => {
                        for word in cmd.iter().skip(1) {
                            print!("{} ", word);
                        }
                        println!();
                    },
                    &"ls" => {
                        let paths = fs::read_dir(current_dir).unwrap();
                        
                        for path in paths {
                            println!("> {}", path.unwrap().path().display());
                        }
                    },
                    &"mkdir" => {
                        let path_str = format!("{}/{}",current_dir.display(),cmd.get(1).unwrap());
                        let new_path = Path::new(&path_str);

                        match fs::create_dir(new_path) {
                            Ok(_) => (),
                            Err(_) => println!("Failed to create directory!"),
                        }
                    },
                    &"cd" => {

                    },
                    &"rmdir" => {

                    }
                    &"exit" | &"Exit" => break 'exec_loop,
                    _ => println!("Unsuported command!")
                }
            },
            Err(_) => println!("Invalid input!")
        }
    }
    println!("Successfully exited the program");
}
