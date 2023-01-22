mod query;

use linked_hash_map::LinkedHashMap;
use query::{EmployerReviewsResponse, Query};
use serde_json::Value;

fn main() {
    for (stock, ratings) in [
        include_str!("../data/Working-at-Amazon-EI_IE6036.11,17.htm"),
        include_str!("../data/Working-at-Google-EI_IE9079.11,17.htm"),
    ]
    .into_iter()
    .filter_map(|page| scrape(page))
    .filter_map(|(stock, root_body)| {
        Some((
            stock,
            root_body
                .into_iter()
                .filter_map(|(query, body)| Query::try_new(&query, body))
                .find_map(|query| match query {
                    Query::EmployerReviews(_, EmployerReviewsResponse { ratings, .. }) => {
                        Some(ratings)
                    }
                    _ => None,
                })?,
        ))
    }) {
        println!("{stock}: {ratings:?}");
    }
}

fn scrape(page: &str) -> Option<(String, LinkedHashMap<String, Value>)> {
    let root_query_head = "\"ROOT_QUERY\":";
    let root_begin = page.find(root_query_head)?;
    let root_body = root_begin + root_query_head.len();
    let root_body = find_matching_curly(&page[root_body..])?;
    let root_body = serde_json::from_str::<LinkedHashMap<String, Value>>(root_body).ok()?;

    let stock_head = "\"stock\":";
    let stock_start = page.find(stock_head)? + stock_head.len();
    let stock_end = stock_start + page[stock_start..].find(',')?;
    let stock_body = &page[stock_start..stock_end];
    let stock_body = serde_json::from_str::<String>(stock_body).ok()?;

    Some((stock_body, root_body))
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
