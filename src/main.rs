use linked_hash_map::LinkedHashMap;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
enum Query {
    Employer(EmployerQueryRequest, EmployerQueryResponse),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmployerQueryRequest {
    _id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmployerQueryResponse {
    #[serde(rename = "__ref")]
    _ref: Value,
}

fn main() {
    if let Some(root_body) = scrape(include_str!(
        "../data/Working-at-Amazon-EI_IE6036.11,17.htm"
    )) {
        let queries: Vec<Query> = root_body
            .into_iter()
            .filter_map(|(query, body)| query_extractor(&query, body))
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

fn query_extractor(query: &str, body: Value) -> Option<Query> {
    let first_paren = query.find('(')?;
    let query_type = &query[..first_paren];
    let request = &query[first_paren + 1..query.len() - 1];
    match query_type {
        "employer" => Some(Query::Employer(
            serde_json::from_str(request).ok()?,
            serde_json::from_value(body).ok()?,
        )),
        _ => None,
    }
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
