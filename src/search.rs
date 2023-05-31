use cli_table::Table;
use serde_derive::Serialize;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Table)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub author: String,
    pub title: String,
    pub points: i64,
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

use reqwest;

pub fn search(keyword: &str) -> Result<SearchResult, reqwest::Error> {
    let query = format!("http://hn.algolia.com/api/v1/search?query={}", keyword);
    let http_response = reqwest::blocking::get(&query)?;
    let search_results = http_response.json::<SearchResult>()?;
    println!("{}", search_results.hits_per_page);
    Ok(search_results)
}
