use std::{
    io::{self, Write},
    process,
    time::Instant,
};
use rand::{
    thread_rng,
    Rng,
    seq::SliceRandom,
    rngs::mock::StepRng,
    random,
};
use crossterm::{
    event::{ read, Event, KeyCode, KeyEvent, KeyEventKind }, style::Stylize, terminal::size
};

type Field = Vec<Vec<usize>>;

const DRAW_STYLE: usize = 1; // 0 = ugly, 1 = pretty
const CLEAR_TERMINAL: bool = true;

macro_rules! swap {
    ($a:expr, $b:expr) => {
        {
            ($a, $b) = ($b, $a);
        }
    };
}

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
    fn size(&self) -> usize {
        self.width * self.height
    }
    fn blank_value(&self) -> usize {
        self.size()
    }
    fn is_sorted(&self) -> bool {
        self.field == self.give_sorted()
    }
    fn give_sorted(&self) -> Field {
        let mut field = vec![vec![0; self.height]; self.width];
        for (x, x_line) in field.iter_mut().enumerate() {
            for (y, item) in x_line.iter_mut().enumerate() {
                *item = x + y * self.width + 1;
            }
        }
        field
    }
    fn shuffle(&mut self) {
        let mut flat: Vec<usize> = self.field.iter().flatten().copied().collect();
        let mut flat_mut: Vec<&mut usize> = self.field.iter_mut().flatten().collect();
        let mut rng = StepRng::new(random(), random());

        flat.shuffle(&mut rng);
        flat.into_iter()
            .enumerate()
            .for_each(|(i, v)| *flat_mut[i] = v);

        loop {
            if self.is_sorted() {
                self.shuffle();
            }

            match self.is_valid_field() {
                true => break,
                false => self.shuffle_once(),
            }
        }
    }
    fn shuffle_once(&mut self) {
        let x1 = thread_rng().gen_range(0..self.width);
        let x2 = (x1 + thread_rng().gen_range(1..self.width)) % self.width;
        let y1 = thread_rng().gen_range(0..self.height);
        let y2 = (y1 + thread_rng().gen_range(1..self.height)) % self.height;
        self.swap(x1, y1, x2, y2);
    }
    fn is_valid_field(&self) -> bool {
        let mut swaps = 0;
        
        let mut flat: Vec<_> = self.field.iter().flatten().copied().collect();

        // this code got kindly stolen from kr8gz
        for i in 0..flat.len() {
            loop {
                let found = flat[i];
                if found == i + 1 { break }
                flat.swap(i, found - 1);
                swaps += 1;
            }
        }

        let (x, y) = self.index_blank();

        let blank_offset_x = self.width - x - 1;
        let blank_offset_y = self.height - y - 1;

        let blank_parity = (blank_offset_x + blank_offset_y) % 2;
        let swaps_parity = swaps % 2;

        // parity is wack

        // 2x2: !=
        // 2x3: !=
        // 2x4: ==
        // 2x5: ==
        // 2x6: !=
        // 2x7: !=
        // 2x8: ==

        // 3x3: !=
        // 3x4: ==
        // 3x5: ==
        // 3x6: !=
        // 3x7: !=
        // 3x8: ==

        if self.width >= 4 || self.height >= 4 {
            blank_parity == swaps_parity
        } else {
            blank_parity != swaps_parity
        }
    }
    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        swap!(self.field[x1][y1], self.field[x2][y2]);
    }
    fn index_blank(&self) -> (usize,usize) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.field[x][y] == self.blank_value() {
                    return (x, y)
                }
            }
        }
        (0,0)
    }
    fn how_to_play(&self) {
        println!("{}", "Welcome to the Sliding-Puzzle!".cyan());
        println!("{}", "Move the blank square around with WASD, Arrow Keys or Numpad (8456).".magenta());
    }
    fn objective(&self) {
        println!("{}", "The objective is to get all numbers in sequence horizontally.".yellow());
        println!("{}", "Just like the example here below:".red());
        SlidingPuzzle::new(self.width, self.height).draw();
    }
    fn draw(&self) {
        match DRAW_STYLE {
            0 => {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let content = self.field[x][y];
                        if content == self.size() - 1 {
                            print!("    ");
                        } else {
                            print!(" {: <3}", content);
                        }
                        if x < self.width-1 { print!("|") }
                    }
                    println!()
                }
            },
            1 => {
                let log = ((self.size() - 1) as f64 + 1.0).log10().ceil() as usize;
                for y in 0..self.height {
                    for x in 0..self.width {
                        let content = self.field[x][y];
                        if content == self.blank_value() {
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
                    println!();
                }
            },
            _ => println!("{}", "Invalid Drawing Style".on_red()),
        }
    }
    fn player_move(&mut self) {
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
                    Up | Char('w') | Char('8') =>
                        if by > 0 { lazy_swap!(bx,by-1) },
                    Left | Char('a') | Char('4') =>
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
    fn win(&self) {
        println!("{}", "Congratulations, you completed the puzzle!".green());
        
        if let Some(start_time) = self.start_time {
            println!("{}", format!("It took you {:.3?} to solve it", start_time.elapsed()).dark_magenta());
        }
        
        exit();
    }
}

fn main() {
    clear_terminal();

    let width = ask_for_size("Input width:", SizeInput::Width);
    let height = ask_for_size("Input height:", SizeInput::Height);

    let mut game = SlidingPuzzle::new(width, height);

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

    game.win();
}

enum SizeInput {
    Width,
    Height,
}

fn ask_for_size(message: &str, size_type: SizeInput) -> usize {
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
fn flush() {
    io::stdout().flush().unwrap();
}

#[inline]
fn exit() {
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
fn clear_terminal() {
    if CLEAR_TERMINAL {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // https://stackoverflow.com/questions/34837011/how-to-clear-the-terminal-screen-in-rust-after-a-new-line-is-printed
    }
}
