macro_rules! nested_1 {
    ($vector:ident, ($i:expr, $j:expr)) => {
        let total = $i * $j;
        $vector.push(total);
    };

    ($vector:expr, $offset:literal with -> $($inputs:tt),+) => {{
        let mut total: i32 = 0;
        let mut vector = $vector;
        $( nested_1!(vector, $inputs); )+
        total + $offset
    }};
}

macro_rules! nested_2 {
    ($vector:ident, ($i:expr, $j:expr)) => {
        let total = $i * $j;
        $vector.push(total);
    };
}


pub fn test_macros() {
    let mut vec = vec![0];
    vec.push(10);
    nested_1!(&mut vec, 1000 with -> (5, 5), (10, 10));
    println!("{:?}", vec);
}
