use cli_table::Table;
use serde_derive::Serialize;
use serde_json::Value;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Table)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub author: String,
    pub title: String,
    pub url: String,
    pub text: Value,
    pub points: i64,
    #[serde(rename = "parent_id")]
    pub parent_id: Value,
    // pub children: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub hits: Vec<Item>,
    pub page: i64,
    pub nb_hits: i64,
    pub nb_pages: i64,
    pub hits_per_page: i64,
    pub processing_time_m_s: i64,
    pub query: String,
    pub params: String,
}

pub fn search(keyword: &String) -> Result<SearchResult, reqwest::Error> {
    let query = format!("http://hn.algolia.com/api/v1/search?query={}", keyword);
    let http_response = reqwest::blocking::get(query)?;
    let search_results: SearchResult = http_response.json::<SearchResult>()?;
    println!("{}", search_results.hits_per_page);
    Ok(search_results)
}