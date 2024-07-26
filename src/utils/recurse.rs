use scraper::{Html, Selector};
use url::Url;
#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::Args::ArgParser;
use crate::utils::request_handler;
use async_recursion::async_recursion;
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::time::{sleep, Duration}; // TODO: IMPLEMENT
lazy_static! {
    // key: href
    // value: html
    static ref CACHE: Mutex<HashMap<String,Vec<String>>> = Mutex::new(HashMap::new());
}
pub async fn recurse(
    base_url: String,
    depth: i8,
    use_chrome: Option<bool>,
) -> HashMap<String, Vec<String>> {
    #[async_recursion(?Send)]
    async fn get_links(
        specific_urls: Vec<String>,
        depth: i8,
        use_chrome: Option<bool>,
    ) -> Vec<String> {
        let args: crate::structs::Args::ArgParser = ArgParser::parse();
        if specific_urls.len() < 1 {
            return Vec::new();
        }
        if Url::parse(&specific_urls[0]).is_err() {
            return *Box::new(Vec::new());
        }
        if depth < 1 {
            return specific_urls;
        }
        let mut links: Vec<String> = Vec::new();

        for specific_url in specific_urls {
            if Url::parse(&specific_url).is_err() {
                continue;
            }
            let parsed_url = Url::parse(&specific_url).unwrap();
            let link_selector = Selector::parse("a").unwrap();
            let mut phtml = Html::new_document();
            if use_chrome.is_some() && use_chrome.unwrap() {
                phtml = request_handler::browse_for_html_from_url(specific_url.clone()).await;
            } else {
                phtml = request_handler::get_html_from_url(&specific_url).await;
            }
            for l in phtml.select(&link_selector) {
                if l.attr("href").is_some()
                    && is_valid_href(
                        l.attr("href").unwrap().to_string(),
                        &args.samehost,
                        args.pathcontains.clone(),
                        &parsed_url,
                    )
                {
                    let mut href = l.attr("href").unwrap().to_string();

                    if Url::parse(&href) == Err(url::ParseError::RelativeUrlWithoutBase) {
                        href = parsed_url.join(&href).unwrap().as_str().to_string();
                    }
                    if args.break_when_found.is_some() && args.break_when_found.unwrap() {
                        let mut re = Regex::new("").unwrap();
                        if args.search.clone().is_some() {
                            if args.insensitive.is_some() && args.insensitive.unwrap() == true {
                                re = Regex::new(
                                    &("(?i)".to_owned() + &args.search.clone().unwrap()),
                                )
                                .unwrap();
                            } else {
                                re = Regex::new(&args.search.clone().unwrap()).unwrap();
                            }
                        }
                        for t in CACHE.lock().unwrap().values_mut() {
                            for a in t {
                                if re.is_match(a) {
                                    return get_links(links, depth - 1, use_chrome).await;
                                }
                            }
                        }
                    }
                    if !CACHE.lock().unwrap().contains_key(&href) && !Url::parse(&href).is_err() {
                        let mut file_type = "html";
                        if crate::utils::determine_file_type::determine_file_type(&href) == "pdf" {
                            CACHE.lock().unwrap().insert(
                                href.clone(),
                                request_handler::read_pdf_from_url(href.clone()).await,
                            )
                        } else {
                            CACHE.lock().unwrap().insert(
                                href.clone(),
                                request_handler::get_text_elements_from_url(
                                    href.clone(),
                                    use_chrome,
                                )
                                .await,
                            )
                        };
                        links.push(href.clone());
                        println!("{}", "Traversed ".to_owned() + &href.to_owned());
                    }
                }
            }
        }
        return get_links(links, depth - 1, use_chrome).await;
    }
    let mut a: Vec<String> = Vec::new();
    a.push(base_url);
    let _links = get_links(a, depth, use_chrome).await;
    return CACHE.lock().unwrap().to_owned();
}

fn is_valid_href(
    l: String,
    samehost: &Option<bool>,
    pathcontains: Option<String>,
    parsed_url: &Url,
) -> bool {
    let mut href = l.clone();
    if Url::parse(&href) == Err(url::ParseError::RelativeUrlWithoutBase) {
        if parsed_url.join(&href).is_err() {
            return false;
        }
        href = parsed_url.join(&href).unwrap().as_str().to_string();
    }
    if !l.contains("/") {
        return false;
    }
    if pathcontains.is_some() && !l.contains(&pathcontains.unwrap()) {
        return false;
    }
    if samehost.is_some() && samehost.unwrap() == true {
        if parsed_url.host().is_none() {
            return false;
        }
        if Url::parse(&href).is_err() {
            return false;
        }
        if Url::parse(&href).unwrap().host().is_none() {
            return false;
        }
        if parsed_url.host().unwrap().to_string()
            != Url::parse(&href).unwrap().host().unwrap().to_string()
        {
            println!(
                "[X] - Not Traversing {} due to samehost being true",
                Url::parse(&href).unwrap().host().unwrap().to_string()
            );
            return false;
        }
    }
    return true;
}
