use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let command = &args[2];
    let phrase = &args[3];
    println!("query: {}", query);
    if query == "grep" {
        grep(command.to_string(), phrase.to_string());
    }
}

fn grep(file: String, check: String) {
    let contents = fs::read_to_string(file).expect("Not a valid Path::");
    println!("file Contents:\n{&contents}");
    if contents.to_string().contains(check) {
        let location = contents.rfind(&check);
        println!("Location of current grep command {:?}", location);
    }
}

fn ls() {}
