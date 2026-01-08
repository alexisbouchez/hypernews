use crate::models::Source;
use once_cell::sync::Lazy;
use rand::prelude::IndexedRandom;

pub static SOURCES: Lazy<Vec<Source>> = Lazy::new(|| {
    vec![
        Source::new("INTER-BBC", "BBC World", "https://feeds.bbci.co.uk/news/world/rss.xml", "INTER", "general").unwrap(),
        Source::new("US-CNN", "CNN", "https://rss.cnn.com/rss/cnn_topstories.rss", "US", "general").unwrap(),
        Source::new("US-NYT", "New York Times", "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml", "US", "general").unwrap(),
        Source::new("US-NPR", "NPR", "https://feeds.npr.org/1001/rss.xml", "US", "general").unwrap(),
        Source::new("US-TC", "TechCrunch", "https://techcrunch.com/feed/", "US", "tech").unwrap(),
        Source::new("US-VERGE", "The Verge", "https://www.theverge.com/rss/index.xml", "US", "tech").unwrap(),
        Source::new("UK-GUARD", "The Guardian", "https://www.theguardian.com/world/rss", "UK", "general").unwrap(),
        Source::new("UK-BBC", "BBC UK", "https://feeds.bbci.co.uk/news/rss.xml", "UK", "general").unwrap(),
        Source::new("TECH-ARS", "Ars Technica", "https://feeds.arstechnica.com/arstechnica/index", "INTER", "tech").unwrap(),
        Source::new("TECH-WIRED", "Wired", "https://www.wired.com/feed/rss", "INTER", "tech").unwrap(),
        Source::new("TECH-HN", "Hacker News", "https://hnrss.org/frontpage", "INTER", "tech").unwrap(),
    ]
});

pub fn get_all_sources() -> &'static Vec<Source> { &SOURCES }

pub fn find_source_by_code(code: &str) -> Option<&'static Source> {
    let code_upper = code.to_uppercase();
    SOURCES.iter().find(|s| s.code.to_uppercase() == code_upper)
}

pub fn find_sources_by_country(country: &str) -> Vec<&'static Source> {
    let c = country.to_uppercase();
    SOURCES.iter().filter(|s| s.country.to_uppercase() == c).collect()
}

pub fn find_sources_by_category(category: &str) -> Vec<&'static Source> {
    let c = category.to_lowercase();
    SOURCES.iter().filter(|s| s.category.to_lowercase() == c).collect()
}

pub fn get_random_source() -> Option<&'static Source> {
    SOURCES.choose(&mut rand::rng())
}
