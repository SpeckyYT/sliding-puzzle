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


fn main() {
    if WIDTH < 2 || HEIGHT < 2 {
        println!("{}", "Invalid Field Size".on_red());
        exit();
    }

    let mut field: [usize; SIZE] = field_sorted();

    shuffle(&mut field);

    clear_terminal();
    draw(&field);
    how_to_play();
    objective();

    while !is_sorted(&field) {
        player_move(&mut field);
        clear_terminal();
        draw(&field);
    }

    win();
}

fn how_to_play() {
    println!("{}", "Welcome to the Sliding-Puzzle!".cyan());
    println!("{}", "Move the blank square around with WASD, Arrow Keys or Numpad (8456).".magenta());
}

fn objective() {
    println!("{}", "The objective is to get all numbers in sequence horizontally.".yellow());
    println!("{}", "Just like the example here below:".red());
    draw(&field_sorted());
}

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

fn field_sorted() -> [usize; SIZE] {
    FIELD_SORTED_RANGE.collect::<Vec<usize>>().try_into().unwrap()
}

fn index_to_position(index: usize) -> (usize, usize) { (index % WIDTH, index / WIDTH) }
fn position_to_index(position: (usize, usize)) -> usize { position.0 + position.1 * WIDTH }

fn index_value(field: &[usize], number: usize) -> usize {
    field.iter().position(|a| *a == number).unwrap()
}
fn index_blank(field: &[usize]) -> usize {
    index_value(field, BLANK_VALUE)
}

fn is_sorted(field: &[usize]) -> bool {
    field[..] == field_sorted()[..]
}

fn shuffle_once(field: &mut [usize]) {
    let mut a: usize = 0;
    let mut b: usize = 0;
    while a == b {
        a = thread_rng().gen_range(0..SIZE);
        b = thread_rng().gen_range(0..SIZE);
    }
    field.swap(a,b);
}

fn shuffle(field: &mut [usize]) {
    for _ in 0..(2 * (WIDTH.pow(2) + HEIGHT.pow(2))) {
        shuffle_once(field);
    }

    for i in 1..usize::MAX {
        let (x,y) = index_to_position(index_blank(field));
        if (WIDTH-1 - x + HEIGHT-1 - y) % 2 == (i % 2) {
            shuffle_once(field);
        } else { break }
    }
}

fn player_move(field: &mut [usize]) {
    let mut moved = false;
    while !moved {
        if let Event::Key(event) = read().unwrap() {
            let blank_index = index_blank(field);
            let blank_position = index_to_position(blank_index);
            let (x, y) = blank_position;

            let mut lazy_swap = |x:usize, y:usize| {
                field.swap(
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

fn draw(field: &[usize]) {
    match DRAW_STYLE {
        0 => {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let content = field[position_to_index((x,y)) as usize];
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
                    let content = field[position_to_index((x,y)) as usize];
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
