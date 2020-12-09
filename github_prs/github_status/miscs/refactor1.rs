use reqwest::{blocking, header};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Results Object
#[derive(Deserialize)]
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
        page_count += 1;
    }

    // the number of results
    println!("Number of open PRs: {}", all.len());
    let oldest = all.len() - 1;
    // using the sort function on the API side
    println!(
        "Oldest PR is number: {:#?} with title: {:#?} and date: {:#?}",
        all[oldest].number, all[oldest].title, all[oldest].created_at
    );

    // folding to get maximum length of the body
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
