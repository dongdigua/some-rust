use std::io::{self, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();

    loop {
        print!("({})> ", &args[1]);
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input!");

        if input == "q\n".to_string() {
            break;
        } else {
            let output = Command::new(&args[1])
                .arg(&input)
                .output()
                .expect("failed to execute!");
            print!("{}",
                   String::from_utf8_lossy(&output.stdout));
        }
        input = String::from("");
    }
}
