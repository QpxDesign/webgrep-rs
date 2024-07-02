use crate::structs::Args::ArgParser;
use clap::Parser;
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
    let mut d: i8 = 1;
    if args.recursive.is_some() {
        d = args.recursive.unwrap();
    }
    text_elements = utils::recurse::recurse(args.url, d).await;
    let mut re = Regex::new(".*").unwrap();
    if args.search.is_some() {
        re = Regex::new(&args.search.unwrap()).unwrap();
    }
    for urls in text_elements.values().collect::<Vec<_>>() {
        for text in urls {
            if re.is_match(&text) {
                utils::prettyprint::prettyprint(text.to_string(), re.clone());
            }
        }
    }
}
