use std::time::Instant;

use crossterm::event::{ read, Event, KeyCode, KeyEvent, KeyEventKind };
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

use crate::SlidingPuzzle;

impl SlidingPuzzle {
    pub fn player_move(&mut self) {
        enable_raw_mode().unwrap();
        loop {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press|KeyEventKind::Repeat, .. }) = read().unwrap() {
                let (bx, by) = self.index_blank();
                disable_raw_mode().unwrap();

                macro_rules! lazy_swap {
                    ($x:expr, $y:expr) => {
                        if $x < self.width && $y < self.height {
                            self.swap($x, $y, bx, by);
                            self.moves += 1;
                            break;
                        }
                    }
                }
                
                use KeyCode::*;

                match code {
                    Up | Char('w') | Char('8') | Char('z') | Char(',') =>
                        if by > 0 { lazy_swap!(bx,by-1) },
                    Left | Char('a') | Char('4') | Char('q') =>
                        if bx > 0 { lazy_swap!(bx-1,by) },
                    Down | Char('s') | Char('5') | Char('2') | Char('o') =>
                        if by < self.height-1 { lazy_swap!(bx,by+1) },
                    Right | Char('d') | Char('6') | Char('e') =>
                        if bx < self.width-1 { lazy_swap!(bx+1,by) },
                    _ => (),
                }
            }
        }

        self.start_time.get_or_insert_with(Instant::now);
    }
}
