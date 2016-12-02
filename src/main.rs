extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin};

#[derive(Debug, Clone)]
struct State {
    buffer: Vec<String>,
    cursor: (u16, u16),
}

impl State {
    fn new(buffer: Vec<String>, cursor: (u16, u16)) -> State {
        State {
            buffer: buffer,
            cursor: cursor,
        }
    }

    fn blank() -> State {
        State::new(vec![String::new()], (1, 1))
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
           termion::cursor::Goto(1, 1))
        .unwrap();
    stdout.flush().unwrap();

    let mut state = State::blank();

    loop {
        let c = stdin.next().unwrap();
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Ctrl('p') => state = up(&state),
            Key::Ctrl('n') => state = down(&state),
            Key::Ctrl('f') => state = right(&state),
            Key::Ctrl('b') => state = left(&state),
            Key::Backspace => state = backspace(&state),
            Key::Char(c) => state = process_keystroke(&state, c),
            _ => {}
        }

        render(&state, &mut stdout);
    }
}

fn up(state: &State) -> State {
    let buffer = state.buffer.clone();
    let mut cursor = state.cursor.clone();

    if cursor.1 > 1 {
        cursor.1 -= 1;

        let buf_len = buffer[(cursor.1 - 1) as usize].len();
        // move curser to end of next line if current cursor pos > next line
        if buf_len == 0 {
            cursor.0 = 1;
        } else if buf_len < cursor.0 as usize {
            cursor.0 = buf_len as u16 + 1;
        }
    }

    State::new(buffer, cursor)
}

fn down(state: &State) -> State {
    let buffer = state.buffer.clone();
    let mut cursor = state.cursor.clone();

    if buffer.len() > (cursor.1) as usize {
        let buf_len = buffer[cursor.1 as usize].len();
        // move curser to end of next line if current cursor pos > next line
        if buf_len == 0 {
            cursor.0 = 1;
        } else if buf_len < cursor.0 as usize {
            cursor.0 = buffer[cursor.1 as usize].len() as u16 + 1;
        }

        cursor.1 += 1;
    }

    State::new(buffer, cursor)
}

fn right(state: &State) -> State {
    let buffer = state.buffer.clone();
    let mut cursor = state.cursor.clone();

    if buffer[(cursor.1 - 1) as usize].len() >= cursor.0 as usize {
        cursor.0 += 1;
    }

    State::new(buffer, cursor)
}

fn left(state: &State) -> State {
    let buffer = state.buffer.clone();
    let mut cursor = state.cursor.clone();

    if cursor.0 - 1 >= 1 {
        cursor.0 -= 1;
    }

    State::new(buffer, cursor)
}

fn backspace(state: &State) -> State {
    let mut buffer = state.buffer.clone();
    let mut cursor = state.cursor.clone();

    if cursor.0 - 1 >= 1 {
        cursor.0 -= 1;

        let pos = (cursor.0 - 1) as usize;
        let ref mut line = buffer[(cursor.1 - 1) as usize];
        if line.len() > pos {
            line.remove(pos);
        } else {
            line.pop();
        }
    } else if cursor.1 > 1 {
        let newline = buffer[(cursor.1 - 2) as usize].clone() + &buffer[(cursor.1 - 1) as usize];
        let newpos = buffer[(cursor.1 - 2) as usize].len() as u16;
        buffer[(cursor.1 - 2) as usize] = newline;
        buffer.remove((cursor.1 - 1) as usize);
        cursor.0 = newpos + 1;
        cursor.1 -= 1;
    }

    State::new(buffer, cursor)
}

fn process_keystroke(state: &State, chr: char) -> State {
    let mut buffer = state.buffer.clone();
    let mut cursor = state.cursor.clone();

    if chr == '\r' || chr == '\n' {
        buffer.push(String::new());
        cursor.0 = 1;
        cursor.1 += 1;
    } else {
        let ref mut line = buffer[(cursor.1 as usize) - 1];
        if cursor.0 <= line.len() as u16 {
            line.insert((cursor.0 - 1) as usize, chr);
        } else {
            line.push(chr);
        }
        cursor.0 += 1;
    }

    State::new(buffer, cursor)
}

fn render(state: &State, stdout: &mut termion::raw::RawTerminal<std::io::StdoutLock>) {
    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
        .unwrap();


    for line in &state.buffer {
        println!("{}\r", line);
    }

    // Debug state
    // println!("\n\n\n\n{:?}", state);

    write!(stdout,
           "{}",
           termion::cursor::Goto(state.cursor.0, state.cursor.1));

    stdout.flush().unwrap();
}
