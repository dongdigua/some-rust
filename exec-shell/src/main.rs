use exec_shell::repl::{self, Repl};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() {
    enable_raw_mode().unwrap();
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not Enough Argument!");
        std::process::exit(1);
    } else {
        repl::run(&args[1]);
    }

    disable_raw_mode().unwrap();
}


