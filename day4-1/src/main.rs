use std::fs::File;
use std::io::{self, BufRead};

const BOARD_SIZE: usize = 5;

fn main() {
    // Parse input
    let file = File::open("./input").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .filter_map(io::Result::ok)
        .collect();

    let draws: Vec<u32> = lines[0]
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect();

    let mut boards = Board::parse_boards(&lines[2..]);

    // Actually play the game
    for draw in draws {
        for board in &mut boards {
            if board.play(draw) {
                println!("VICTORY ACHIEVED. Score: {}", board.sum * draw);
                return;
            }
        }
    }

    println!("YOU DIED");
}

type Val = (u32, bool);
struct Board {
    sum: u32,
    rows: [[Val; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    fn parse_boards(lines: &[String]) -> Vec<Self> {
        let mut boards: Vec<Board> = Vec::new();
        let mut board = Board::new();
        let mut line_num = 0;
        for line in lines {
            if line.len() <= 1 {
                // blank lines are end of board
                boards.push(board);
                board = Board::new();
                line_num = 0;
                continue;
            }

            for (col_idx, col) in line.split(' ').filter(|c| !c.is_empty()).enumerate() {
                let num = col.parse::<u32>().unwrap();
                board.rows[line_num][col_idx] = (num, false);
                board.sum += num;
            }

            line_num += 1;
        }

        boards.push(board);
        boards
    }

    fn new() -> Self {
        Board {
            sum: 0,
            rows: [[(0, false); BOARD_SIZE]; BOARD_SIZE],
        }
    }

    fn play(&mut self, draw: u32) -> bool {
        for row in &mut self.rows {
            for col in row {
                if col.0 == draw && !col.1 {
                    col.1 = true;
                    self.sum -= draw;
                }
            }
        }

        // check win condition
        for row_idx in 0..BOARD_SIZE {
            let mut row_all_true = true;
            for col_idx in 0..BOARD_SIZE {
                if !self.rows[row_idx][col_idx].1 {
                    row_all_true = false;
                    continue; // no point performing column check
                }

                // check column win condition
                if (0..BOARD_SIZE).all(|i| self.rows[i][col_idx].1) {
                    return true;
                }
            }

            if row_all_true {
                return true;
            }
        }

        false
    }
}
