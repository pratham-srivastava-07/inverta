# Inverta

**Inverta** is a basic search engine written in Rust. It allows you to crawl websites, index their content using TF-IDF, and perform search queries against the indexed data.

## Features

-   **Web Crawler**: Downloads pages starting from a seed URL.
-   **Indexer**: Builds an inverted index with TF-IDF scoring.
-   **Search**: Query the index to find relevant pages.

## Installation

Ensure you have Rust and Cargo installed.

```bash
git clone https://github.com/pratham-srivastava-07/Inverta.git
cd Inverta
cargo build --release
```

## Usage

Run the application using Cargo:

```bash
cargo run
```

1.  **Enter a valid URL** when prompted (e.g., `https://example.com`).
2.  The crawler will fetch pages (up to a limit).
3.  Enters a **search loop**:
    -   Type a query to search the indexed pages.
    -   Type `exit` to quit.

## Dependencies

-   `reqwest`: HTTP client for crawling.
-   `select`: HTML parsing.
-   `url`: URL parsing and normalization.
-   `regex`: Tokenization.
-   `lazy_static`: Static initialization.

## License

MIT
