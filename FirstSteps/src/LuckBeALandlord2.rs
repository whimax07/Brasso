struct Symbol {
    name: str,
    display_char: char,
    base_value: i32,
    multipliers: [f32; SYMBOLS.len()],
}

const SYMBOLS: [Symbol; 2] = [
    Symbol { name: *"Flower", display_char: 'F', base_value: 1, multipliers: [1., 1.] },
    Symbol { name: *"Sun", display_char: 'S', base_value: 3, multipliers: [5., 1.] },
];
