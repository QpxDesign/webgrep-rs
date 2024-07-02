use colored::Colorize;
use regex::Regex;

pub fn prettyprint(text: String, re: Regex) {
    // remove whitespace:
    let t = trim_whitespace(text.clone());
    let locs: Vec<(usize, usize)> = re
        .find_iter(&text)
        .map(|mat| (mat.start(), mat.end()))
        .collect();
    let mut i = 0;
    let mut color_map: Vec<bool> = vec![false; t.len()];
    while i < locs.len() {
        if locs.get(i).is_some() {
            let mut i2 = locs.get(i).unwrap().0;
            while i2 <= locs.get(i).unwrap().1 && i2 < color_map.len() {
                color_map[i2] = true;
                i2 += 1;
            }
        }
        i += 1;
    }

    let mut letter_index = 0;
    for l in t.split("") {
        if letter_index == color_map.len() {
            break;
        }
        if color_map[letter_index] {
            print!("{}", l.red());
        } else {
            print!("{}", l);
        }
        letter_index += 1;
    }
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
