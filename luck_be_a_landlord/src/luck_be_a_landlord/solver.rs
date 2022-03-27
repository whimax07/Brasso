use crate::luck_be_a_landlord::calculate_score::calc_symbol_contribution_by_mixed;


// Macros.
macro_rules! mut_for_2d {
    ($sym:ident in $array:expr => $code:block) => {
        for row in $array.iter_mut() {
            for $sym in row.iter_mut() {
                $code
            }
        }
    };

    ($sym:ident in $array:expr => $code:stmt) => {
        mut_for_2d!($sym in $array => { $code })
    };
}


// Define the symbols set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub name: &'static str,
    pub display_char: char,
    pub base_value: i32,
    /// Note(Max): Using a function handle here means function calls are needed each time. This
    /// could be slow. Unfortunately I can't think of a (possibly) faster way (to use arrays?) that
    /// is as expressive and easy to catch errors.
    pub multi_map: fn(effector: Symbol) -> f64,
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
    /// an index and an array of the symbols I can index into the array with the index.
    pub fn next_and_carry(&mut self) -> bool {
        match *self {
            Symbol::FLOWER => *self = Symbol::SUN,
            Symbol::SUN=> *self = Symbol::FIRST_SYMBOL,
            _ => panic!("Please use with a game symbol, one of the predefined const symbols.")
        }
        *self == Symbol::FIRST_SYMBOL
    }
}

fn flower_multipliers(symbol: Symbol) -> f64 {
    match symbol {
        Symbol::SUN => 5.,
        _ => 1.,
    }
}

fn sun_multipliers(_symbol: Symbol) -> f64 {
    1.
}


// Game structure.
pub struct GameState {
    pub(crate) board: Board,
    best_board: Board,
    best_score: i128,
}

impl GameState {
    pub(crate) const WIDTH: usize = 5;
    pub(crate) const HEIGHT: usize = 4;

    pub fn new() -> Self {
        Self {
            board: [[Symbol::FIRST_SYMBOL; GameState::WIDTH]; GameState::HEIGHT],
            best_board: [[Symbol::FIRST_SYMBOL; GameState::WIDTH]; GameState::HEIGHT],
            best_score: 0,
        }
    }

    pub fn next_board(&mut self) -> bool {
        mut_for_2d!(symbol in self.board => if !symbol.next_and_carry() { return true } );
        false
    }
}

pub type Board = [[Symbol; GameState::WIDTH]; GameState::HEIGHT];


// Method code.
pub fn run_solver() {
    // init the game boards.
    let mut game_state: GameState = GameState::new();
    while GameState::next_board(&mut game_state) {
        // Calculate the score of the board.
        let score = calc_board_score(&mut game_state);
        if score > game_state.best_score {
            game_state.best_score = score;
        }
    }

    // Print the results.
    println!("The high score is: {}", game_state.best_score);
}

fn calc_board_score(game_state: &mut GameState) -> i128 {
    let mut score: i128 = 0;
    for (y, row) in game_state.board.iter().enumerate() {
        for (x, symbol) in row.iter().enumerate() {
            score += calc_symbol_contribution_by_mixed(game_state, symbol, x as i32, y as i32);
        }
    }
    score
}