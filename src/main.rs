mod util;
mod shuffle;
mod player;
mod draw;

pub const DRAW_STYLE: usize = 1; // 0 = ugly, 1 = pretty
pub const CLEAR_TERMINAL: bool = true;

type Field = Vec<Vec<usize>>;

use std::time::Instant;
use util::{ask_for_size, clear_terminal, cmd_fix, exit, flush, SizeInput};

const BOT_PLAY: bool = true;

#[derive(Clone)]
struct SlidingPuzzle {
    field: Field,
    width: usize,
    height: usize,
    start_time: Option<Instant>,
}

impl SlidingPuzzle {
    fn new(width: usize, height: usize) -> SlidingPuzzle {
        let mut game = SlidingPuzzle {
            field: vec![vec![0; height]; width],
            width,
            height,
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

    let mut print_time = Instant::now();

    while !game.is_sorted() {
        if BOT_PLAY {
            game.bot_random_move();
            if print_time.elapsed() > Duration::from_millis(16) {
                print_time = Instant::now();
                clear_terminal();
                game.draw();
                flush();
            }
        } else {
            game.player_move();
            clear_terminal();
            game.draw();
            flush();
        }
    }

    clear_terminal();
    game.draw();
    game.win();
    flush();
    exit();
}
