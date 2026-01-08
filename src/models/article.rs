use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Article {
    pub source: String,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub pub_date: String,
}

impl Article {
    pub fn new(
        source: impl Into<String>,
        title: impl Into<String>,
        link: impl Into<String>,
        description: Option<String>,
        pub_date: impl Into<String>,
    ) -> Self {
        Self {
            source: source.into(),
            title: title.into(),
            link: link.into(),
            description,
            pub_date: pub_date.into(),
        }
    }

    pub fn matches_keyword(&self, keyword: &str) -> bool {
        let kw = keyword.to_lowercase();
        self.title.to_lowercase().contains(&kw)
            || self.description.as_ref().map(|d| d.to_lowercase().contains(&kw)).unwrap_or(false)
    }

    pub fn clean_html(text: &str) -> String {
        let re = Regex::new(r"<[^>]*>").unwrap();
        re.replace_all(text, "")
            .replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&nbsp;", " ")
            .trim()
            .to_string()
    }
}
