use crate::{custom_error::Error, shared::TodoDetails};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RepoInfo {
    id: u32,
    number: u32,
    title: String,
}

#[allow(dead_code)]
pub struct Issues {
    user_issues: Vec<RepoInfo>,
}

impl Issues {
    pub async fn get_issues() -> Result<Vec<RepoInfo>, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/repos/charleslukes/issue-tracker/issues")
            .header("User-Agent", "request")
            .send()
            .await?;

        let result: Vec<RepoInfo> = response.json().await?;
        Ok(result)
    }

    pub async fn create_issue(todo: TodoDetails) -> Result<RepoInfo, Error> {
        let client = reqwest::Client::new();
        let body = json!({
            "title": todo.title,
            "body": todo.body
        });

        let auth_key = env::var("AUTH_KEY");
        match auth_key {
            Ok(key) => {
                let mut bearer = String::from("Bearer ");
                bearer.push_str(&key.to_owned());

                let response = client
                    .post("https://api.github.com/repos/charleslukes/issue-tracker/issues")
                    .header("User-Agent", "request")
                    .header(AUTHORIZATION, bearer)
                    .header(ACCEPT, "application/vnd.github+json")
                    .json(&body)
                    .send()
                    .await?;

                let res: RepoInfo = response.json().await?;
                Ok(res)
            }
            Err(err) => Err(Error::AuthKeyError(err)),
        }
    }

    pub async fn create_all_issues(
        all_todos: Vec<TodoDetails>,
        all_repos: &Vec<RepoInfo>,
    ) -> Result<(), Error> {
        for todo in all_todos {
            // check if issue is already created
            let title = todo.title.trim().to_owned();
            let mut all_repos_iter = all_repos.into_iter();
            let check_issue = all_repos_iter.find(|&x| x.title.trim().eq_ignore_ascii_case(&title));
            match check_issue {
                Some(_) => {
                    continue;
                }
                None => {
                    // if none found create issue
                    Issues::create_issue(todo).await?;
                }
            };
        }

        Ok(())
    }
}
