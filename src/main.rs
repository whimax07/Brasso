mod gen_csv;

fn main() {
    let image = gen_csv::wrap_csv_body("");
    print!("\n");
    println!("{}", image);
    print!("\n");
}
