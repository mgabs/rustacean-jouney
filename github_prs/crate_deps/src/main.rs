use reqwest::{header, Result};
use serde::Deserialize;
// use reqwest::{blocking, };

#[derive(Deserialize, Debug)]
struct ApiResponse {
    dependencies: Vec<Dependencies>,
    meta: Meta,
}

#[derive(Deserialize, Debug)]
struct Meta {
    total: u32,
}

#[derive(Deserialize, Debug)]
struct Dependencies {
    id: u32,
    version_id: u32,
    crate_id: String,
    req: String,
    optional: bool,
}

#[derive(Debug)]
struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependencies> as IntoIterator>::IntoIter,
    client: reqwest::blocking::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
        Ok(ReverseDependencies {
            crate_id: crate_id.to_owned(),
            dependencies: vec![].into_iter(),
            client: reqwest::blocking::Client::new(),
            page: 0,
            per_page: 100,
            total: 0,
        })
    }

    fn try_next(&mut self) -> Result<Option<Dependencies>> {
        if let Some(dep) = self.dependencies.next() {
            return Ok(Some(dep));
        }

        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        self.page += 1;
        let url = format!(
            "https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
            self.crate_id, self.page, self.per_page
        );
        // my implementation
        // let response = self.client.get(&url).header(header::USER_AGENT, "AwesomeBuilder").send().expect("Invalid json to parse");
        // if let Ok(a_response) = serde_json::from_str::<ApiResponse>(&response.text()?){
        //     self.dependencies = a_response.dependencies.into_iter();
        //     self.total = a_response.meta.total;
        //     // self.crate_id =self.dependencies
        // }

        // cookbook - fixed
        // let response = self
        //     .client
        //     .get(&url)
        //     .header(header::USER_AGENT, "AwesomeBuilder")
        //     .send()?
        //     .json::<ApiResponse>()?;
        // self.dependencies = response.dependencies.into_iter();
        // self.total = response.meta.total;

        // cookbook
        let response = self
            .client
            .get(&url)
            .header(header::USER_AGENT, "AwesomeBuilder")
            .send()?
            .json::<ApiResponse>()?;
        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;
        Ok(self.dependencies.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependencies>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}
#[allow(clippy::explicit_counter_loop)]
#[allow(clippy::never_loop)]
fn main() -> Result<()> {
    for dep in ReverseDependencies::of("tokio")? {
        // println!("reverse dependency: {:#?}", dep?);
        println!("{:#?}", dep?.crate_id);
        println!("{:#?}", dep?.req);
    }

    Ok(())
}
b