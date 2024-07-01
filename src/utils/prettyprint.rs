pub fn prettyprint(text: String) {
    // remove whitespace:
    println!("{}", trim_whitespace(text));
}

fn trim_whitespace(text: String) -> String {
    let words = text.split_whitespace();
    let mut out: String = "".to_string();
    for word in words.collect::<Vec<_>>() {
        out += word;
        out += " ";
    }
    return out;
}
