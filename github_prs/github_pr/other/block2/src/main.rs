/// Github Api implementation using
/// Reqwest::blocking & method iteration
/// current status:
/// 1. error consuming the last segment
/// 2. can't invoke more than 1 item.
///
use reqwest::{blocking, header, Result};
use serde::Deserialize;
// use std::error::Error;

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

// #[derive(Debug, Deserialize)]
// struct ResultVec {
//     /// The human-visible number of the pull request
//     number: usize,
//     /// The title
//     title: String,
//     // creation date
//     created_at: String,
//     /// The URL of the pull request
//     url: String,
//     // body - for body length
//     body: usize,
// }

#[derive(Debug)]
struct Issues {
    repo: String,
    items: <Vec<Issue> as IntoIterator>::IntoIter,
    client: blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl Issues {
    fn of(repo: &str) -> Result<Self> {
        Ok(Issues {
            repo: repo.to_owned(),
            items: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: 100,
            total: 0,
        })
    }

    fn try_next(&mut self) -> Result<Option<Issue>> {
        if let Some(pr) = self.items.next() {
            return Ok(Some(pr));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url =  format!( "https://api.github.com/search/issues?q=repo:{} is:open type:pr&sort=created_at&order=asc&per_page={}&page={}", 
        self.repo, self.per_page, self.page);

        let response = self
            .client
            .get(&url)
            .header(header::USER_AGENT, "AwesomeBuilder")
            .send()?
            .json::<ApiResponse>()?;
        self.items = response.items.into_iter();
        self.total = response.total_count;
        Ok(self.items.next())
    }
}

impl Iterator for Issues {
    type Item = Result<Issue>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(pr)) => Some(Ok(pr)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

// #[allow(clippy::clippy::len_zero)]
fn main() -> Result<()> {
    //! defining the repo here.
    // let repo = "torvalds/linux";
    let repo = "octocat/Hello-World";

    for pr in Issues::of(repo)? {
        println!("PR: {}", pr?.title,);
        println!("date: {}", pr?.created_at);
    }

    // ! Oldest: rust-lang/rust#65819: Add `IntoIterator` impl for arrays by value (`for [T; N]`)
    // ! Longest body: rust-lang/rust#79135: stabilize `#![feature(min_const_generics)]` in 1.50

    Ok(())
}
