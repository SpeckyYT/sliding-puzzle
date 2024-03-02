use std::time::Instant;

use crossterm::event::{ read, Event, KeyCode, KeyEvent, KeyEventKind };
use rand::random;

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
    // so basically a function is fn ? yes
    pub fn bot_random_move(&mut self) {
        let (bx, by) = self.index_blank();
        macro_rules! lazy_swap {
            ($x:expr, $y:expr) => {
                if $x < self.width && $y < self.height {
                    self.swap($x, $y, bx, by);
                    self.moves += 1;
                }
            }
        }
        match (random::<f64>() * 4.0) as usize {
            0 => lazy_swap!(bx-1,by),
            1 => lazy_swap!(bx,by-1),
            2 => lazy_swap!(bx+1,by),
            3 => lazy_swap!(bx,by+1),
            _ => unreachable!()
        }
        self.start_time.get_or_insert_with(Instant::now);
    }

    fn get_surrouding(&self) -> [Option<Direction<usize>>; 4] {
        let (x, y) = self.index_blank();

        [
            (y + 1 < self.height).then(|| Direction::Up(self.field[x][y+1])),
            (x + 1 < self.width).then(|| Direction::Right(self.field[x+1][y])),
            (x > 0).then(|| Direction::Left(self.field[x-1][y-1])),
            (y > 0).then(|| Direction::Down(self.field[x][y-1])),
        ]
    }

    fn get_distance(&self, x: usize, y: usize) -> usize {
        let tile_index = self.field[x][y] - 1;
        let (dest_x, dest_y) = (tile_index % self.width, tile_index / self.width);
        
        dest_x.abs_diff(x) + dest_y.abs_diff(y)
    }

    pub fn bot_move(&mut self) {
        let tile_arr = self.get_surrouding();
        let mut best_move = (Direction::Up(0), usize::MAX);
        for direction in tile_arr.into_iter().filter_map(|a| a) {
            let (blank_x, blank_y) = self.index_blank();
            //is that valid or shall I do blank_corods = self.blank_index();
            // this works without issues

            // I don't think the blank's distance is something you want
            // if you switch a tile, it will get the blank's distance
            // then you want `blank_x + 1, blank_y` for example
            // basically when I move up, the upward value go downward to where blank was
            // so I have to know the current blank distance to the tile (i think)
            // but you need to move it first before getting the distance
            // then yes, you're right
            let blank_distance = self.get_distance(blank_x, blank_y);
            // let tile_distance = self.get_distance(x, y);
            // blank_distance < tile_distance
            if blank_distance < best_move.1 {
                let tile_number = *direction.get_value();
                best_move.0 = direction; // 
                // best_move.1 = tile_number
            }
            // update best move
        }
                
    }
}

enum Direction<T> {
    Up(T),
    Left(T),
    Down(T),
    Right(T),
}

impl<T> Direction<T> {
    pub fn get_value(&self) -> &T {
        match self {
            Direction::Up(t) => t,
            Direction::Left(t) => t,
            Direction::Down(t) => t,
            Direction::Right(t) => t,
        }
    }
}