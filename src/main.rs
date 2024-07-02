use crate::structs::Args::ArgParser;
use clap::Parser;
use colored::Colorize;
use regex::Regex;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;

#[path = "./structs/mod.rs"]
mod structs;
#[path = "./utils/mod.rs"]
mod utils;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let text_selector = Selector::parse("p, h1, h2, h3, h4, h5, blockquote, dd, div, dl, dt, figcaption, figure, hr, li, menu, ol, p, pre, ul, a, abbr, b, bdi, bdo, br, cite, code, data, dfn, em, i, kbd, mark, q, rp, rt, ruby, s, samp, small, span, strong, sub, sup, time, u, var, wbr, caption, col, colgroup, table, tbody, td, tfoot, th, thead, tr").unwrap();

    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let resp = client
        .get(&args.url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut text_elements: HashMap<String, Vec<String>> = HashMap::new();
    if args.recursive.is_some() {
        text_elements = utils::recurse::recurse(args.url, args.recursive.unwrap()).await;
    } else {
        let parsed_html = Html::parse_document(&resp);
        let mut tmp: Vec<String> = Vec::new();
        for e in parsed_html.select(&text_selector) {
            if e.text().next().is_some() {
                tmp.push(e.text().next().unwrap().to_string());
            }
        }
        text_elements.insert(args.url, tmp);
    }

    let mut re = Regex::new(".*").unwrap();
    if args.search.is_some() {
        re = Regex::new(&args.search.unwrap()).unwrap();
    }
    for (link, texts) in text_elements.into_iter() {
        for text in texts {
            if re.is_match(&text) {
                print!("{}", link.blue());
                print!("{}", ":".green());
                utils::prettyprint::prettyprint(text.to_string(), re.clone());
            }
        }
    }
}
