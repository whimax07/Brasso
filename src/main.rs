use crate::gen_csv::wrap_csv_body;


mod gen_csv;


fn main() {
    let image = wrap_csv_body("");
    print!("{}", image);
}
