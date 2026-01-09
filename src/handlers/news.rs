use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::ws::AppState;
use crate::config::*;
use crate::error::ApiError;
use crate::models::{Article, Source};
use crate::services::{fetch_rss, filter_by_keyword, parse_feed};

#[derive(Debug, Deserialize)]
pub struct NewsQuery {
    pub source: Option<String>,
    pub keyword: Option<String>,
    pub topic: Option<String>,
    pub country: Option<String>,
}

pub async fn get_news(Query(params): Query<NewsQuery>) -> Result<Json<Value>, ApiError> {
    let sources: Vec<&Source> = if let Some(ref code) = params.source {
        match find_source_by_code(code) {
            Some(s) => vec![s],
            None => return Err(ApiError::BadRequest(format!("Invalid source: {}", code))),
        }
    } else if let Some(ref country) = params.country {
        let s = find_sources_by_country(country);
        if s.is_empty() { return Err(ApiError::BadRequest("No sources for country".into())); }
        s
    } else if let Some(ref topic) = params.topic {
        let s = find_sources_by_category(topic);
        if s.is_empty() { return Err(ApiError::BadRequest("No sources for topic".into())); }
        s
    } else {
        vec![get_random_source().ok_or(ApiError::InternalError("No sources".into()))?]
    };

    let mut articles = Vec::new();
    for source in sources {
        if let Ok(content) = fetch_rss(&source.url).await {
            if let Ok(parsed) = parse_feed(&content, &source.name) {
                articles.extend(parsed);
            }
        }
    }

    let articles: Vec<Article> = if let Some(ref kw) = params.keyword {
        filter_by_keyword(&articles, kw)
    } else {
        articles
    }.into_iter().take(20).collect();

    Ok(Json(json!({"status": "success", "data": articles})))
}

pub async fn get_sources(Query(params): Query<SourcesQuery>) -> Json<Value> {
    let sources: Vec<Source> = if let Some(ref country) = params.country {
        find_sources_by_country(country).into_iter().cloned().collect()
    } else if let Some(ref topic) = params.topic {
        find_sources_by_category(topic).into_iter().cloned().collect()
    } else {
        get_all_sources().clone()
    };
    Json(json!({"status": "success", "data": sources}))
}

#[derive(Debug, Deserialize)]
pub struct SourcesQuery {
    pub country: Option<String>,
    pub topic: Option<String>,
}

/// Fetch news from all sources and broadcast to WebSocket clients
pub async fn fetch_and_broadcast(
    State(state): State<AppState>,
    Query(params): Query<NewsQuery>,
) -> Result<Json<Value>, ApiError> {
    let sources: Vec<&Source> = if let Some(ref topic) = params.topic {
        let s = find_sources_by_category(topic);
        if s.is_empty() {
            return Err(ApiError::BadRequest("No sources for topic".into()));
        }
        s
    } else if let Some(ref country) = params.country {
        let s = find_sources_by_country(country);
        if s.is_empty() {
            return Err(ApiError::BadRequest("No sources for country".into()));
        }
        s
    } else {
        // Fetch from all sources
        get_all_sources().iter().collect()
    };

    let mut articles = Vec::new();
    let mut broadcast_count = 0;

    for source in sources {
        if let Ok(content) = fetch_rss(&source.url).await {
            if let Ok(parsed) = parse_feed(&content, &source.name) {
                for article in &parsed {
                    // Broadcast each article to WebSocket clients
                    if state.tx.send(article.clone()).is_ok() {
                        broadcast_count += 1;
                    }
                }
                articles.extend(parsed);
            }
        }
    }

    let articles: Vec<Article> = if let Some(ref kw) = params.keyword {
        filter_by_keyword(&articles, kw)
    } else {
        articles
    };

    Ok(Json(json!({
        "status": "success",
        "fetched": articles.len(),
        "broadcast": broadcast_count
    })))
}
