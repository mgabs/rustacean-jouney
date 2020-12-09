//! Write a program that, given a GitHub repository, outputs the
//!
//!   1/ Oldest open pull request
//!   2/ Open pull request with the longest body (most characters or bytes)
//!
//! Which, when given "rust-lang/rust" should look something like this
//!
//! """
//! Oldest: rust-lang/rust#65819: Add `IntoIterator` impl for arrays by value (`for [T; N]`)
//! Longest body: rust-lang/rust#79135: stabilize `#![feature(min_const_generics)]` in 1.50
//! """
//!
//! Note: GitHub has both a REST and a GraphQL API. Use the one you feel more comfortable with.
//! Note: `reqwest` is a great library to use to get data from a URL. `reqwest::blocking` is a great place to start unless you are already familiar with async Rust.
//! Note: `serde` and `serde_json` are great libraries to use to turn JSON into structured elements. See example below.
//! Note: You may have to do some error handing, but error handing isn't the primary purpose of the exercise.

//! Dependencies we would recommend putting in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! reqwest = { version = "0.10", default-features = false, features = ["rustls-tls", "blocking", "json"] }
//! serde = { version = "1", features = ["derive"] }
//! serde_json = "1"
//! ```

//! Example of using serde and serde_json:

/// Basic information about a pull request
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct PullRequest {
    /// The URL of the pull request
    url: String,
    /// The human-visible number of the pull request
    number: usize,
    /// The title
    title: String,
}

fn main() {
    let _input = r#"
    {
        "url": "https://api.github.com/repos/octocat/Hello-World/pulls/1347",
        "id": 1,
        "node_id": "MDExOlB1bGxSZXF1ZXN0MQ==",
        "html_url": "https://github.com/octocat/Hello-World/pull/1347",
        "diff_url": "https://github.com/octocat/Hello-World/pull/1347.diff",
        "patch_url": "https://github.com/octocat/Hello-World/pull/1347.patch",
        "issue_url": "https://api.github.com/repos/octocat/Hello-World/issues/1347",
        "commits_url": "https://api.github.com/repos/octocat/Hello-World/pulls/1347/commits",
        "review_comments_url": "https://api.github.com/repos/octocat/Hello-World/pulls/1347/comments",
        "review_comment_url": "https://api.github.com/repos/octocat/Hello-World/pulls/comments{/number}",
        "comments_url": "https://api.github.com/repos/octocat/Hello-World/issues/1347/comments",
        "statuses_url": "https://api.github.com/repos/octocat/Hello-World/statuses/6dcb09b5b57875f334f61aebed695e2e4193db5e",
        "number": 1347,
        "state": "open",
        "locked": true,
        "title": "Amazing new feature"
    }
    "#;

    let input = r#"


    "#;
    let pull_request: PullRequest = serde_json::from_str(input).expect("Expected valid JSON parse");
    println!("#{}: {}", pull_request.number, pull_request.title);
}
