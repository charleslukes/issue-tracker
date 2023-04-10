use regex::Regex;
use std::env;
use std::fs;
use reqwest::Error;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let file_data = FileData::new(&args[1]);
    let todo_details = FileData::search_todo(&file_data.file_content);
    println!("todo details {:?}", todo_details);
    let res = make_api_call().await;
    println!("Result ==> {:?}", res);
}

struct FileData {
    file_content: String,
}
#[derive(Debug)]
#[allow(dead_code)]
struct TodoDetails {
    todo: String,
    description: String,
}

impl FileData {
    fn new(file_path: &String) -> FileData {
        let file_content = fs::read_to_string(file_path).expect("Invalid file");
        let content_to_lowercase = file_content.to_lowercase();
        FileData {
            file_content: content_to_lowercase,
        }
    }

    fn remove_first_column_char(value: &char, text: &String) -> String {
        let column = text.chars().nth(0).unwrap();
        if column.eq(value) {
            let mut iter = text.chars();
            iter.by_ref().nth(0);
            let slice_text = iter.as_str().trim();
            return slice_text.to_owned();
        }

        text.to_string()
    }
        let reg_ex = Regex::new(r"((todo|description)(.*?)(\n))").unwrap();
        let mut todo_issues: Vec<TodoDetails> = Vec::new();
        for cap in reg_ex.captures_iter(text) {
            let check_desc = cap[2].to_string().trim().to_owned();
            if check_desc.eq("todo") {
                todo_issues.push(TodoDetails {
                    todo: cap[3].to_string().trim().to_owned(),
                    description: String::new(),
                });
            };
            // add description
            if check_desc.eq("description") {
                let todo_issue_length = todo_issues.len() - 1;
                todo_issues[todo_issue_length].description = cap[3].to_string();
            };
        }

        todo_issues
    }
}

#[allow(dead_code)]
struct Issues {
    user_issues: Vec<RepoInfo>,
}

impl Issues {
    async fn get_issues() -> Result<Vec<RepoInfo>, Error> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/repos/charleslukes/issue-tracker/issues")
            .header("User-Agent", "request")
            .send()
            .await?;

        let result: Vec<RepoInfo> = response.json().await?;
        Ok(result)
    }

    async fn create_issue(todo: TodoDetails) -> Result<RepoInfo, Error> {
        let client = reqwest::Client::new();
        let body = json!({
            "title": todo.title,
            "body": todo.body
        });

        let response = client
            .post("https://api.github.com/repos/charleslukes/issue-tracker/issues")
            .header("User-Agent", "request")
            .header(AUTHORIZATION, "Bearer xxxx")
            .header(ACCEPT, "application/vnd.github+json")
            .json(&body)
            .send()
            .await?;

        let res = response.json().await?;
        Ok(res)
    }

    async fn create_all_issues(
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
