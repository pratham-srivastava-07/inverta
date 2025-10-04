use crate::indexer::SearchEngine;
use crate::utils::{tokenize, stopwords};
use std::collections::HashMap;

/// Search the engine for a query string and return ranked results
/// Returns Vec<(url, score)> sorted by score desc
pub fn search_query(query: &str, engine: &SearchEngine) -> Vec<(String, f64)> {
    let stops: std::collections::HashSet<String> = stopwords().into_iter().map(|s| s.to_string()).collect();

    // tokenize query
    let tokens = tokenize(query)
        .into_iter()
        .filter(|t| !stops.contains(t))
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        return Vec::new();
    }

    // accumulator: url -> score
    let mut scores: HashMap<String, f64> = HashMap::new();

    for token in tokens.iter() {
        if let Some(postings) = engine.inverted.get(token) {
            for (url, score) in postings.iter() {
                *scores.entry(url.clone()).or_insert(0.0) += *score;
            }
        }
    }

    // convert to vec and sort
    let mut results: Vec<(String, f64)> = scores.into_iter().collect();
    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    results
}
