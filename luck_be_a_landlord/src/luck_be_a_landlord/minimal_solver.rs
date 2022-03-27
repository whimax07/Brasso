use std::cmp::max;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

type Board = [[usize; WIDTH]; HEIGHT];

const FIRST_SYMBOL: usize = 0;
#[allow(dead_code)]
const FLOWER: usize = 0;
#[allow(dead_code)]
const SUN: usize = 1;
const NUM_OF_SYMBOLS: usize = 2;

const BASE_SCORE: [f64; NUM_OF_SYMBOLS] = [1.0, 3.0];

const MULTI_MAP: [[f64; NUM_OF_SYMBOLS]; NUM_OF_SYMBOLS] = [
    [1.0, 1.0],
    [5.0, 1.0],
];


fn new_board(board: &mut Board) -> bool {
    for row in board.iter_mut() {
        for symbol in row.iter_mut() {
            *symbol += 1;
            if *symbol < NUM_OF_SYMBOLS {
                return true
            };
            *symbol = FIRST_SYMBOL;
        }
    }

    false
}

fn calc_board_score(board: Board) -> i128 {
    let mut total_score = 0.0;
    let width = WIDTH as i128;
    let height = HEIGHT as i128;

    for (y, row) in board.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            let x_int = x as i128;
            let y_int = y as i128;

            let get_multi = |i, j| {
                if i >= 0 && i < width && j >= 0 && j < height {
                    MULTI_MAP[*symbol][board[j as usize][i as usize]]
                } else {
                    1.0
                }
            };

            let multiplier = 1.0
                * get_multi(x_int - 1, y_int - 1)
                * get_multi(x_int - 1, y_int)
                * get_multi(x_int - 1, y_int + 1)
                * get_multi(x_int, y_int - 1)
                * get_multi(x_int, y_int + 1)
                * get_multi(x_int + 1, y_int - 1)
                * get_multi(x_int + 1, y_int)
                * get_multi(x_int + 1, y_int + 1);

            total_score += BASE_SCORE[*symbol] * multiplier;
        }
    }

    total_score as i128
}

pub fn run_sim() {
    let mut board: Board = [[FLOWER; WIDTH]; HEIGHT];

    let mut best_score = calc_board_score(board);
    while new_board(&mut board) {
        let board_score = calc_board_score(board);
        best_score = max(best_score, board_score);
    }

    println!("The best score is: {}.", best_score);
}
