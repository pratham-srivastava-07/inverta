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

    let client = reqwest::blocking::Client::new();

    let mut result = client.get(user_input).send().unwrap();
    println!("{}", result.status());

    let mut body = String::new();

    result.read_to_string(&mut body).unwrap();
    println!("{}", body);
    println!("Hello, world!");

    // Document::from(body.as_str()).find(Name("a")).filter_map(|n| n.attr("href")).for_each(|x| println!("{}", x));
    let unique_urls = Document::from(body.as_str()).find(Name("a")).filter_map(|n| n.attr("href")).map(str::to_string)
    .collect::<HashSet<String>>();

    println!("{:#?}", unique_urls);

    
}