use std::{ ops::RangeInclusive, process };
use rand::{ thread_rng, Rng };
use crossterm::{
    event::{ read, Event, KeyCode }, style::Stylize,
};

const WIDTH:usize = 4;
const HEIGHT:usize = 4;

const DRAW_STYLE:usize = 1; // 0 = ugly, 1 = pretty

const SIZE:usize = WIDTH * HEIGHT;
const BLANK_VALUE:usize = SIZE;
const FIELD_SORTED_RANGE:RangeInclusive<usize> = 1..=SIZE;

struct SlidingPuzzle {
    field: [usize; SIZE],
}

impl SlidingPuzzle {
    fn is_sorted(&self) -> bool {
        self.field[..] == SlidingPuzzle::give_sorted()[..]
    }
    fn give_sorted() -> [usize; SIZE] {
        return FIELD_SORTED_RANGE.collect::<Vec<_>>().try_into().unwrap()
    }
    fn shuffle(&mut self) {
        for _ in 0..(2 * (WIDTH.pow(2) + HEIGHT.pow(2))) {
            self.shuffle_once();
        }
    
        for i in 1..usize::MAX {
            let (x,y) = index_to_position(self.index_blank());
            if (WIDTH-1 - x + HEIGHT-1 - y) % 2 == (i % 2) {
                self.shuffle_once();
            } else { break }
        }
    }
    fn shuffle_once(&mut self) {
        let mut a: usize = 0;
        let mut b: usize = 0;
        while a == b {
            a = thread_rng().gen_range(0..SIZE);
            b = thread_rng().gen_range(0..SIZE);
        }
        self.field.swap(a,b);
    }
    fn index_value(&self, number: usize) -> usize {
        self.field.iter().position(|a| *a == number).unwrap()
    }
    fn index_blank(&self) -> usize {
        self.index_value(BLANK_VALUE)
    }
    fn how_to_play(&self) {
        println!("{}", "Welcome to the Sliding-Puzzle!".cyan());
        println!("{}", "Move the blank square around with WASD, Arrow Keys or Numpad (8456).".magenta());
    }
    fn objective(&self) {
        println!("{}", "The objective is to get all numbers in sequence horizontally.".yellow());
        println!("{}", "Just like the example here below:".red());
        SlidingPuzzle::draw_field(&mut SlidingPuzzle::give_sorted());
    }
    fn draw(&self) {
        match DRAW_STYLE {
            0 => {
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let content = self.field[position_to_index((x,y)) as usize];
                        if content == BLANK_VALUE {
                            print!("    ");
                        } else {
                            print!(" {: <3}", content);
                        }
                        if x < WIDTH-1 { print!("|") }
                    }
                    if y < HEIGHT-1 { print!("\n") }
                }
            },
            1 => {
                let log = (SIZE as f64).log10().ceil() as usize;
                for y in 0..HEIGHT {
                    for x in 0..WIDTH {
                        let content = self.field[position_to_index((x,y)) as usize];
                        if content == BLANK_VALUE {
                            print!("{}", " ".repeat(log));
                        } else {
                            let stringified = format!("{:log$}", content);
                            print!(
                                "{}",
                                if content % 2 == 0 {
                                    stringified.on_red().white()
                                } else {
                                    stringified.on_green().black()
                                },
                            )
                        }
                    }
                    print!("{}", (160 as char).reset()); // fixes window rescaling ansi bug
                    print!("\n");
                }
            },
            _ => println!("{}", "Invalid Drawing Style".on_red()),
        }
    }
    fn draw_field(field: &mut [usize; SIZE]) {
        SlidingPuzzle {
            field: *field,
        }.draw();
    }
    fn player_move(&mut self) {
        let mut moved = false;
        while !moved {
            if let Event::Key(event) = read().unwrap() {
                let blank_index = self.index_blank();
                let blank_position = index_to_position(blank_index);
                let (x, y) = blank_position;
    
                let mut lazy_swap = |x:usize, y:usize| {
                    self.field.swap(
                        blank_index,
                        position_to_index((x,y))
                    );
                    moved = true;
                };
    
                match event.code {
                    KeyCode::Up |
                    KeyCode::Char('w') |
                    KeyCode::Char('8') =>
                        if y > 0 { lazy_swap(x,y-1) },
                    KeyCode::Left |
                    KeyCode::Char('a') |
                    KeyCode::Char('4') =>
                        if x > 0 { lazy_swap(x-1,y) },
                    KeyCode::Down |
                    KeyCode::Char('s') |
                    KeyCode::Char('5') =>
                        if y < HEIGHT-1 { lazy_swap(x,y+1) },
                    KeyCode::Right |
                    KeyCode::Char('d') |
                    KeyCode::Char('6') =>
                        if x < WIDTH-1 { lazy_swap(x+1,y) },
                    _ => (),
                }
            }
        }
    }
}

fn main() {
    if WIDTH < 2 || HEIGHT < 2 {
        println!("{}", "Invalid Field Size".on_red());
        exit();
    }

    let mut game = SlidingPuzzle {
        field: SlidingPuzzle::give_sorted(),
    };

    game.shuffle();

    clear_terminal();
    game.draw();
    game.how_to_play();
    game.objective();

    while !game.is_sorted() {
        game.player_move();
        clear_terminal();
        game.draw();
    }

    win();
}

fn index_to_position(index: usize) -> (usize, usize) { (index % WIDTH, index / WIDTH) }
fn position_to_index(position: (usize, usize)) -> usize { position.0 + position.1 * WIDTH }

fn win() {
    println!("{}", "Congratulations, you completed the puzzle!".green());
    exit();
}

fn exit() {
    println!("{}", "Press any key 3 times to close...".dark_cyan());
    for _ in 0..3 { read().unwrap(); }
    process::exit(0);
}

fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
}
