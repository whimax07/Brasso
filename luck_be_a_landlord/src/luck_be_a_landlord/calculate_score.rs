use crate::luck_be_a_landlord::solver::{Board, GameState, Symbol};


macro_rules! calc_multiplier {
    ($array:expr, $symbol:ident, $width:expr, $height:expr, $x:expr, $y:expr) => {{
        let mut multipler: f64 = 1.0;
        if ($x >= 0 && $x < $width && $y >= 0 && $y < $height) {
            let get_multi = $symbol.multi_map;
            multipler *= get_multi($array[$y as usize][$x as usize]);
        }
        multipler
    }};

    ($array:expr, $symbol:ident, $width:expr, $height:expr, with offsets: $(($x:expr, $y:expr));+) => {{
        let mut multipler: f64 = 1.0;
        $( multipler *= calc_multiplier!($array, $symbol, $width, $height, $x, $y); )+
        // $( println!("symbol:{:?}, width:{:?}, height:{:?}, x:{:?} and y:{:?}.", $symbol, $width, $height, $x, $y); )+
        multipler
    }};
}

pub fn calc_symbol_contribution_macro(game_state: &GameState, symbol: &Symbol, x: i32, y: i32)
                                      -> i128 {
    let multiplier = calc_multiplier!(
        game_state.board, symbol, GameState::WIDTH as i32, GameState::HEIGHT as i32, with offsets:
        (x - 1, y - 1);
        (x - 1, y);
        (x - 1, y + 1);
        (x, y - 1);
        (x, y + 1);
        (x + 1, y - 1);
        (x + 1, y);
        (x + 1, y + 1)
    );
    (symbol.base_value as f64 * multiplier) as i128
}


pub fn calc_symbol_contribution_closures(game_state: &GameState, symbol: &Symbol, x: i32, y: i32)
                                         -> i128 {
    let mut multiplier: f64 = 1.;
    let board: Board = game_state.board;
    let multiplier_getter = symbol.multi_map;

    let _in_range = |i, j| {
        i >= 0 && i < GameState::WIDTH as i32 && j >= 0 && j < GameState::HEIGHT as i32
    };

    let mut _check_contribution = |i, j| {
        if _in_range(i, j) {
            let effector_row: [Symbol; GameState::WIDTH] = board[j as usize];
            multiplier *= multiplier_getter(effector_row[i as usize]);
        }
    };

    _check_contribution(x - 1, y - 1);
    _check_contribution(x - 1, y);
    _check_contribution(x - 1, y + 1);

    _check_contribution(x, y - 1);
    _check_contribution(x, y + 1);

    _check_contribution(x + 1, y - 1);
    _check_contribution(x + 1, y);
    _check_contribution(x + 1, y + 1);

    (symbol.base_value as f64 * multiplier) as i128
}

macro_rules! check_range {
    ($i:expr, $j:expr) => {
        $i >= 0 && $i < GameState::WIDTH as i32 && $j >= 0 && $j < GameState::HEIGHT as i32
    };
}

pub fn calc_symbol_contribution_by_mixed(game_state: &GameState, symbol: &Symbol, x: i32, y: i32)
                                         -> i128 {
    let mut multiplier: f64 = 1.;
    let board: Board = game_state.board;
    let multiplier_getter = symbol.multi_map;

    let _get_symbol_i32 = |i: i32, j: i32| board[j as usize][i as usize];

    if check_range!(x - 1, y - 1) {
        multiplier *= multiplier_getter(_get_symbol_i32(x - 1, y - 1));
    }

    if check_range!(x - 1, y) {
        multiplier *= multiplier_getter(_get_symbol_i32(x - 1, y));
    }

    if check_range!(x - 1, y + 1) {
        multiplier *= multiplier_getter(_get_symbol_i32(x - 1, y + 1));
    }

    if check_range!(x, y - 1) {
        multiplier *= multiplier_getter(_get_symbol_i32(x, y - 1));
    }

    if check_range!(x, y + 1) {
        multiplier *= multiplier_getter(_get_symbol_i32(x, y + 1));
    }

    if check_range!(x + 1, y - 1) {
        multiplier *= multiplier_getter(_get_symbol_i32(x + 1, y - 1));
    }

    if check_range!(x + 1, y) {
        multiplier *= multiplier_getter(_get_symbol_i32(x + 1, y));
    }

    if check_range!(x + 1, y + 1) {
        multiplier *= multiplier_getter(_get_symbol_i32(x + 1, y + 1));
    }

    (symbol.base_value as f64 * multiplier) as i128
}
