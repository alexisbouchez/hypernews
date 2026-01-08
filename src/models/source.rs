use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Source {
    pub code: String,
    pub name: String,
    pub url: String,
    pub country: String,
    pub category: String,
}

impl Source {
    pub fn new(
        code: impl Into<String>,
        name: impl Into<String>,
        url: impl Into<String>,
        country: impl Into<String>,
        category: impl Into<String>,
    ) -> Result<Self, String> {
        let code = code.into();
        let name = name.into();
        let url = url.into();
        let country = country.into();
        let category = category.into();

        if code.trim().is_empty() { return Err("Code cannot be empty".into()); }
        if name.trim().is_empty() { return Err("Name cannot be empty".into()); }
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err("Invalid URL".into());
        }

        Ok(Self { code, name, url, country, category })
    }
}
