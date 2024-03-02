use std::time::Instant;

use crossterm::event::{ read, Event, KeyCode, KeyEvent, KeyEventKind };

use crate::SlidingPuzzle;

impl SlidingPuzzle {
    pub fn player_move(&mut self) {
        loop {
            if let Event::Key(KeyEvent { code, kind: KeyEventKind::Press|KeyEventKind::Repeat, .. }) = read().unwrap() {
                let (bx, by) = self.index_blank();

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
                    Up | Char('w') | Char('8') | Char('z') => // azerty moment 💀
                        if by > 0 { lazy_swap!(bx,by-1) },
                    Left | Char('a') | Char('4') | Char('q') => // azerty moment 💀
                        if bx > 0 { lazy_swap!(bx-1,by) },
                    Down | Char('s') | Char('5') | Char('2') =>
                        if by < self.height-1 { lazy_swap!(bx,by+1) },
                    Right | Char('d') | Char('6') =>
                        if bx < self.width-1 { lazy_swap!(bx+1,by) },
                    _ => (),
                }
            }
        }

        self.start_time.get_or_insert_with(Instant::now);
    }
}