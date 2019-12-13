
pub mod usecase;
pub mod repository;

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

