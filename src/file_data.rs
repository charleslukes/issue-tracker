use std::fs;
use regex::Regex;
use crate::shared::TodoDetails;
pub struct FileData {
    pub file_content: String,
}

impl FileData {
    pub fn new(file_path: &String) -> FileData {
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

    pub fn create_todo(text: &String) -> Vec<TodoDetails> {
        let reg_ex = Regex::new(r"((todo|description)(.*?)(\n))").unwrap();
        let mut todo_issues: Vec<TodoDetails> = Vec::new();
        for cap in reg_ex.captures_iter(text) {
            let check_desc = cap[2].to_string().trim().to_owned();
            if check_desc.eq("todo") {
                let title_text = cap[3].to_string().trim().to_owned();
                todo_issues.push(TodoDetails {
                    title: FileData::remove_first_column_char(&':', &title_text),
                    body: String::new(),
                });
            };
            // add description
            if check_desc.eq("description") {
                let desc = cap[3].to_string();
                let todo_issue_length = todo_issues.len() - 1;
                todo_issues[todo_issue_length].body =
                    FileData::remove_first_column_char(&':', &desc);
            };
        }

        todo_issues
    }
}
