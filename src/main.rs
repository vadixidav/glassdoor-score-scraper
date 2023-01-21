mod query;

use linked_hash_map::LinkedHashMap;
use query::Query;
use serde_json::Value;

fn main() {
    if let Some(root_body) = scrape(include_str!(
        "../data/Working-at-Amazon-EI_IE6036.11,17.htm"
    )) {
        let queries: Vec<Query> = root_body
            .into_iter()
            .filter_map(|(query, body)| Query::try_new(&query, body))
            .collect();
        for query in queries {
            println!("{query:?}");
        }
    }
}

fn scrape(page: &str) -> Option<LinkedHashMap<String, Value>> {
    let root_query_head = "\"ROOT_QUERY\":";
    let root_begin = page.find(root_query_head)?;
    let root_body = root_begin + root_query_head.len();
    let root_body = find_matching_curly(&page[root_body..])?;
    serde_json::from_str::<LinkedHashMap<String, Value>>(root_body).ok()
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
