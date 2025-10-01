use std::collections::HashSet;
use std::io::Read;
use select::document::Document;
use select::predicate::Name;

// fn get_links_from_html(html: String) -> HashSet<String>  {
//      Document::from(html.as_str())
//         .find(Name("a"))
//         .filter_map(|n| n.attr("href"))
//         .collect::<HashSet<String>>()
// }

pub fn crawl_website(user_input: &str) {
    // web crawler that works its way through the web like this:
    // Start at a known URL
    // Download the page
    // Make a note of any URLs it contains
    // GOTO 1 (for the new URLs I’ve found)

    // keep track of visited and non visited urls 
    let mut visited_urls = HashSet::new();
    // non visited urls
    let mut to_visit_url = HashSet::new();

    to_visit_url.insert(user_input.to_string());    


    let client = reqwest::blocking::Client::new();

    while let Some(url) = to_visit_url.iter().next().cloned() {
        // because now its visited, remove from to_visit
        to_visit_url.remove(&url);

        // continue 
        if visited_urls.contains(&url) {
            continue;
        }
        
        let result = client.get(&url).send();

        if let Ok(mut resp) = result {
            let mut body = String::new();
             resp.read_to_string(&mut body).unwrap();
            println!("{}", body);
            // println!("Hello, world!");

            // Document::from(body.as_str()).find(Name("a")).filter_map(|n| n.attr("href")).for_each(|x| println!("{}", x));
            let unique_urls = Document::from(body.as_str()).find(Name("a")).filter_map(|n| n.attr("href")).map(str::to_string)
            .collect::<HashSet<String>>();

            for link in unique_urls {
                if !visited_urls.contains(&link) {
                    to_visit_url.insert(link);
                }
            }

            visited_urls.insert(url);
            // println!("{:#?}", unique_urls);
        }
        println!("All visited urls, {:#?}", visited_urls);
    }

    
}