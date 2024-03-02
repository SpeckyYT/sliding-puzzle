mod util;
mod shuffle;
mod player;
mod draw;

pub const DRAW_STYLE: usize = 1; // 0 = ugly, 1 = pretty
pub const CLEAR_TERMINAL: bool = true;

type Field = Vec<Vec<usize>>;

use std::time::Instant;
use util::{ask_for_size, clear_terminal, cmd_fix, exit, flush, SizeInput};

#[derive(Clone)]
struct SlidingPuzzle {
    field: Field,
    width: usize,
    height: usize,
    moves: usize,
    start_time: Option<Instant>,
}

impl SlidingPuzzle {
    fn new(width: usize, height: usize) -> SlidingPuzzle {
        let mut game = SlidingPuzzle {
            field: vec![vec![0; height]; width],
            width,
            height,
            moves: 0,
            start_time: None,
        };

        game.field = game.give_sorted();

        game
    }
}

fn main() {
    cmd_fix();
    clear_terminal();
    flush();

    let width = ask_for_size("Input width:", SizeInput::Width);
    let height = ask_for_size("Input height:", SizeInput::Height);

    let mut game = SlidingPuzzle::new(width, height);

    game.shuffle();

    clear_terminal();
    flush();
    game.draw();
    game.how_to_play();
    game.objective();
    flush();

    while !game.is_sorted() {
        game.player_move();
        clear_terminal();
        game.draw();
        flush();
    }

    game.win();
    flush();
    exit();
}
