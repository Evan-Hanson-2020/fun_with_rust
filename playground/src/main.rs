use dirs;
use rand::prelude::*;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
struct User {
    user: String,
    userid: i32,
    password: String,
}

fn user_dir() -> PathBuf {
    dirs::home_dir().expect("/src/user_dir")
}

fn main() {
    println!("Hello, world!");
    login_user_info();
}

fn login_user_info() {
    println!("Please provide the following information, \"name\",\"Password\"");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Input..{}", &input),
        Err(error) => println!("Misinput, received error: {}", error),
    }

    let split_pos = input.bytes().position(|b| b == b',');
    match split_pos {
        Some(index) => {
            let curr_user = User {
                user: input[0..index].to_string(),
                userid: rand::random::<i32>(),
                password: input[index + 1..].trim().to_string(),
            };
            let mut user_path = user_dir();
            user_path.push("user_info.txt");
            let mut file = OpenOptions::new().append(true).create(true).open(user_path);
            match file {
                Ok(mut file) => {
                    file.write_all(
                        format!(
                            "{},{},{}\n",
                            curr_user.user,
                            curr_user.password,
                            curr_user.userid.to_string()
                        )
                        .as_bytes(),
                    )
                    .expect("Write Failed");
                    println!("Added User: {:?}", curr_user);
                }
                Err(e) => println!("errors occurred: {}", e),
            }
        }
        None => println!("Incorrect input"),
    }
}
