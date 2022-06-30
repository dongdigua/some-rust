use crossterm::{execute, cursor};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::{stdin, stdout, Write};
use std::process::Command;

#[derive(Debug)]
pub struct Repl {
    pub buffer: Vec<char>,
    pub cursor: usize,
    pub history: Vec<Vec<char>>,
    pub history_index: usize,
    //pub complete: CompleteTree,
}

enum Deletion {
    Bs,
    Del,
}

impl Repl {
    fn new() -> Self {
        Repl {
            buffer: vec![],
            cursor: 0,
            history: vec![],
            history_index: 0,  // where to put the next history!
        }
    }

    // &self is shorthand of "self: &Self", Self is the struct to implment
    fn clear(&mut self) {
        // oh gosh, a function without a return value is evil!
        self.cursor = 0;
        self.buffer = vec![];
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
        }
        self.history_index = self.history.len();
        self.clear();
    }

    fn previous(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.buffer = self.history[self.history_index].clone()
        }
    }

    fn next(&mut self) {
        if self.history_index < self.history.len() {
            self.history_index += 1;
            self.buffer = self.history[self.history_index].clone()
        }
    }

    // basically from tsoding's video
    fn render(&self, prompt: &String, out: &mut impl Write) {
        let buffer: String = self.buffer.iter().collect();
        //clear commands must be executed/queued for execution otherwise they do nothing.
        execute!(out, Clear(ClearType::CurrentLine));
        write!(
            out,
            "\r{}{}\r{}",
            prompt,
            &buffer,
            cursor::MoveRight((prompt.len() + self.cursor).try_into().unwrap())
        )
        .unwrap();
    }
}

pub fn run(arg: &String) {
    let mut repl = Repl::new();
    let prompt = format!("({})> ", &arg);
    loop {
        repl.render(&prompt, &mut stdout());
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
                KeyCode::Tab => print!("completion is not done yet"),
                KeyCode::Enter => {
                    if ['q'] == &repl.buffer[..] {
                        break;
                    }
                    repl.add_history();
                    print!("\r\n");
                }
                _ => print!("{:?}", key.code),
            },
            _ => (),
        }
    }
}
