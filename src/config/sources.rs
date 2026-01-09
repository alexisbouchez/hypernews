use crate::models::Source;
use once_cell::sync::Lazy;
use rand::prelude::IndexedRandom;

pub static SOURCES: Lazy<Vec<Source>> = Lazy::new(|| {
    vec![
        // International
        Source::new("INTER-BBC", "BBC World", "https://feeds.bbci.co.uk/news/world/rss.xml", "INTER", "general").unwrap(),
        Source::new("INTER-AJ", "Al Jazeera", "https://www.aljazeera.com/xml/rss/all.xml", "INTER", "general").unwrap(),
        Source::new("INTER-ECON", "The Economist", "https://www.economist.com/international/rss.xml", "INTER", "general").unwrap(),
        Source::new("INTER-DW", "Deutsche Welle", "https://rss.dw.com/rdf/rss-en-all", "INTER", "general").unwrap(),
        Source::new("INTER-BLOOM", "Bloomberg", "https://feeds.bloomberg.com/markets/news.rss", "INTER", "business").unwrap(),
        // US
        Source::new("US-CNN", "CNN", "https://rss.cnn.com/rss/cnn_topstories.rss", "US", "general").unwrap(),
        Source::new("US-NYT", "New York Times", "https://rss.nytimes.com/services/xml/rss/nyt/HomePage.xml", "US", "general").unwrap(),
        Source::new("US-NPR", "NPR", "https://feeds.npr.org/1001/rss.xml", "US", "general").unwrap(),
        Source::new("US-NBC", "NBC News", "https://feeds.nbcnews.com/nbcnews/public/news", "US", "general").unwrap(),
        Source::new("US-CBS", "CBS News", "https://www.cbsnews.com/latest/rss/main", "US", "general").unwrap(),
        Source::new("US-POLITICO", "Politico", "https://rss.politico.com/politics-news.xml", "US", "politics").unwrap(),
        Source::new("US-TC", "TechCrunch", "https://techcrunch.com/feed/", "US", "tech").unwrap(),
        Source::new("US-VERGE", "The Verge", "https://www.theverge.com/rss/index.xml", "US", "tech").unwrap(),
        Source::new("US-9TO5", "9to5Mac", "https://9to5mac.com/feed/", "US", "tech").unwrap(),
        // UK
        Source::new("UK-BBC", "BBC UK", "https://feeds.bbci.co.uk/news/rss.xml", "UK", "general").unwrap(),
        Source::new("UK-GUARD", "The Guardian", "https://www.theguardian.com/world/rss", "UK", "general").unwrap(),
        Source::new("UK-SKY", "Sky News", "https://feeds.skynews.com/feeds/rss/world.xml", "UK", "general").unwrap(),
        // Tech
        Source::new("TECH-ARS", "Ars Technica", "https://feeds.arstechnica.com/arstechnica/index", "INTER", "tech").unwrap(),
        Source::new("TECH-WIRED", "Wired", "https://www.wired.com/feed/rss", "INTER", "tech").unwrap(),
        Source::new("TECH-HN", "Hacker News", "https://hnrss.org/frontpage", "INTER", "tech").unwrap(),
        Source::new("TECH-NYT", "NYT Tech", "https://rss.nytimes.com/services/xml/rss/nyt/Technology.xml", "US", "tech").unwrap(),
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
