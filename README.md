# hypernews

RSS news aggregator API in Rust.

## Run

```
cargo run
```

Server starts on `localhost:3000`

## Usage

```bash
# get news from random source
curl localhost:3000/api/news

# get news from specific source
curl localhost:3000/api/news?source=US-CNN

# filter by keyword in title/description
curl localhost:3000/api/news?keyword=bitcoin

# get all tech news
curl localhost:3000/api/news?topic=tech

# get news from US sources only
curl localhost:3000/api/news?country=US

# combine filters
curl "localhost:3000/api/news?topic=tech&keyword=ai"

# list available sources
curl localhost:3000/api/sources

# health check
curl localhost:3000/health
```

## Sources

| Code | Name |
|------|------|
| US-CNN | CNN |
| US-NYT | New York Times |
| US-NPR | NPR |
| US-TC | TechCrunch |
| US-VERGE | The Verge |
| UK-BBC | BBC UK |
| UK-GUARD | The Guardian |
| INTER-BBC | BBC World |
| TECH-ARS | Ars Technica |
| TECH-WIRED | Wired |
| TECH-HN | Hacker News |

## Test

```
cargo test
```
