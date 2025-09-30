// build a basic search engine that downloads pages and matches your search query against their contents.
// STEP 1: BUILD A MACHINE THAT CAN DOWNLOAD THE ENTIRE WEB 

use std::io;
mod crawler;

fn main() {
    // user input variable
    let mut new_url = String::new();

    println!("Enter a valid url");

    // taking user input
    io::stdin()
    .read_line(&mut new_url).expect("error occured while accepting a url from user");

    let _result = crawler::crawl_website(&new_url);
}
