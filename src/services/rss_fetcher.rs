use crate::error::ApiError;
use std::time::Duration;

pub async fn fetch_rss(url: &str) -> Result<String, ApiError> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0")
        .build()
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let response = client.get(url).send().await.map_err(|e| {
        if e.is_timeout() {
            ApiError::Timeout(format!("Request to {} timed out", url))
        } else {
            ApiError::RssFetchError(e.to_string())
        }
    })?;

    if !response.status().is_success() {
        return Err(ApiError::RssFetchError(format!("HTTP {}", response.status())));
    }

    response.text().await.map_err(|e| ApiError::RssFetchError(e.to_string()))
}
