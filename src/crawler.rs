use std::collections::{HashMap, HashSet};
use std::io::Read;
use select::document::Document;
use select::predicate::{Name, Text};

// fn get_links_from_html(html: String) -> HashSet<String>  {
//      Document::from(html.as_str())
//         .find(Name("a"))
//         .filter_map(|n| n.attr("href"))
//         .collect::<HashSet<String>>()
// }

pub fn crawl_website(user_input: &str) -> HashMap<String, String> {
    // web crawler that works its way through the web like this:
    // Start at a known URL
    // Download the page
    // Make a note of any URLs it contains
    // GOTO 1 (for the new URLs I’ve found)

    // keep track of visited and non visited urls 
    let mut visited_urls = HashSet::new();
    // non visited urls
    let mut to_visit_url = HashSet::new();

    // index pages 
    let mut index = HashMap::new();

    to_visit_url.insert(user_input.to_string());    

    let client = reqwest::blocking::Client::new();

    let max_pages = 3;

    while let Some(url) = to_visit_url.iter().next().cloned() {
        // because now its visited, remove from to_visit
        to_visit_url.remove(&url);

        if visited_urls.len() > max_pages {
            break;
        }

        // continue 
        if visited_urls.contains(&url) {
            continue;
        }
        
        let result = client.get(&url).send();

        if let Ok(mut resp) = result {
            // 🔹 FIX: read as bytes, then safely convert to UTF-8
            let mut bytes = Vec::new();
            if resp.read_to_end(&mut bytes).is_ok() {
                let body = String::from_utf8_lossy(&bytes).to_string();

                // extracting plain text
                let text = Document::from(body.as_str())
                .find(Text)
                .map(|n| n.text())
                .collect::<Vec<_>>()
                .join(" ");

                // adding to the set
                index.insert(url.clone(), text);

                println!("{}", body);

                let unique_urls = Document::from(body.as_str())
                    .find(Name("a"))
                    .filter_map(|n| n.attr("href"))
                    .map(str::to_string)
                    .collect::<HashSet<String>>();

                for link in unique_urls {
                    if !visited_urls.contains(&link) {
                        to_visit_url.insert(link);
                    }
                }

                visited_urls.insert(url);
            }
        }
        println!("All visited urls, {:#?}", visited_urls);
    }
    index
}
