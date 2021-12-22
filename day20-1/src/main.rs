use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

type NumType = i64;
type Image = HashSet<(NumType, NumType)>;
type Index = Vec<char>;

struct Limits {
    r_min: NumType,
    r_max: NumType,
    c_min: NumType,
    c_max: NumType,
}

impl Limits {
    fn new() -> Self {
        Self {
            r_min: NumType::MAX,
            r_max: NumType::MIN,
            c_min: NumType::MAX,
            c_max: NumType::MIN,
        }
    }

    fn set(&mut self, row: NumType, col: NumType) {
        self.c_max = self.c_max.max(col);
        self.c_min = self.c_min.min(col);
        self.r_max = self.r_max.max(row);
        self.r_min = self.r_min.min(row);
    }
}

fn main() {
    let file = File::open("./input").unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let index: Index = lines.next().unwrap().unwrap().chars().collect();
    lines.next(); // Skip blank line
    let mut image = Image::new();
    let mut limits = Limits::new();

    for (row_num, row) in lines.enumerate() {
        for (col_num, col) in row.unwrap().chars().enumerate() {
            if col == '#' {
                limits.set(row_num as NumType, col_num as NumType);
                image.insert((row_num as NumType, col_num as NumType));
            }
        }
    }

    // Flickering only happens with index[0] set to #
    let do_a_flip = index[0] == '#';

    // Change to 50 for part 2
    for step in 0..2 {
        let (e, el) = enhance(&image, &index, &limits, do_a_flip && step % 2 != 0);
        image = e;
        limits = el;
    }

    println!("Lit count is: {}", image.len());
}

fn enhance(image: &Image, index: &Index, limits: &Limits, flip: bool) -> (Image, Limits) {
    let mut new_image = Image::new();
    let mut new_limits = Limits::new();
    let pixel_target = if flip { '#' } else { '.' };

    for row in limits.r_min - 1..=limits.r_max + 1 {
        for col in limits.c_min - 1..=limits.c_max + 1 {
            let num = expand(image, row, col, flip);
            let pixel = index[num as usize];
            if pixel == pixel_target {
                new_limits.set(row as NumType, col as NumType);
                new_image.insert((row, col));
            }
        }
    }

    (new_image, new_limits)
}

fn expand(image: &Image, row: NumType, col: NumType, flip: bool) -> u16 {
    let mut out_num = if flip { (1 << 9) - 1 } else { 0 };
    let mut bit_pos = 9;
    for r_num in row - 1..=row + 1 {
        for c_num in col - 1..=col + 1 {
            bit_pos -= 1;
            if image.contains(&(r_num, c_num)) {
                out_num ^= 1 << bit_pos;
            }
        }
    }

    out_num
}
