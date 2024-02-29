use std::{io::{self, Write}, process};

use crossterm::{event::{read, Event, KeyEvent, KeyEventKind}, style::Stylize, terminal::size};

use crate::{SlidingPuzzle, CLEAR_TERMINAL};

macro_rules! swap {
    ($a:expr, $b:expr) => {
        {
            ($a, $b) = ($b, $a);
        }
    };
}

impl SlidingPuzzle {
    pub fn size(&self) -> usize {
        self.width * self.height
    }
    pub fn blank_value(&self) -> usize {
        self.size()
    }
    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        swap!(self.field[x1][y1], self.field[x2][y2]);
    }
    pub fn index_blank(&self) -> (usize,usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.field[x][y] == self.blank_value() {
                    return (x, y)
                }
            }
        }
        (0,0)
    }
}

pub enum SizeInput {
    Width,
    Height,
}

pub fn ask_for_size(message: &str, size_type: SizeInput) -> usize {
    loop {
        print!("{} ", message);
        flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        clear_terminal();
        flush();

        let size = size().map(|(w,h)| match size_type {
            SizeInput::Width => w,
            SizeInput::Height => h,
        });

        match input.trim().parse::<usize>() {
            Ok(number) if number <= 1 =>
                println!("{}", "Number should be greater than 1".on_red()),
            Ok(number) if size.is_ok_and(|size| number > size.into()) =>
                println!("{}", "That's bigger than your terminal window".on_red()),
            Ok(number) => return number,
            Err(_) => println!("{}", "Input should be a number".on_red()),
        }
    }
}

#[inline]
pub fn flush() {
    io::stdout().flush().unwrap();
}

#[inline]
pub fn exit() {
    println!("{}", "Press any key 3 times to close...".dark_cyan());
    let mut count: u8 = 3;
    while count > 0 {
        if let Event::Key(KeyEvent { kind: KeyEventKind::Press, .. }) = read().unwrap() {
            count -= 1
        }
    }
    process::exit(0);
}

#[inline]
pub fn clear_terminal() {
    if CLEAR_TERMINAL {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    }
}
