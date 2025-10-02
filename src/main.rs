// build a basic search engine that downloads pages and matches your search query against their contents.
// STEP 1: BUILD A MACHINE THAT CAN DOWNLOAD THE ENTIRE WEB 

use std::{io};
mod crawler;
mod search;
mod utils;

fn main() {
    // user input variable
    let mut new_url = String::new();

    println!("Enter a valid url");

    // taking user input
    io::stdin()
    .read_line(&mut new_url).expect("error occured while accepting a url from user");

    let result = crawler::crawl_website(&new_url);
    // println!("Indexed {} pages", result.len())

    // starting a search loop 
    loop {
        println!("\nEnter search query (or 'exit' to quit):");
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let query = query.trim();

        if query == "exit" {
            break;
        }

        let results = search::search_query(query, &result);
        if results.is_empty() {
            println!("No results found.");
        } else {
            println!("Results:");
            for url in results {
                println!("- {}", url);
            }
    }
  }
}
