use serde::Deserialize;
use serde_json::Value;

#[derive(Debug)]
pub enum Query {
    Employer(EmployerQueryRequest, EmployerQueryResponse),
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
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerQueryRequest {
    _id: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmployerQueryResponse {
    #[serde(rename = "__ref")]
    _ref: Value,
}
