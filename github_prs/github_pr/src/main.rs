// use paginate::Pages;
use reqwest::{blocking, header};
use serde::Deserialize;
use std::error::Error;

// ApiResponse Results
#[derive(Debug, Deserialize)]
struct ApiResponse {
    items: Vec<Issue>,
    total_count: u32,
}

/// Basic information about a pull request
#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
struct ResultVec {
    /// The human-visible number of the pull request
    number: usize,
    /// The title
    title: String,
    // creation date
    created_at: String,
    /// The URL of the pull request
    url: String,
    // body - for body length
    body: usize,
}

#[allow(clippy::clippy::len_zero)]
fn main() -> Result<(), Box<dyn Error>> {
    //! defining the repo here.
    // let repo = "torvalds/linux";
    let repo = "octocat/Hello-World";
    let per_page = 100;
    let mut page_count = 0;
    let mut total_results = 0;
    let mut results: Vec<ResultVec> = vec![];
    let url = format!( "https://api.github.com/search/issues?q=repo:{} is:open type:pr&sort=created_at&order=asc&per_page={}&page=", repo, per_page);
    println!("Fetching data from Github for '{}' ..", repo);

    let client = blocking::Client::new();
    // since github has a limit of 100 per response, looping to get all the data
    loop {
        // prepare url
        let page_url = format!("{}{}", url, page_count);
        // get data
        println!("Fetching: {}", page_url);
        let resp = client
            .get(&page_url)
            .header(header::USER_AGENT, "AwesomeBuilder")
            .send()
            .expect("Not valid http response");

        // check request response
        if resp.status().is_success() {
            if let Ok(a) = serde_json::from_str::<ApiResponse>(&resp.text()?) {
                total_results = a.total_count;
                if a.items.len() == 0 {
                    // break if request is empty - github return empty success headers
                    break;
                }
                for i in a.items {
                    let result_vec = ResultVec {
                        number: i.number,
                        title: i.title,
                        created_at: i.created_at,
                        url: i.url,
                        body: i.body.len(),
                    };
                    results.push(result_vec);
                }
            };
        } else {
            // break if request failed
            break;
        };
        if page_count * per_page > total_results {
            break;
        }
        page_count += 1;
    }

    // sanity check to ensure we get any results
    if results.len() > 0 {
        // the number of results
        println!("Number of open PRs: {}", results.len());
        let oldest = results.len() - 1;
        // using the sort function on the API side
        println!(
            "Oldest: {}#{}: {}",
            repo, results[oldest].number, results[oldest].title
        );

        // ! Oldest: rust-lang/rust#65819: Add `IntoIterator` impl for arrays by value (`for [T; N]`)
        // ! Longest body: rust-lang/rust#79135: stabilize `#![feature(min_const_generics)]` in 1.50

        // folding to get maximum length of the body
        let x = results.iter().max_by(|&x, y| x.body.cmp(&y.body)).unwrap();
        // The longest PR is of length 65570 - this is API limitation on github's side
        println!("Longest body: {}:#{}: {}", repo, x.number, x.title);
    } else {
        println!("The query didn't retrive any results!")
    };

    Ok(())
}
