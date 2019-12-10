
pub mod usecase;
pub mod repository;
pub use usecase::IssueUseCase;
pub use usecase::create_issue_usecase;

pub struct Issue {
    id: Option<IssueId>,
    pub title: String,
    label: Vec<String>
}

#[derive(PartialEq, Debug)]
pub struct IssueId {
    pub id: i32
}

impl Issue {
    pub fn new(title: &str, label: Vec<&str>) -> Issue {
        Issue {id: None, title: title.to_string(), label: label.iter().map(|e| e.to_string()).collect()}
    }
}

#[cfg(test)]
mod test {
    use super::super::issue::*;
    #[test]
    fn test_create_issue() {
        usecase::create_issue_usecase().create_issue(
            &usecase::CreateIssue {
                title: "title",
                description: "description",
                todos: vec!["todo1", "todo2"],
                labels: vec!("bug", "feature")
            });
    }
}    
