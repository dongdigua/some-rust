use crossterm::{execute, cursor};
use crossterm::event::{read, Event, KeyCode};
//use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use trie_rs::TrieBuilder;

use std::io::{stdout, Write};
use std::process::Command;

//#[derive(Debug)]
pub struct Repl {
    pub arg: String,
    pub buffer: Vec<char>,
    pub cursor: usize,
    pub history: Vec<Vec<char>>,
    pub history_index: usize,
    pub trie: TrieBuilder<char>,
    pub completion: Vec<String>,
    pub print_completion: bool,
}

enum Deletion {
    Bs,
    Del,
}

impl Repl {
    fn new(arg: String) -> Self {
        Repl {
            arg: arg,
            buffer: vec![],
            cursor: 0,
            history: vec![],
            history_index: 0,  // where to put the next history!
            trie: TrieBuilder::new(),
            completion: vec![],
            print_completion: false,
        }
    }

    // &self is shorthand of "self: &Self", Self is the struct to implment
    fn clear(&mut self) {
        // oh gosh, a function without a return value is evil!
        self.cursor = 0;
        self.buffer = vec![];
        self.completion = vec![];
    }

    fn left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        };
    }

    fn right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        };
    }

    fn insert(&mut self, ch: char) {
        self.buffer.insert(self.cursor, ch);
        self.cursor += 1;
    }

    fn delete(&mut self, mode: Deletion) {
        match mode {
            Deletion::Del => {
                if self.cursor < self.buffer.len() {
                    self.buffer.remove(self.cursor);
                }
            }
            Deletion::Bs => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.buffer.remove(self.cursor);
                }
            }
        };
    }

    fn add_history(&mut self) {
        if Some(&self.buffer) != self.history.last() {
            self.history.push(self.buffer.clone());  //TODO need optimize, probably Box<_>
            self.trie.push(&self.buffer);
        }
        self.history_index = self.history.len();
    }

    fn previous(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.buffer = self.history[self.history_index].clone()
        }
    }

    fn next(&mut self) {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
            self.buffer = self.history[self.history_index].clone()
        }
    }

    fn complete(&mut self) {
        if self.buffer.len() == 0 {
            self.completion = self.history
                .iter()
                .map(|chs| chs.iter().collect())
                .collect();
        } else {
            let trie = self.trie.build();
            self.completion =
                trie.predictive_search(&self.buffer)
                .iter()
                .map(|chs| chs.iter().collect())
                .collect();
        }
        self.print_completion = true;
    }

    // basically from tsoding's video
    fn render(&mut self, out: &mut impl Write) {
        let prompt = format!("({})> ", &self.arg);
        let buffer: String = self.buffer.iter().collect();
        //clear commands must be executed/queued for execution otherwise they do nothing.
        execute!(out, Clear(ClearType::CurrentLine)).unwrap();
        write!(
            out,
            "\r{}{}\r{}",
            prompt,
            &buffer,
            cursor::MoveRight((prompt.len() + self.cursor).try_into().unwrap())
        ).unwrap();
        if self.print_completion {
            write!(out, "\r\n");
            for s in &self.completion {
                write!(out, "{}\r\n", s);
            }
            self.print_completion = false;
        }
    }

    fn execute(&mut self) {
        let output = Command::new(&self.arg)
            .arg(self.buffer.iter().collect::<String>())
            .output()
            .expect("Failed to execute command");
        for line in  String::from_utf8_lossy(&output.stdout).lines() {
            println!("\r{}", line);
        }
        self.clear();
    }
}

pub fn run(arg: &String) {
    let mut repl = Repl::new(arg.to_string());
    loop {
        repl.render(&mut stdout());
        stdout().flush().unwrap();

        match read().unwrap() {
            Event::Key(key) => match key.code {
                KeyCode::Char(ch) => repl.insert(ch),
                KeyCode::Delete => repl.delete(Deletion::Del),
                KeyCode::Backspace => repl.delete(Deletion::Bs),
                KeyCode::Left => repl.left(),
                KeyCode::Right => repl.right(),
                KeyCode::Up => repl.previous(),
                KeyCode::Down => repl.next(),
                KeyCode::Tab => repl.complete(),
                KeyCode::Enter => {
                    if ['q'] == &repl.buffer[..] {
                        break;
                    }
                    repl.add_history();
                    repl.execute();
                    print!("\r\n");
                }
                _ => print!("{:?}", key.code),
            },
            _ => (),
        }
    }
}
