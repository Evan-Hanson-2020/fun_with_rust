use rand::prelude::*;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
#[derive(Debug)]
struct User {
    user: String,
    userid: i32,
    password: String,
}
const USER_PATH: &str = "~/user_info/log_info";
fn main() {
    println!("Hello, world!");
    login_user_info();
}
//
fn login_user_info() {
    println!("Please provde the following information, \"name\",\"Password\"");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => println!("Input..{}", &input),
        Err(error) => println!("Missinput, recieved error: {error}"),
    }

    println!("{input}");
    let split_pos = input.bytes().position(|b| b == b',');
    match split_pos {
        Some(index) => {
            let curr_user = User {
                user: input[0..index].to_string(),
                userid: rand::random::<i32>(),
                password: input[index..].to_string(),
            };

            let mut file = OpenOptions::new()
                .append(true)
                .open(USER_PATH)
                .expect("File Does Not Exist");
            file.write(
                format!(
                    "{},{},{}",
                    curr_user.user,
                    curr_user.password,
                    curr_user.userid.to_string()
                )
                .as_bytes(),
            )
            .expect("Write Failed");
            println!("Added User: {:?}", curr_user);
        }
        None => println!("Incorrect input"),
    }
}
