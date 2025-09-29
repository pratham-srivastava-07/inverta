// build a basic search engine that downloads pages and matches your search query against their contents.


// STEP 1: BUILD A MACHINE THAT CAN DOWNLOAD THE ENTIRE WEB 

use std::collections::HashSet;
use std::io::Read;
use select::document::Document;
use select::predicate::Name;

fn main() {
    // web crawler that works its way through the web like this:
    // Start at a known URL
    // Download the page
    // Make a note of any URLs it contains
    // GOTO 1 (for the new URLs I’ve found)

    let client = reqwest::blocking::Client::new();
    let origin_url =  "https://github.com/";
    let mut result = client.get(origin_url).send().unwrap();
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
