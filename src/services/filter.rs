use crate::models::Article;

pub fn filter_by_keyword(articles: &[Article], keyword: &str) -> Vec<Article> {
    articles.iter().filter(|a| a.matches_keyword(keyword)).cloned().collect()
}
