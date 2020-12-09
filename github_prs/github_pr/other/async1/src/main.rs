use serde::Deserialize;

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

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let repo = "octocat/Hello-World";
    let per_page = 100u32;
    let mut page_count = 0u32;
    let mut total_count: u32 = 0;
    let mut results: Vec<ResultVec> = vec![];
    let url = format!( "https://api.github.com/search/issues?q=repo:{} is:open type:pr&sort=created_at&order=asc&per_page={}&page=", repo, per_page);
    println!("Fetching data from Github for '{}' ..", repo);

    // since github has a limit of 100 per response, looping to get all the data
    loop {
        // prepare url
        let page_url = format!("{}{}", url, page_count);
        let resp = reqwest::Client::new()
            .get(&page_url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "AwesomeBuilder")
            .send()
            .await?;

        // check request response
        if resp.status().is_success() {
            if let Ok(a) = resp.json::<ApiResponse>().await {
                total_count = a.total_count;
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
        if page_count * per_page > total_count {
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

        // folding to get maximum length of the body
        let x = results.iter().max_by(|&x, y| x.body.cmp(&y.body)).unwrap();
        // The longest PR is of length 65570 - this is API limitation on github's side
        println!("Longest body: {}:#{}: {}", repo, x.number, x.title);
    } else {
        println!("The query didn't retrive any results!")
    };
    Ok(())
}
