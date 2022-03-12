// A comment at the top.

use core::fmt;
use std::fmt::{Debug, Display, Formatter};


const WIDTH: usize = 5;
const HEIGHT: usize = 4;
const NUMBER_OF_SYMBOLS: u32 = Symbol::SUN as u32;


type Grid = [[Symbol; WIDTH]; HEIGHT];

trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl PrettyPrint for Grid {
    fn pretty_print(&self) -> String {
        let mut output = "".to_owned();
        for row in self {
            output += format!(
                "[{}, {}, {}, {}, {}]\n", row[0], row[1], row[2], row[3], row[4]
            ).as_str();
        }
        output
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Symbol {
    FLOWER{base: i32, multi: f32},
    SUN
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = match self {
            Symbol::FLOWER => {"F"}
            Symbol::SUN => {"S"}
        };
        write!(f, "{}", name)
    }
}


#[derive(Debug)]
struct GameState {
    grid: Grid,
    best_grid: Grid,
    score: u128,
    best_score: u128,
}

impl PrettyPrint for GameState {
    fn pretty_print(&self) -> String {
        format!(
            "\
            Game State: \n\
            Current Grid (Score = {}): \n\
            {}\
            Best Grid (Score = {}): \n\
            {} \
            ",
            self.score, self.grid.pretty_print(), self.best_score, self.best_grid.pretty_print()
        )
    }
}


fn set_up_sim() -> Box<GameState> {
    Box::new(GameState{
        grid: [[Symbol::FLOWER; WIDTH]; HEIGHT],
        best_grid: [[Symbol::FLOWER; WIDTH]; HEIGHT],
        score: 0,
        best_score: 0
    })
}

fn form_next_grid(game_state: &mut Box<GameState>) -> bool {
    for row in game_state.grid.iter_mut() {
        for symbol in row.iter_mut() {
            if *symbol == Symbol::SUN {
                continue;
            }
            // Note(Max): Add 1 (or similar) should go here.
            *symbol = Symbol::SUN;
            return true;
        }
    }

    return false;
}

fn run_sim(game_state: &mut Box<GameState>) {
    while form_next_grid(game_state) {
        println!("{}", game_state.grid.pretty_print());
        continue;
        todo!()
    }
}


pub fn main() {
    println!("LuckBeALandlord.rs, main(): Start.");
    let mut game_state = set_up_sim();
    run_sim(&mut game_state);
    println!("\nTest print: \n{}", game_state.pretty_print());
    println!("LuckBeALandlord, main(): End.");
}
