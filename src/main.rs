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

    fn search_todo(text: &String) -> Vec<TodoDetails> {
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
