use scraper::{Html, Selector};
use std::process::Command;
use url::Url;
#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::Element::Element;

pub fn recurse(url: String, depth: i8) -> Vec<Element> {
    let parsed_url = Url::parse(&url).unwrap();
    let link_selector = Selector::parse("a").unwrap();
    let text_selector = Selector::parse("p, h1, h2, h3, h4, h5, blockquote, dd, div, dl, dt, figcaption, figure, hr, li, menu, ol, p, pre, ul, a, abbr, b, bdi, bdo, br, cite, code, data, dfn, em, i, kbd, mark, q, rp, rt, ruby, s, samp, small, span, strong, sub, sup, time, u, var, wbr, caption, col, colgroup, table, tbody, td, tfoot, th, thead, tr").unwrap();
    let resp = reqwest::blocking::get(url.clone()).unwrap().text().unwrap();
    let html = Html::parse_document(&resp);

    let mut links: Vec<String> = Vec::new();
    for l in html.select(&link_selector) {
        if l.attr("href").is_none() {
            continue;
        }
        let href = l.attr("href").unwrap().to_string();
        if !href.contains("/") {
            continue;
        }
        if !href.contains("http") {
            links.push(
                parsed_url.scheme().to_string()
                    + "://"
                    + &parsed_url.host().unwrap().to_string()
                    + &href,
            );
        } else {
            links.push(href);
        }
    }
    let mut text_elements: Vec<Element> = Vec::new();
    for e in html.select(&text_selector) {
        if e.text().next().is_some() {
            text_elements.push(Element {
                text: e.text().next().unwrap().to_string(),
                from_url: url.clone(),
            });
        }
    }
    for l in links {
        if depth - 1 > 0 {
            text_elements.append(&mut recurse(l.clone(), depth - 1));
        }
        // TODO: add progress bar/indicate that something is happening
        let resp = reqwest::blocking::get(l.clone()).unwrap().text().unwrap();
        let parsed_html = Html::parse_document(&resp);
        for e in parsed_html.select(&text_selector) {
            if e.text().next().is_some() {
                text_elements.push(Element {
                    text: e.text().next().unwrap().to_string(),
                    from_url: l.clone(),
                });
            }
        }
        // TODO: allow user to set delay
        let mut child = Command::new("sleep").arg("0.1").spawn().unwrap();
        let _result = child.wait().unwrap();
    }
    return text_elements;
}
