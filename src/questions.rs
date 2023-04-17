use std::io;

#[derive(Debug, Clone)]
pub struct Question {
    pub value: String,
    pub name: String,
    pub kind: QuestionType,
}

#[derive(Debug, Clone)]
pub enum QuestionType {
    USERNAME,
    REPONAME,
}


#[derive(Debug)]
pub struct Reply {
    pub answer: String,
    pub question_type: QuestionType,
}

impl Question {
    pub fn ask_reply(questions: Vec<Question>) -> Vec<Reply> {
        let all_questions = questions.clone();
        let mut reply: Vec<Reply> = Vec::new();

        for mut question in all_questions {
            println!("{:?}", &question.name);
            io::stdin()
                .read_line(&mut question.value)
                .expect("Failed to read line");

            reply.push(Reply {
                answer: question.value.trim().to_owned(),
                question_type: question.kind,
            });
        }

        reply
    }
}
