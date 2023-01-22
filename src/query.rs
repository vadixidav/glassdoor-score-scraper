use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub enum Query {
    Employer(EmployerRequest, EmployerResponse),
    EmployerReviews(EmployerReviewsRequest, EmployerReviewsResponse),
}

impl Query {
    pub fn try_new(query: &str, body: Value) -> Option<Query> {
        let first_paren = query.find('(')?;
        let query_type = &query[..first_paren];
        let request = &query[first_paren + 1..query.len() - 1];
        match query_type {
            "employer" => Some(Query::Employer(
                serde_json::from_str(request).ok()?,
                serde_json::from_value(body).ok()?,
            )),
            "employerReviews" => Some(Query::EmployerReviews(
                serde_json::from_str(request).ok()?,
                serde_json::from_value(body).ok()?,
            )),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerRequest {
    #[serde(flatten)]
    _employer: Employer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerResponse {
    #[serde(rename = "__ref")]
    _ref: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerReviewsRequest {
    _apply_default_criteria: bool,
    _division: Division,
    _dynamic_profile_id: u64,
    _employer: Employer,
    _is_row_profile_enabled: bool,
    _language: String,
    _page: Page,
    _preferred_tld_id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerReviewsResponse {
    #[serde(rename = "__typename")]
    _typename: String,
    _all_reviews_count: u64,
    pub ratings: Ratings,
    _reviews: Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Employer {
    _id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Division {
    _id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    _num: u64,
    _size: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ratings {
    #[serde(rename = "__typename")]
    pub _typename: String,
    pub business_outlook_rating: f64,
    pub career_opportunities_rating: f64,
    pub ceo_rating: f64,
    pub ceo_ratings_count: u64,
    pub compensation_and_benefits_rating: f64,
    pub culture_and_values_rating: f64,
    pub diversity_and_inclusion_rating: f64,
    pub overall_rating: f64,
    pub _rated_ceo: Value,
    pub recommend_to_friend_rating: f64,
    pub review_count: u64,
    pub senior_management_rating: f64,
    pub work_life_balance_rating: f64,
}
