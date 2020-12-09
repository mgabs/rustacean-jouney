use reqwest::{blocking, header};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Object of PullRequests
#[derive(Deserialize)]
struct Obj {
    items: Vec<Issue>,
    total_count: usize,
}

/// Basic information about a pull request
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Issue {
    /// The URL of the pull request
    /// The human-visible number of the pull request
    number: usize,
    /// The title
    title: String,
    created_at: String,
    url: String,
    body: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Fetching data from Github for torvalds/linux..");

    let mut page_count = 0;
    let mut all: Vec<Issue> = vec![];
    let url = "https://api.github.com/search/issues?q=repo:torvalds/linux is:open type:pr&sort=created_at&order=asc&per_page=100&page=";

    let client = blocking::Client::new();
    // since github has a limit of 100 per response, looping to get all the data
    loop {
        let page_url = format!("{}{}", url, page_count);
        let resp = client
            .get(&page_url)
            .header(header::USER_AGENT, "AwesomeBuilder")
            .send()?;

        println!("Fetched Page{} - result {}.", page_count, resp.status());
        // check request response
        if resp.status().is_success() || !resp.body.is_null() {
            // let _issues =
            if let Ok(a) = serde_json::from_str::<Obj>(&resp.text()?) {
                for i in a.items {
                    all.push(i);
                }
            };
        } else {
            break;
        };
        page_count += 1;
    }

    // the number of results
    let vec_length = all.len();
    println!("Number of open PRs: {}", vec_length);
    // using the sort function on the API side
    println!(
        "Oldest PR is number: {:#?} with title: {:#?} and date: {:#?}",
        all[vec_length - 1].number,
        all[vec_length - 1].title,
        all[vec_length - 1].created_at
    );

    let x = all
        .iter()
        .max_by(|&x, y| x.body.len().cmp(&y.body.len()))
        .unwrap();
    // The longest PR is of length 65570, I suspect this is API limitation on github's side
    println!(
        "Longest PR body is number {:#?} with title: {:#?} and body length: {:#?}",
        x.number,
        x.title,
        x.body.len()
    );

    Ok(())
}
