// build a basic search engine that downloads pages and matches your search query against their contents.
// STEP 1: BUILD A MACHINE THAT CAN DOWNLOAD THE ENTIRE WEB 

use std::{io};

mod crawler;
mod search;
mod utils;
mod indexer;

fn main() {
    // user input variable
    let mut new_url = String::new();

    println!("Enter a valid url");

    // taking user input
    io::stdin()
    .read_line(&mut new_url).expect("error occured while accepting a url from user");

    // crawl_website returns HashMap<String, String> (url -> text)
    let pages = crawler::crawl_website(&new_url.trim());

    // Build an inverted index + TF-IDF scores from crawled pages
    // indexer.build_index returns a SearchEngine (contains inverted index and metadata)
    let engine = indexer::build_index(&pages);

    // starting a search loop 
    loop {
        println!("\nEnter search query (or 'exit' to quit):");
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let query = query.trim();

        if query == "exit" {
            break;
        }

        // now search using the search module which uses the built engine
        let results = search::search_query(query, &engine);
        if results.is_empty() {
            println!("No results found.");
        } else {
            println!("Results:");
            for (url, score) in results {
                println!("- {} (score: {:.4})", url, score);
            }
        }
    }
}
