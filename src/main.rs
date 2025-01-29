use std::io::{self, Write};
use std::path::Path;
use std::fs;

fn main() {
    let mut current_dir = Path::new("C:/").to_path_buf();
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
                        let paths = fs::read_dir(current_dir.clone()).unwrap();
                        for path in paths {
                            println!("> {}", path.unwrap().path().display());
                        }
                    },
                    &"mov" => {
                        unimplemented!("Unimplemented code error!");              
                    },
                    &"mkdir" => {
                        let path_str = format!("{}/{}", current_dir.display(), cmd.get(1).unwrap());
                        let new_path = Path::new(path_str.as_str());
                        match fs::create_dir(new_path) {
                            Ok(_) => (),
                            Err(_) => println!("Failed to create directory!"),
                        }
                    },
                    &"cd" => {
                        let path_str = format!("{}/{}", current_dir.clone().display(), cmd.get(1).unwrap());
                        let new_path = Path::new(&path_str);
                        if new_path.is_dir() {
                            current_dir = new_path.to_path_buf();
                        } else {
                            println!("Invalid directory!")
                        }
                    },
                    &"rmdir" => {
                        let path_str = format!("{}/{}",current_dir.display(),cmd.get(1).unwrap());
                        let new_path = Path::new(&path_str);
                        match fs::remove_dir(new_path) {
                            Ok(_) => (),
                            Err(_) => println!("Failed to remove directory!"),
                        }
                    },
                    &"exit" | &"Exit" => break 'exec_loop,
                    _ => println!("Unsupported command!")
                }
            },
            Err(_) => println!("Invalid input!")
        }
    }
    println!("Successfully exited the program");
}
