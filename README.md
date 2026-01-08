# hypernews

RSS news aggregator API in Rust.

## Endpoints

```
GET /health
GET /api/sources
GET /api/news
GET /api/news?source=US-CNN
GET /api/news?keyword=tech
GET /api/news?topic=tech
GET /api/news?country=US
```

## Run

```
cargo run
```

## Test

```
cargo test
```

## Sources

BBC, CNN, NYT, NPR, TechCrunch, The Verge, The Guardian, Ars Technica, Wired, Hacker News
