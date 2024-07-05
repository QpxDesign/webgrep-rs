use headless_chrome::Browser;
use lazy_static::lazy_static;
use pdf_extract;
use scraper::{Html, Selector};
use url::Url;
lazy_static! {
    static ref CLIENT : reqwest::Client = reqwest::Client::builder().user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36").build().unwrap();
}

pub async fn get_text_elements_from_url(url: String, use_chrome: Option<bool>) -> Vec<String> {
    let text_selector = Selector::parse("p, h1, h2, h3, h4, h5, blockquote, dd, div, dl, dt, figcaption, figure, hr, li, menu, ol, p, pre, ul, a, abbr, b, bdi, bdo, br, cite, code, data, dfn, em, i, kbd, mark, q, rp, rt, ruby, s, samp, small, span, strong, sub, sup, time, u, var, wbr, caption, col, colgroup, table, tbody, td, tfoot, th, thead, tr, noscript, font").unwrap();
    let mut parsed_html = Html::new_document();
    if use_chrome.is_some() && use_chrome.unwrap() {
        parsed_html = browse_for_html_from_url(url.clone()).await;
    } else {
        parsed_html = get_html_from_url(&url).await;
    }
    let mut tmp: Vec<String> = Vec::new();
    if Url::parse(&url).is_err() {
        panic!("ERROR: '{}' is not a valid URL ", url);
    }

    for e in parsed_html.select(&text_selector) {
        if e.text().next().is_some() {
            tmp.push(e.text().next().unwrap().to_string());
        }
    }
    return tmp;
}
pub async fn get_html_from_url(url: &str) -> Html {
    let conn = CLIENT.get(url.clone()).send().await;
    if conn.is_err() {
        println!("Failed to Traverse {}", url);
        return Html::new_document();
    }
    let resp = conn.unwrap().text().await;
    if resp.is_err() {
        println!("Failed to Parse {}", url);
        return Html::new_document();
    }
    return Html::parse_document(&resp.unwrap());
}
pub async fn browse_for_html_from_url(url: String) -> Html {
    let browser =
        Browser::default().expect("Failed to create broswer (try running without --chrome)");

    let tab = browser
        .new_tab()
        .expect("Failed to create broswer tab (try running without --chrome)");
    tab.navigate_to(&url);
    let elem = tab.wait_for_element("body");
    if elem.is_err() {
        println!("[X] - Not Traversing {} - No Body Element", url);
        return Html::new_document();
    }
    let html = elem.unwrap().get_content();
    return Html::parse_document(&html.unwrap());
}

pub async fn read_pdf_from_url(url: String) -> Vec<String> {
    let mut v = Vec::new();
    let conn = CLIENT.get(url.clone()).send().await;
    if conn.is_err() {
        println!("Failed to Traverse {}", url);
        return v;
    }
    let resp = conn.unwrap().bytes().await;
    if resp.is_err() {
        println!("Failed to Parse {}", url);
        return v;
    }
    let out = pdf_extract::extract_text_from_mem(&resp.unwrap());
    v = out
        .expect(&format!("X] - Not Traversing {} - Could Not Read PDF", url).to_string())
        .split("\n")
        .map(|v| v.to_string())
        .collect();
    return v;
}
