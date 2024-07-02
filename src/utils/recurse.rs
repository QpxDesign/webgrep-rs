use scraper::{Html, Selector};
use url::Url;
#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::Args::ArgParser;
use async_recursion::async_recursion;
use clap::Parser;
use lazy_static::lazy_static;
use reqwest::Client;
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::time::{sleep, Duration}; // TODO: IMPLEMENT

lazy_static! {
    // key: href
    // value: html
    static ref CLIENT : reqwest::Client = Client::new();
    static ref CACHE: Mutex<HashMap<String,Vec<String>>> = Mutex::new(HashMap::new());
}
pub async fn recurse(base_url: String, depth: i8) -> HashMap<String, Vec<String>> {
    #[async_recursion(?Send)]
    async fn get_links(specific_urls: Vec<String>, depth: i8) -> Vec<String> {
        let text_selector = Selector::parse("p, h1, h2, h3, h4, h5, blockquote, dd, div, dl, dt, figcaption, figure, hr, li, menu, ol, p, pre, ul, a, abbr, b, bdi, bdo, br, cite, code, data, dfn, em, i, kbd, mark, q, rp, rt, ruby, s, samp, small, span, strong, sub, sup, time, u, var, wbr, caption, col, colgroup, table, tbody, td, tfoot, th, thead, tr").unwrap();
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
            let parsed_url = Url::parse(&specific_url).unwrap();
            let link_selector = Selector::parse("a").unwrap();
            let resp = CLIENT
                .get(specific_url.clone())
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let phtml = Html::parse_document(&resp);
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

                    if !href.contains("http") {
                        href = parsed_url.scheme().to_string()
                            + "://"
                            + &parsed_url.host().unwrap().to_string()
                            + &href;
                    }
                    if !CACHE.lock().unwrap().contains_key(&href) {
                        let s_resp = reqwest::get(href.clone())
                            .await
                            .unwrap()
                            .text()
                            .await
                            .unwrap();

                        let s_html = Html::parse_document(&s_resp);
                        let mut text_elements: Vec<String> = Vec::new();
                        for e in s_html.select(&text_selector) {
                            if e.text().next().is_some() {
                                text_elements.push(e.text().next().unwrap().to_string());
                            }
                        }
                        CACHE.lock().unwrap().insert(href.clone(), text_elements);
                        links.push(href.clone());
                        println!("{}", "Traversed ".to_owned() + &href.to_owned());
                    }
                }
            }
        }
        return get_links(links, depth - 1).await;
    }
    let mut a: Vec<String> = Vec::new();
    a.push(base_url);
    let _links = get_links(a, depth).await;
    return CACHE.lock().unwrap().to_owned();
}

fn is_valid_href(
    l: String,
    samehost: &Option<bool>,
    pathcontains: Option<String>,
    parsed_url: &Url,
) -> bool {
    let mut href = l.clone();
    if !l.contains("http") {
        href =
            parsed_url.scheme().to_string() + "://" + &parsed_url.host().unwrap().to_string() + &l;
    }

    if !l.contains("/") {
        return false;
    }
    if pathcontains.is_some() && !l.contains(&pathcontains.unwrap()) {
        return false;
    }
    if samehost.is_some()
        && samehost.unwrap() == true
        && parsed_url.host().unwrap().to_string()
            != Url::parse(&href).unwrap().host().unwrap().to_string()
    {
        return false;
    }
    return true;
}
