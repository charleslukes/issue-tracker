use crate::{
    custom_error::Error,
    service::{Service, ServiceEnum},
    shared::TodoDetails,
};
use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION};
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
    pub async fn get_issues(service: &Service<'_>) -> Result<Vec<RepoInfo>, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", "request".parse().unwrap());

        let response = service.call(ServiceEnum::GET, Some(headers), None).await?;
        let result: Vec<RepoInfo> = response.json().await?;
        Ok(result)
    }

    pub async fn create_issue(todo: TodoDetails, service: &Service<'_>) -> Result<RepoInfo, Error> {
        let body = json!({
            "title": todo.title,
            "body": todo.body
        });

        let auth_key = env::var("AUTH_KEY");
        match auth_key {
            Ok(key) => {
                let mut bearer = String::from("Bearer ");
                bearer.push_str(&key.to_owned());

                let mut headers = HeaderMap::new();
                headers.insert("User-Agent", "request".parse().unwrap());
                headers.insert(AUTHORIZATION, bearer.parse().unwrap());
                headers.insert(ACCEPT, "application/vnd.github+json".parse().unwrap());

                let response = service
                    .call(ServiceEnum::POST, Some(headers), Some(body))
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
        service: &Service<'_>,
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
                    Issues::create_issue(todo, &service).await?;
                }
            };
        }

        Ok(())
    }
}
