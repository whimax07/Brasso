// Define the symbols set.
#[derive(Debug, PartialEq, Eq)]
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

    pub const FLOWER: Symbol = Symbol {
        name: "Flower",
        display_char: 'F',
        base_value: 1,
        multi_map: flower_multipliers,
    };

    pub const SUN: Symbol = Symbol {
        name: "Sun",
        display_char: 'S',
        base_value: 3,
        multi_map: sun_multipliers,
    };


    /// Note(Max): This is the best way to get the next symbol I have so far managed. If I can add
    /// an index and an array of the symbols I can index into the array with the index.
    pub fn next(self) -> Option<Symbol> {
        return match self {
            Symbol::FLOWER => Some(Symbol::SUN),
            Symbol::SUN=> None,
            _ => panic!("Please use one of the predefined Symbols.")
        }
    }
}

fn flower_multipliers(symbol: Symbol) -> f32 {
    return match symbol {
        Symbol::SUN => 5.,
        _ => 1.,
    }
}

fn sun_multipliers(_symbol: Symbol) -> f32 {
    1.
}


// ...
pub fn main() {
    println!("{:?}", Symbol::FLOWER.next());
    println!("{:?}", Symbol::SUN.next());
}

