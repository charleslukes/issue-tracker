mod file_data;
mod issues_repo;
mod custom_error;
mod shared;

use std::{env};
use custom_error::Error;
use crate::{file_data::FileData, issues_repo::Issues};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let file_data = FileData::new(&args[1]);
    let todos = FileData::create_todo(&file_data.file_content);
    let user_issues = Issues::get_issues().await?;
    println!("{:?}", user_issues);
    Issues::create_all_issues(todos, &user_issues).await?;
    Ok(())
}
