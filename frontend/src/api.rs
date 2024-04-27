use reqwest::Client;
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dish {
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>
}

#[derive(Clone)]
pub enum SummarizeError {
    InvalidUrl,
    BadResponse
}

pub async fn summarize(url: &str) -> Result<Vec<Dish>, SummarizeError> {
    let url = parse_url(url)?;
    let video_id = trim_to_video_id(url)?;

    let client = Client::new();
    let endpoint = "http://127.0.0.1:8081/summarize";
    let params = [("video_id", video_id)];
    
    let response = client.post(endpoint)
        .form(&params)
        .send()
        .await
        .map_err(|_| {SummarizeError::BadResponse})?;

    if !response.status().is_success() {
        return Err(SummarizeError::BadResponse)
    }

    Ok(response.json().await.map_err(|_| SummarizeError::BadResponse)?)
}

fn parse_url(url: &str) -> Result<Url, SummarizeError> {
    let parsed_url = Url::parse(url).map_err(|_| SummarizeError::InvalidUrl)?;
    if parsed_url.domain() == Some("youtube.com")
            || parsed_url.domain() == Some("youtu.be")
            || parsed_url.domain() == Some("www.youtube.com")
            || parsed_url.domain() == Some("www.youtu.be") {
        Ok(parsed_url)
    } else {
        Err(SummarizeError::InvalidUrl)
    }
}

fn trim_to_video_id(url: Url) -> Result<String, SummarizeError> {
    match url.domain() {
        Some(domain) => {
            if domain == "youtube.com" || domain == "www.youtube.com" {
                Ok(url.query_pairs().into_iter().find(|pair| pair.0 == "v")
                                    .ok_or(SummarizeError::InvalidUrl)?.1.into_owned())
            } else if domain == "youtu.be" || domain == "www.youtu.be" {
                Ok(url.path()[1..].to_owned())
            } else {
                Err(SummarizeError::InvalidUrl)
            }
        }
        None => Err(SummarizeError::InvalidUrl)
    }
}