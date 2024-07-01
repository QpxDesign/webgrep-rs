use crate::structs::Args::ArgParser;
use clap::Parser;
use regex::Regex;
use scraper::{Html, Selector};

#[path = "./structs/mod.rs"]
mod structs;
#[path = "./utils/mod.rs"]
mod utils;

fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let resp = reqwest::blocking::get(&args.url).unwrap().text().unwrap();
    let parsed_html = Html::parse_document(&resp);
    let text_selector = Selector::parse("*").unwrap();

    let mut text_elements: Vec<structs::Element::Element> = Vec::new();
    if args.recursive.is_some() && args.recursive.unwrap() {
        text_elements = utils::recurse::recurse(args.url, 3);
    } else {
        for e in parsed_html.select(&text_selector) {
            if e.text().next().is_some() {
                text_elements.push(structs::Element::Element {
                    text: e.text().next().unwrap().to_string(),
                    from_url: args.url.clone(),
                });
            }
        }
    }

    let re = Regex::new(&args.search.unwrap()).unwrap();
    for text in text_elements {
        if re.is_match(&text.text) {
            utils::prettyprint::prettyprint(text.text);
        }
    }
}
