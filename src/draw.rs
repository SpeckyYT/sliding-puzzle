use crossterm::style::Stylize;

use crate::{util::exit, SlidingPuzzle, DRAW_STYLE};

impl SlidingPuzzle {
    pub fn how_to_play(&self) {
        println!("{}", "Welcome to the Sliding-Puzzle!".cyan());
        println!("{}", "Move the blank square around with WASD, Arrow Keys or Numpad (8456).".magenta());
    }
    pub fn objective(&self) {
        println!("{}", "The objective is to get all numbers in sequence horizontally.".yellow());
        println!("{}", "Just like the example here below:".red());
        SlidingPuzzle::new(self.width, self.height).draw();
    }
    pub fn win(&self) {
        println!("{}", "Congratulations, you completed the puzzle!".green());
        
        if let Some(start_time) = self.start_time {
            println!("{}", format!("It took you {:.3?} to solve it", start_time.elapsed()).dark_magenta());
        }
        
        exit();
    }
    pub fn draw(&self) {
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
}