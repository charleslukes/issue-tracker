mod custom_error;
mod file_data;
mod issues_repo;
mod questions;
mod service;
mod shared;

use crate::{file_data::FileData, issues_repo::Issues, questions::Question};
use custom_error::Error;
use service::Service;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_data = FileData::new(&args[1]);
    let todos = FileData::create_todo(&file_data.file_content);

    let all_questions = vec![
        Question {
            name: String::from("Whats your github username?"),
            value: String::from(""),
            kind: questions::QuestionType::USERNAME,
        },
        Question {
            name: String::from("Whats your repo name?"),
            value: String::from(""),
            kind: questions::QuestionType::REPONAME,
        },
    ];

    let answers = Question::ask_reply(all_questions);

    // init service
    // no need to loop through the answers we can get them by index
    let service = Service {
        username: &answers[0].answer,
        repo_name: &answers[1].answer,
    };

    let user_issues = Issues::get_issues(&service).await?;
    Issues::create_all_issues(todos, &user_issues, &service).await?;
    Ok(())
}
