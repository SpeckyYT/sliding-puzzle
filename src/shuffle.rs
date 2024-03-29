use rand::{prelude::*, rngs::mock::StepRng};

use crate::{Field, SlidingPuzzle};

impl SlidingPuzzle {
    pub fn is_sorted(&self) -> bool {
        self.field == self.give_sorted()
    }
    pub fn give_sorted(&self) -> Field {
        let mut field = vec![vec![0; self.height]; self.width];
        for (x, x_line) in field.iter_mut().enumerate() {
            for (y, item) in x_line.iter_mut().enumerate() {
                *item = x + y * self.width + 1;
            }
        }
        field
    }
    pub fn shuffle(&mut self) {
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
    pub fn shuffle_once(&mut self) {
        let mut rng = thread_rng();
        let x1 = rng.gen_range(0..self.width);
        let x2 = (x1 + rng.gen_range(1..self.width)) % self.width;
        let y1 = rng.gen_range(0..self.height);
        let y2 = (y1 + rng.gen_range(1..self.height)) % self.height;
        self.swap(x1, y1, x2, y2);
    }
    pub fn is_valid_field(&self) -> bool {
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
        let parity = blank_parity == swaps_parity;

        // parity is wack

        // min(<=3) & max(<=3): !parity
        // min(<=3): max(>=4) % 4 in [0, 1] ? parity : !parity
        // min(>=4) & max(>=4): parity

        let min = self.width.min(self.height);
        let max = self.width.max(self.height);

        match (min, max) {
            (0..=3, 0..=3) => !parity,
            (0..=3, 4..) => parity == (max % 4 < 2),
            _ => parity,
        }
    }
}