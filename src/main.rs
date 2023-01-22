mod query;

use std::time::Duration;

use linked_hash_map::LinkedHashMap;
use query::{EmployerReviewsResponse, Query};
use reqwest::Client;
use serde_json::Value;

// https://www.glassdoor.com/Explore/browse-companies.htm?overall_rating_low=3&page=6&locId=1&locType=N&locName=United%20States&filterType=RATING_OVERALL

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("utf-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
}

#[tokio::main]
async fn main() {
    let client = Client::new();

    for page in 1..3 {
        match download_top_companies_page(&client, page).await {
            Ok(page) => {
                let top_companies = scrape_top_companies(&page);
                println!("top companies: {top_companies:?}");
                for company in top_companies {
                    let url = "https://www.glassdoor.com".to_owned() + &company;
                    println!("scraping company {url}");
                    match download_page(&client, &url).await {
                        Ok(page) => {
                            if let Some((stock, ratings)) =
                                scrape_working_at(&page).and_then(|(stock, root_body)| {
                                    Some((
                                        stock,
                                        root_body
                                            .into_iter()
                                            .filter_map(|(query, body)| {
                                                Query::try_new(&query, body)
                                            })
                                            .find_map(|query| match query {
                                                Query::EmployerReviews(
                                                    _,
                                                    EmployerReviewsResponse { ratings, .. },
                                                ) => Some(ratings),
                                                _ => None,
                                            })?,
                                    ))
                                })
                            {
                                println!("{stock}: {ratings:?}");
                            }
                        }
                        Err(e) => {
                            eprintln!("failed to download company: {e}");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("failed to download page {page}: {e}");
            }
        }
    }
}

/// Download one page of glassdoor top companies.
async fn download_top_companies_page(client: &Client, page: u32) -> Result<String, Error> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    let response = client
        .get("https://www.glassdoor.com/Explore/browse-companies.htm")
        .query(&[
            ("overall_rating_low", "3"),
            ("page", &format!("{page}")),
            ("locId", "1"),
            ("locType", "N"),
            ("locName", "United States"),
            ("filterType", "RATING_OVERALL"),
        ])
        .send()
        .await?;
    let bytes = response.bytes().await?;
    Ok(std::str::from_utf8(bytes.as_ref())?.to_owned())
}

/// Download page.
async fn download_page(client: &Client, url: &str) -> Result<String, Error> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    let response = client.get(url).send().await?;
    let bytes = response.bytes().await?;
    Ok(std::str::from_utf8(bytes.as_ref())?.to_owned())
}

/// Extract URLs from top companies.
fn scrape_top_companies(page: &str) -> Vec<String> {
    let overview_url_head = "\"overviewUrl\":";
    page.match_indices(overview_url_head)
        .filter_map(|(overview_tag, _)| {
            let overview_url_body = overview_tag + overview_url_head.len();
            let overview_url_body_end =
                page[overview_url_body + 1..].find('"')? + overview_url_body + 2;
            let overview_url_body = &page[overview_url_body..overview_url_body_end];
            serde_json::from_str::<String>(overview_url_body).ok()
        })
        .collect()
}

fn scrape_working_at(page: &str) -> Option<(String, LinkedHashMap<String, Value>)> {
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
