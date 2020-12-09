// env_logger::init();
// env::set_var("RUST_LOG", "hyperurl=info");
// env::set_var("RUST_LOG", "hyperurl=info");
//

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct PullRequest {
    // /// The URL of the pull request
    // url: String,
    // /// The human-visible number of the pull request
    // number: usize,
    // /// The title
    // title: String,
    id: usize,
    body: String,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let input: serde_json::Value = reqwest::Client::new()
        .get("https://api.github.com/repos/octocat/hello-world/pulls")
        .json(&serde_json::json!({
            "state": "open",
            "sort": "created_at",
            "direction": "asc"
        }))
        .send()
        .await?
        .json()
        .await?;

    // let pull_request: PullRequest = serde_json::from_str(input).expect("Expected valid JSON parse");
    // println!("#{}: {}", pull_request.number, pull_request.title);

    // let pull_request: PullRequest = serde_json::from_value(input).expect("Error parsing JSON!");
    // println!("#{}: {}", pull_request.id, pull_request.title);
    //
    println!("{}", input);

    Ok(())
}
