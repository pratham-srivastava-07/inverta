use url::Url;
use regex::Regex;
use lazy_static::lazy_static;

/// Normalize links relative to a base URL, remove fragments, and ensure scheme.
/// Returns Some(normalized_string) or None if the URL can't be parsed.
pub fn _normalize_link(base: &str, link: &str) -> Option<String> {
    // try absolute parse
    if let Ok(u) = Url::parse(link) {
        // remove fragment
        let mut u = u;
        u.set_fragment(None);
        return Some(u.into_string());
    }

    // try relative to base
    if let Ok(base_u) = Url::parse(base) {
        if let Ok(joined) = base_u.join(link) {
            let mut j = joined;
            j.set_fragment(None);
            return Some(j.into_string());
        }
    }

    None
}

/// Basic tokenization: lowercase, strip punctuation, split on whitespace.
/// Removes empty tokens and returns Vec<String>
pub fn tokenize(text: &str) -> Vec<String> {
    lazy_static! {
        static ref NON_ALPHANUMERIC: Regex = Regex::new(r"[^A-Za-z0-9]+").unwrap();
    }

    text.split_whitespace()
        .map(|w| {
            let cleaned = NON_ALPHANUMERIC.replace_all(w, " ");
            cleaned.to_string()
        })
        .flat_map(|s| s.split_whitespace().map(|t| t.to_lowercase()).collect::<Vec<_>>())
        .filter(|t| !t.is_empty())
        .collect()
}

/// A small list of stopwords for English 
pub fn stopwords() -> Vec<&'static str> {
    vec![
        "the", "is", "at", "which", "on", "and", "a", "an", "in", "for", "to", "of", "by", "with",
        "that", "this", "it", "as", "are", "was", "from", "or", "be", "has", "have"
    ]
}
