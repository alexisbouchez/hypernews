use crate::error::ApiError;
use crate::models::Article;
use feed_rs::parser;

pub fn parse_feed(content: &str, source_name: &str) -> Result<Vec<Article>, ApiError> {
    let feed = parser::parse(content.as_bytes())
        .map_err(|e| ApiError::RssParseError(e.to_string()))?;

    let articles = feed.entries.into_iter().take(20).filter_map(|entry| {
        let title = entry.title.map(|t| Article::clean_html(&t.content))?;
        let link = entry.links.first().map(|l| l.href.clone())
            .unwrap_or_else(|| entry.id.clone());

        if !link.starts_with("http") { return None; }

        let description = entry.summary.map(|s| Article::clean_html(&s.content));
        let pub_date = entry.published.or(entry.updated)
            .map(|d| d.to_rfc2822()).unwrap_or_else(|| "Unknown".into());

        Some(Article::new(source_name, title, link, description, pub_date))
    }).collect();

    Ok(articles)
}
