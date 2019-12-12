

use super::{Issue, IssueId};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub struct CreateIssue<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub todos: Vec<&'a str>,
    pub labels: Vec<&'a str>
}

pub fn create_issue_usecase() -> impl IssueUseCase {
    return IssueUseCaseInteractor {}
}

struct IssueUseCaseInteractor {}

impl IssueUseCase for IssueUseCaseInteractor {
    fn create_issue(&self, create_issue: &CreateIssue) {
        Issue::new(create_issue.title, create_issue.labels.clone());
    }
}
