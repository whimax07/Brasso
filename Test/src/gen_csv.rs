pub fn wrap_csv_body(body: &str) -> String {
    let mut header = String::from(get_header());
    header.push_str(body);
    header.push_str(get_footer());
    header
}

fn get_header() -> &'static str {
    let header = "<svg height=\"200\" width=\"200\" xmlns=\"http://www.w3.org/2000/svg\">\n";
    header
}

fn get_footer() -> &'static str {
    let footer = "</svg>";
    footer
}
