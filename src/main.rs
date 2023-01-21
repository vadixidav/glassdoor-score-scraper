fn main() {
    for finding in scrape(include_str!(
        "../data/Working-at-Amazon-EI_IE6036.11,17.htm"
    )) {
        println!("{finding}");
        println!();
        println!();
    }
}

fn scrape(page: &str) -> impl Iterator<Item = String> + '_ {
    page.match_indices("\"employerReviews({")
        .filter_map(|(start, _)| {
            page[start..].find("})\"").map(|e| {
                let end = e + start + 3;
                page[start..end].to_owned()
            })
        })
        .filter_map(|s| serde_json::from_str::<String>(&s).ok())
}
