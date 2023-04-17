use reqwest::{header::HeaderMap, Response};
use serde_json::Value;

pub enum ServiceEnum {
    GET,
    POST,
}

pub struct Service<'a> {
    pub username: &'a String,
    pub repo_name: &'a String,
}

impl Service<'_> {
    fn issues_url(&self) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/issues",
            self.username, self.repo_name
        )
    }

    pub async fn call(
        &self,
        request_type: ServiceEnum,
        headers: Option<HeaderMap>,
        body: Option<Value>,
    ) -> Result<Response, reqwest::Error> {
        let client = reqwest::Client::new();

        match request_type {
            ServiceEnum::GET => {
                client
                    .get(self.issues_url())
                    .send()
                    .await
            }
            ServiceEnum::POST => {
                // check for headers
                let req_headers = match headers {
                    Some(h) => h,
                    None => {
                        panic!("Please include headers");
                    }
                };

                // check for body
                let req_body = match body {
                    Some(b) => b,
                    None => {
                        panic!("Please include body");
                    }
                };

                client
                    .post(self.issues_url())
                    .headers(req_headers)
                    .json(&req_body)
                    .send()
                    .await
            }
        }
    }
}
