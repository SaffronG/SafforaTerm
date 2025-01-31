use std::io::{self, Read, Write};
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
                    // Helper Functions
                    &"clear" | &"cls" => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),
                    &"help" => println!("\nCommands: clear: clears the screen,\n help: shows this menu,\n echo: prints the input string,\n ls: displays the relative path of the current directory,\n als: displays the actual path of the current directory,\n pwd,\n mov or rname: renames a file or directory or moves a file or directory,\n mkdir: creates a new folder,\n cd: changes directory,\n rmdir: removes a directiory,\n grep: finds a line containing the input string,\n wc or words: return the wordcount of the file,\n exit: exit the terminal\n"),
                    &"echo" | &"Echo" => {
                        for word in cmd.iter().skip(1) {
                            print!("{} ", word);
                        }
                        println!();
                    },
                    // Directory Commands
                    &"ls" => {
                        let paths = fs::read_dir(current_dir.clone()).unwrap();
                        for path in paths {
                            let path = path.unwrap().path();
                            if path.is_dir() {
                                print!("D ");
                            } else {
                                print!("F ");
                            }
                            let mut path_str = path.display().to_string();
                            path_str.replace_range(0..current_dir.display().to_string().len(), "");
                            println!("> ~{}", path_str);
                        }
                    },
                    &"als" | &"la" => {
                        let paths = fs::read_dir(current_dir.clone()).unwrap();
                        for path in paths {
                            let path = path.unwrap().path();
                            if path.is_dir() {
                                print!("D ");
                            } else {
                                print!("F ");
                            }
                            println!("> {}", path.display());
                        }
                    },
                    &"pwd" | &"dir" => println!("{}", current_dir.display()),
                    &"mov" | &"rname" => {
                        let src_str = format!("{}/{}", current_dir.display(), cmd.get(1).unwrap());
                        let dest_str = format!("{}/{}", current_dir.display(), cmd.get(2).unwrap());
                        let src_path = Path::new(&src_str);
                        let dest_path = Path::new(&dest_str);
                        match fs::rename(src_path, dest_path) {
                            Ok(_) => (),
                            Err(_) => println!("Failed to move file!"),
                        }
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
                    // File Commands
                    &"grep" => {
                        let path_str = format!("{}/{}",current_dir.display(),cmd.get(1).unwrap());
                        let new_path = Path::new(&path_str);
                        let mut file: String = String::new();
                        match fs::File::open(new_path).unwrap().read_to_string(&mut file) {
                            Ok(_) => (),
                            Err(_) => println!("Failed to open file!"),
                        }
                        let mut line_no: i32 = 0;
                        for line in file.lines() {
                            line_no += 1;
                            match cmd.get(2) {
                                Some(result) => {
                                    if line.contains(result) {
                                        println!("{}: {}", line_no, line);
                                    }
                                }
                                _ => println!("Failed to grep file!"),
                            }
                        }
                    },
                    &"wc" | &"words" => {
                        let path_str = format!("{}/{}",current_dir.display(),cmd.get(1).unwrap());
                        let new_path = Path::new(&path_str);
                        let mut file: String = String::new();
                        match fs::File::open(new_path).unwrap().read_to_string(&mut file) {
                            Ok(_) => {
                                let words: Vec<&str> = file.split_whitespace().collect();
                                println!("Word Count: {}", words.len());
                            },
                            Err(_) => println!("Failed to open file!"),
                        }
                    },
                    // Misc Commands
                    &"exit" | &"Exit" => break 'exec_loop,
                    _ => println!("Unsupported command!")
                }
            },
            Err(_) => println!("Invalid input!")
        }
    }
    println!("Successfully exited the program");
}
