use std::collections::{HashMap, HashSet};
use crate::utils::{tokenize, stopwords, _normalize_link};

/// A simple search engine index container that holds:
///  inverted index: token -> Vec<(url, tfidf-score)>
///  doc_count: total number of documents
#[derive(Debug)]
pub struct SearchEngine {
    pub inverted: HashMap<String, Vec<(String, f64)>>,
    #[warn(dead_code)]
    pub doc_count: usize,
}

/// Build an inverted index with TF-IDF scores from the pages (url -> raw_text)
pub fn build_index(pages: &HashMap<String, String>) -> SearchEngine {
    // compute term frequencies per document and document frequencies.
    let mut doc_term_freqs: HashMap<String, HashMap<String, usize>> = HashMap::new(); // term -> count
    let mut doc_lengths: HashMap<String, usize> = HashMap::new(); // 
    let mut doc_freqs: HashMap<String, usize> = HashMap::new();

    let stops: HashSet<String> = stopwords().into_iter().map(|s| s.to_string()).collect();

    for (url, body) in pages.iter() {
        // normalize URL when indexing (keeps original key)
        let normalized_url = match _normalize_link(url, "") {
            Some(_) => url.clone(),
            None => url.clone(),
        };

        // tokenize and filter stopwords
        let tokens = tokenize(body)
            .into_iter()
            .filter(|t| !stops.contains(t))
            .collect::<Vec<_>>();

        let length = tokens.len();
        doc_lengths.insert(normalized_url.clone(), length);

        let mut freqs: HashMap<String, usize> = HashMap::new();
        for token in tokens.into_iter() {
            *freqs.entry(token.clone()).or_insert(0) += 1;
        }

        // record term frequencies for this document
        for (term, _) in freqs.iter() {
            *doc_freqs.entry(term.clone()).or_insert(0) += 1;
        }

        doc_term_freqs.insert(normalized_url.clone(), freqs);
    }

    let total_docs = doc_term_freqs.len();

    // build inverted index with TF-IDF per (term, doc)
    let mut inverted: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for (doc, freqs) in doc_term_freqs.iter() {
        let doc_len = *doc_lengths.get(doc).unwrap_or(&1) as f64;
        for (term, &count) in freqs.iter() {
            let tf = (count as f64) / doc_len; // normalized term frequency
            let df = *doc_freqs.get(term).unwrap_or(&1) as f64;
            let idf = ( (total_docs as f64) / (1.0 + df) ).ln() + 1.0; // smoothing + 1
            let tfidf = tf * idf;

            inverted.entry(term.clone())
                .or_insert_with(Vec::new)
                .push((doc.clone(), tfidf));
        }
    }

    // sort postings by score descending for faster retrieval
    for postings in inverted.values_mut() {
        postings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    }

    SearchEngine {
        inverted,
        doc_count: total_docs,
    }
}
