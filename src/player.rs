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
                    Up | Char('w' | 'z' | '8' | ',') =>
                        if by > 0 { lazy_swap!(bx,by-1) },
                    Left | Char('a' | 'q' | '4') =>
                        if bx > 0 { lazy_swap!(bx-1,by) },
                    Down | Char('s' | '5' | '2' | 'o') =>
                        if by < self.height-1 { lazy_swap!(bx,by+1) },
                    Right | Char('d' | '6' | 'e') =>
                        if bx < self.width-1 { lazy_swap!(bx+1,by) },
                    _ => (),
                }
            }
        }

        self.start_time.get_or_insert_with(Instant::now);
    }
}
