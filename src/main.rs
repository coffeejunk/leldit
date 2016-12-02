extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin};

#[derive(Debug, Clone)]
struct State {
    buffer: Vec<String>,
    cursor: (usize, usize),
}

impl State {
    fn new(buffer: Vec<String>, cursor: (usize, usize)) -> State {
        State {
            buffer: buffer,
            cursor: cursor,
        }
    }

    fn blank() -> State {
        State::new(vec![String::new()], (1, 1))
    }

    fn up(&self) -> State {
        let buffer = self.buffer.clone();
        let mut cursor = self.cursor.clone();

        if cursor.1 > 1 {
            cursor.1 -= 1;

            let buf_len = buffer[cursor.1 - 1].len();
            // move curser to end of next line if current cursor pos > next line
            if buf_len == 0 {
                cursor.0 = 1;
            } else if buf_len < cursor.0 {
                cursor.0 = buf_len + 1;
            }
        }

        State::new(buffer, cursor)
    }

    fn down(&self) -> State {
        let buffer = self.buffer.clone();
        let mut cursor = self.cursor.clone();

        if buffer.len() > cursor.1 {
            let buf_len = buffer[cursor.1].len();
            // move curser to end of next line if current cursor pos > next line
            if buf_len == 0 {
                cursor.0 = 1;
            } else if buf_len < cursor.0 {
                cursor.0 = buffer[cursor.1].len() + 1;
            }

            cursor.1 += 1;
        }

        State::new(buffer, cursor)
    }

    fn right(&self) -> State {
        let buffer = self.buffer.clone();
        let mut cursor = self.cursor.clone();

        if buffer[cursor.1 - 1].len() >= cursor.0 {
            cursor.0 += 1;
        }

        State::new(buffer, cursor)
    }

    fn left(&self) -> State {
        let buffer = self.buffer.clone();
        let mut cursor = self.cursor.clone();

        if cursor.0 - 1 >= 1 {
            cursor.0 -= 1;
        }

        State::new(buffer, cursor)
    }

    fn backspace(&self) -> State {
        let mut buffer = self.buffer.clone();
        let mut cursor = self.cursor.clone();

        if cursor.0 - 1 >= 1 {
            cursor.0 -= 1;

            let pos = cursor.0 - 1;
            let ref mut line = buffer[cursor.1 - 1];
            if line.len() > pos {
                line.remove(pos);
            } else {
                line.pop();
            }
        } else if cursor.1 > 1 {
            let newline = buffer[cursor.1 - 2].clone() + &buffer[cursor.1 - 1];
            let newpos = buffer[cursor.1 - 2].len();
            buffer[cursor.1 - 2] = newline;
            buffer.remove(cursor.1 - 1);
            cursor.0 = newpos + 1;
            cursor.1 -= 1;
        }

        State::new(buffer, cursor)
    }

    fn newline(&self) -> State {
        let mut buffer = self.buffer.clone();
        let mut cursor = self.cursor.clone();


        let buf_len = buffer[cursor.1 - 1].len();
        if buf_len < cursor.0 {
            buffer.insert(cursor.1, String::new());
        } else {
            let old_line = buffer[cursor.1 - 1].clone();
            let (first, second) = old_line.split_at(cursor.0 - 1);
            let first = first.to_string();
            let second = second.to_string();
            buffer.remove(cursor.1 - 1);
            buffer.insert(cursor.1 - 1, first);
            buffer.insert(cursor.1, second);
        }

        cursor.0 = 1;
        cursor.1 += 1;

        return State::new(buffer, cursor);
    }

    fn insert(&self, chr: char) -> State {
        let mut buffer = self.buffer.clone();
        let mut cursor = self.cursor.clone();

        {
            let ref mut line = buffer[cursor.1 - 1];
            if cursor.0 <= line.len() {
                line.insert(cursor.0 - 1, chr);
            } else {
                line.push(chr);
            }
        }

        cursor.0 += 1;

        State::new(buffer, cursor)
    }
}

fn main() {
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let mut stdin = stdin.keys();

    write!(stdout,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Show,
           termion::cursor::Goto(1 as u16, 1 as u16))
        .unwrap();
    stdout.flush().unwrap();

    let mut state = State::blank();

    loop {
        let c = stdin.next().unwrap();
        match process_keystroke(&state, c.unwrap()) {
            None => break,
            Some(x) => state = x,
        }

        render(&state, &mut stdout);
    }
}

fn process_keystroke(state: &State, chr: Key) -> Option<State> {
    match chr {
        Key::Ctrl('c') => None,
        Key::Ctrl('p') => Some(state.up()),
        Key::Ctrl('n') => Some(state.down()),
        Key::Ctrl('f') => Some(state.right()),
        Key::Ctrl('b') => Some(state.left()),
        Key::Backspace => Some(state.backspace()),
        Key::Char(chr) => {
            if chr == '\r' || chr == '\n' {
                Some(state.newline())
            } else {
                Some(state.insert(chr))
            }
        }
        _ => Some(state.clone()),
    }

}

fn render(state: &State, stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock>) {
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1 as u16, 1 as u16))
        .unwrap();


    for line in &state.buffer {
        println!("{}\r", line);
    }

    // Debug state
    // println!("\n\n\n\n{:?}", state);

    write!(stdout,
           "{}",
           termion::cursor::Goto(state.cursor.0 as u16, state.cursor.1 as u16));

    stdout.flush().unwrap();
}
