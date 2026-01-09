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

# filter by keyword
curl localhost:3000/api/news?keyword=bitcoin

# get all tech news
curl localhost:3000/api/news?topic=tech

# get news from US sources
curl localhost:3000/api/news?country=US

# combine filters
curl "localhost:3000/api/news?topic=tech&keyword=ai"

# list sources
curl localhost:3000/api/sources
```

## Sources

| Code | Name | Category |
|------|------|----------|
| INTER-BBC | BBC World | general |
| INTER-AJ | Al Jazeera | general |
| INTER-ECON | The Economist | general |
| INTER-DW | Deutsche Welle | general |
| INTER-BLOOM | Bloomberg | business |
| US-CNN | CNN | general |
| US-NYT | New York Times | general |
| US-NPR | NPR | general |
| US-NBC | NBC News | general |
| US-CBS | CBS News | general |
| US-POLITICO | Politico | politics |
| US-TC | TechCrunch | tech |
| US-VERGE | The Verge | tech |
| US-9TO5 | 9to5Mac | tech |
| UK-BBC | BBC UK | general |
| UK-GUARD | The Guardian | general |
| UK-SKY | Sky News | general |
| TECH-ARS | Ars Technica | tech |
| TECH-WIRED | Wired | tech |
| TECH-HN | Hacker News | tech |
| TECH-NYT | NYT Tech | tech |

## Test

```
cargo test
```
