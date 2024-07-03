use crate::structs::Args::ArgParser;
use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::collections::HashMap;
#[path = "./structs/mod.rs"]
mod structs;
#[path = "./utils/mod.rs"]
mod utils;

use utils::request_handler;
#[tokio::main]
async fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let mut text_elements: HashMap<String, Vec<String>> = HashMap::new();
    if args.recursive.is_some() {
        text_elements = utils::recurse::recurse(
            args.url,
            args.recursive.unwrap(),
            Some(utils::should_use_chrome::should_use_chrome(args.use_chrome)),
        )
        .await;
    } else {
        text_elements.insert(
            args.url.clone(),
            request_handler::get_text_elements_from_url(
                args.url,
                Some(utils::should_use_chrome::should_use_chrome(args.use_chrome)),
            )
            .await,
        );
    }
    let num_urls = text_elements.keys().len();
    let mut re = Regex::new(".*").unwrap();
    if args.search.is_some() {
        re = Regex::new(&args.search.unwrap()).unwrap();
    }
    for (link, texts) in text_elements.into_iter() {
        for text in texts {
            if re.is_match(&text) {
                if num_urls > 1 {
                    print!("{}", link.blue());
                    print!("{}", ":".green());
                }

                utils::prettyprint::prettyprint(text.to_string(), re.clone());
            }
        }
    }
}
