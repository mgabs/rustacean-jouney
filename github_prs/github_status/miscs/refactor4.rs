use reqwest::{blocking, header};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Results Object
#[derive(Debug, Deserialize)]
struct Obj {
    items: Vec<Issue>,
}

/// Basic information about a pull request
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Issue {
    /// The human-visible number of the pull request
    number: usize,
    /// The title
    title: String,
    // creation date
    created_at: String,
    /// The URL of the pull request
    url: String,
    // body - for body length
    body: String,
}

fn getter(get_url: &str) -> reqwest::blocking::Response {
    let client = blocking::Client::new();
    client
        .get(get_url)
        .header(header::USER_AGENT, "AwesomeBuilder")
        .send()
        .expect("Invalid json to parse")
}
fn main() -> Result<(), Box<dyn Error>> {
    //! defining the repo here.
    // let repo = "rust-lang/rust";
    let repo = "torvalds/linux";
    println!("Fetching data from Github for {}..", repo);

    let mut page_count = 0;
    let mut all: Vec<Issue> = vec![];
    let url = format!( "https://api.github.com/search/issues?q=repo:{} is:open type:pr&sort=created_at&order=asc&per_page=100&page=", repo);

    let resp = getter(&url);
    if resp.status().is_success() {
        // if let Ok(a) = serde_json::from_str(resp.headers().get("link")) {
        //     println!("{:#?}", a);
        // }
        // if let Some(etag) = resp.headers().get(header::ETAG) {
        //     println!(" {:#?}", etag.as_bytes());
        // }

        // links
        // "<https://api.github.com/search/issues?q=repo%3Atorvalds%2Flinux+is%3Aopen+type%3Apr&sort=created_at&order=asc&per_page=100&page=2>;
        //  rel=\"next\", <https://api.github.com/search/issues?q=repo%3Atorvalds%2Flinux+is%3Aopen+type%3Apr&sort=created_at&order=asc&per_page=100&page=4>;
        // rel=\"last\""

        println!("{:#?}", resp.headers()["link"]);
    }
    // since github has a limit of 100 per response, looping to get all the data
    // loop {
    //     // prepare url
    //     let page_url = format!("{}{}", url, page_count);

    //     let resp = getter(&page_url);
    //     println!("Fetched Page{} - result {}.", page_count, resp.status());
    //     // check request response
    //     if resp.status().is_success() {
    //         if let Ok(a) = serde_json::from_str::<Obj>(&resp.text()?) {
    //             if a.items.len() == 0 {
    //                 // break if request is empty - github return empty success headers
    //                 break;
    //             }
    //             for i in a.items {
    //                 all.push(i);
    //             }
    //         };
    //     } else {
    //         // break if request failed
    //         break;
    //     };
    //     page_count += 1;
    // }

    for j in 0..4 {
        //    prepare url
        let page_url = format!("{}{}", url, j);

        let resp = getter(&page_url);
        println!("Fetched Page{} - result {}.", j, resp.status());
        // check request response
        if resp.status().is_success() {
            if let Ok(a) = serde_json::from_str::<Obj>(&resp.text()?) {
                if a.items.len() == 0 {
                    // break if request is empty - github return empty success headers
                    break;
                }
                for i in a.items {
                    all.push(i);
                }
            };
        } else {
            // break if request failed
            break;
        };
    }

    // the number of results
    println!("Number of open PRs: {}", all.len());
    let oldest = all.len() - 1;
    // using the sort function on the API side
    println!(
        "Oldest: {}#{}: {} - {}",
        repo, all[oldest].number, all[oldest].title, all[oldest].created_at
    );

    // ! Oldest: rust-lang/rust#65819: Add `IntoIterator` impl for arrays by value (`for [T; N]`)
    // ! Longest body: rust-lang/rust#79135: stabilize `#![feature(min_const_generics)]` in 1.50

    // folding to get maximum length of the body
    let x = all
        .iter()
        .max_by(|&x, y| x.body.len().cmp(&y.body.len()))
        .unwrap();
    // The longest PR is of length 65570, I suspect this is API limitation on github's side
    println!(
        "Longest body: {}:#{}: {} - body length: {}",
        repo,
        x.number,
        x.title,
        x.body.len()
    );

    Ok(())
}
