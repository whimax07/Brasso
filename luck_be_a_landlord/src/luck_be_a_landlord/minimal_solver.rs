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


fn board_to_string(board: Board) -> String {
    let mut board_str = "".to_string();
    for row in board.iter() {
        board_str.push_str(format!("{:?}\n", row).as_str());
    }
    board_str
}

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

macro_rules! get_multi {
    ($i:expr, $j:expr, $i_u:expr, $j_u:expr, $width:ident, $height:ident, $symbol:ident, $board:ident) => {
        if $i >= 0 && $i < $width && $j >= 0 && $j < $height {
            MULTI_MAP[$board[$j_u][$i_u]][*$symbol]
        } else {
            1.0
        }
    };
}

// 3.8S.
fn calc_board_score_macro(board: Board) -> i128 {
    let mut total_score = 0.0;
    let width = WIDTH as i128;
    let height = HEIGHT as i128;

    for (y, row) in board.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            let x_int = x as i128;
            let y_int = y as i128;

            let mut multiplier = 1.0;
            multiplier *= get_multi!(x_int - 1, y_int - 1,  x - 1,  y - 1,      width, height, symbol, board);
            multiplier *= get_multi!(x_int - 1, y_int,      x - 1,  y,          width, height, symbol, board);
            multiplier *= get_multi!(x_int - 1, y_int + 1,  x - 1,  y + 1,      width, height, symbol, board);

            multiplier *= get_multi!(x_int,     y_int - 1,  x,      y - 1,      width, height, symbol, board);
            multiplier *= get_multi!(x_int,     y_int + 1,  x,      y + 1,      width, height, symbol, board);

            multiplier *= get_multi!(x_int + 1, y_int - 1,  x + 1,  y - 1,      width, height, symbol, board);
            multiplier *= get_multi!(x_int + 1, y_int,      x + 1,  y,          width, height, symbol, board);
            multiplier *= get_multi!(x_int + 1, y_int + 1,  x + 1,  y + 1,      width, height, symbol, board);


            total_score += BASE_SCORE[*symbol] * multiplier;
        }
    }

    total_score as i128
}

// 3.8S
fn calc_board_score_inline(board: Board) -> i128 {
    let mut total_score = 0.0;
    let width = WIDTH as i128;
    let height = HEIGHT as i128;

    for (y, row) in board.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            let x_int = x as i128;
            let y_int = y as i128;

            let mut multiplier = 1.0;

            multiplier *= if x_int - 1 >= 0 && x_int - 1 < width && y_int - 1 >= 0 && y_int - 1 < height {
                MULTI_MAP[board[y - 1][x - 1]][*symbol]
            } else {
                1.0
            };

            multiplier *= if x_int - 1 >= 0 && x_int - 1 < width && y_int >= 0 && y_int < height {
                MULTI_MAP[board[y][x - 1]][*symbol]
            } else {
                1.0
            };

            multiplier *= if x_int - 1 >= 0 && x_int - 1 < width && y_int + 1 >= 0 && y_int + 1 < height {
                MULTI_MAP[board[y + 1][x - 1]][*symbol]
            } else {
                1.0
            };


            multiplier *= if x_int >= 0 && x_int - 1 < width && y_int - 1 >= 0 && y_int - 1 < height {
                MULTI_MAP[board[y - 1][x]][*symbol]
            } else {
                1.0
            };

            multiplier *= if x_int >= 0 && x_int < width && y_int + 1 >= 0 && y_int + 1 < height {
                MULTI_MAP[board[y + 1][x]][*symbol]
            } else {
                1.0
            };


            multiplier *= if x_int + 1 >= 0 && x_int + 1 < width && y_int - 1 >= 0 && y_int - 1 < height {
                MULTI_MAP[board[y - 1][x + 1]][*symbol]
            } else {
                1.0
            };

            multiplier *= if x_int + 1 >= 0 && x_int + 1 < width && y_int >= 0 && y_int < height {
                MULTI_MAP[board[y][x + 1]][*symbol]
            } else {
                1.0
            };

            multiplier *= if x_int + 1 >= 0 && x_int + 1 < width && y_int + 1 >= 0 && y_int + 1 < height {
                MULTI_MAP[board[y + 1][x + 1]][*symbol]
            } else {
                1.0
            };

            total_score += BASE_SCORE[*symbol] * multiplier;
        }
    }

    total_score as i128
}

// 4.1S.
fn calc_board_score(board: Board) -> i128 {
    let mut total_score = 0.0;
    let width = WIDTH as i128;
    let height = HEIGHT as i128;

    for (y, row) in board.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            let x_int = x as i128;
            let y_int = y as i128;

            let get_multi = |x_offset, y_offset| {
                let i: i128 = x_int + x_offset;
                let j: i128 = y_int + y_offset;

                if i >= 0 && i < width && j >= 0 && j < height {
                    MULTI_MAP[board[j as usize][i as usize]][*symbol]
                } else {
                    1.0
                }
            };

            let multiplier = 1.0
                * get_multi(1,-1)
                * get_multi(1, 0)
                * get_multi(1,1)

                * get_multi(0, -1)
                * get_multi(0, 1)

                * get_multi(-1, -1)
                * get_multi(-1, 0)
                * get_multi(-1, 1);

            total_score += BASE_SCORE[*symbol] * multiplier;
        }
    }

    total_score as i128
}

pub fn run_sim() {
    let mut board: Board = [[FLOWER; WIDTH]; HEIGHT];

    let mut best_score = calc_board_score_inline(board);
    while new_board(&mut board) {
        let board_score = calc_board_score_inline(board);
        if board_score > best_score {
            best_score = board_score;
            println!("High score: {}", best_score);
            println!("{}", board_to_string(board));
        }
    }

    println!("The best score is: {}.", best_score);
}
