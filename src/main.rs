use serde_json::Value;
use std::collections::HashMap;

fn main() {
    if let Some(root_body) = scrape(include_str!(
        "../data/Working-at-Amazon-EI_IE6036.11,17.htm"
    )) {
        for (query, body) in root_body {
            println!("{query}: {body}");
            println!();
        }
    }
}

fn scrape(page: &str) -> Option<HashMap<String, Value>> {
    let root_query_head = "\"ROOT_QUERY\":";
    page.find(root_query_head).and_then(|root_begin| {
        let root_body = root_begin + root_query_head.len();
        find_matching_curly(&page[root_body..])
            .and_then(|root_body| serde_json::from_str::<HashMap<String, Value>>(root_body).ok())
    })
}

fn find_matching_curly(s: &str) -> Option<&'_ str> {
    let mut num_curly = 1;
    if !s.starts_with('{') {
        return None;
    }
    for (ix, c) in s[1..].char_indices() {
        if c == '{' {
            num_curly += 1;
        } else if c == '}' {
            num_curly -= 1;
        }
        if num_curly == 0 {
            return Some(&s[..ix + 2]);
        }
    }
    None
}
