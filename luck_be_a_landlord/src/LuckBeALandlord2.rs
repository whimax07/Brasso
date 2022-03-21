// Macros.
macro_rules! add_score {
    () => {};
}

macro_rules! loop_thought_2d_array {}

// Define the symbols set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: &'static str,
    pub display_char: char,
    pub base_value: i32,
    /// Note(Max): Using a function handle here means function calls are needed each time. This
    /// could be slow. Unfortunately I can't think of a (possibly) faster way (to use arrays?) that
    /// is as expressive and easy to catch errors.
    pub multi_map: fn(effector: Symbol) -> f32,
}

impl Symbol {
    pub const FIRST_SYMBOL: Symbol = Symbol::FLOWER;

    const FLOWER: Symbol = Symbol {
        name: "Flower",
        display_char: 'F',
        base_value: 1,
        multi_map: flower_multipliers,
    };

    const SUN: Symbol = Symbol {
        name: "Sun",
        display_char: 'S',
        base_value: 3,
        multi_map: sun_multipliers,
    };


    /// This function returns true if it carried aka loop around to the start of the symbol set.
    ///
    /// Note(Max): This is the best way to get the next symbol I have so far managed. If I can add
    // an index and an array of the symbols I can index into the array with the index.
    pub fn next_and_carry(&mut self) -> bool {
        match *self {
            Symbol::FLOWER => *self = Symbol::SUN,
            Symbol::SUN=> *self = Symbol::FIRST_SYMBOL,
            _ => panic!("Please use with a game symbol, one of the predefined const symbols.")
        }
        *self == Symbol::FIRST_SYMBOL
    }
}

fn flower_multipliers(symbol: Symbol) -> f32 {
    match symbol {
        Symbol::SUN => 5.,
        _ => 1.,
    }
}

fn sun_multipliers(_symbol: Symbol) -> f32 {
    1.
}


// Game code.
struct GameState {
    board: Board,
    best_board: Board,
    score: i128,
    best_score: i128,
}

impl GameState {
    const WIDTH: usize = 5;
    const HEIGHT: usize = 4;

    pub fn new() -> Self {
        Self {
            board: [[Symbol::FIRST_SYMBOL; GameState::WIDTH]; GameState::HEIGHT],
            best_board: [[Symbol::FIRST_SYMBOL; GameState::WIDTH]; GameState::HEIGHT],
            score: 0,
            best_score: 0
        }
    }

    pub fn next_board(&mut self) -> bool {
        for row in self.board.iter_mut() {
            for symbol in row.iter_mut() {
                if !symbol.next_and_carry() { return true }
            }
        }
        false
    }
}

type Board = [[Symbol; GameState::WIDTH]; GameState::HEIGHT];


pub fn run_solver() {
    // init the game boards.
    let mut game_state: GameState = GameState::new();
    while GameState::next_board(&mut game_state) {
        // Calculate the score of the board.
    }

    // Print the results.
}

fn calc_board_score() {

}

/// This is a direct way to calc the scores.
fn calc_symbol_contribution(game_state: GameState, symbol: Symbol, x: usize, y: usize) -> i128 {
    let mut multipler = 0.0;
    for row in &game_state.board[y-1..y+2] {
        for sym in &row[x-1..x+2] {
            println!("The sym is : {:?}", sym);
        }
    }
    (symbol.base_value as f64 * multipler) as i128
}


// ...
pub fn main() {
    run_solver();
    let mut x = Symbol::FLOWER;
    x.next_and_carry();
    println!("{:?}", x);
    x = Symbol::SUN;
    x.next_and_carry();
    println!("{:?}", x);
}

