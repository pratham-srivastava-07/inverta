use std::collections::HashMap;

pub fn search_query(query: &str, index: &HashMap<String, String>) -> Vec<String> {
    let query = query.to_lowercase();

    index.iter().filter(|(_, content)| content.to_lowercase().contains(&query)).map(|(url, _)| url.clone()).collect()

}