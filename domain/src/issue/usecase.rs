

use super::{Issue, IssueId};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::super::{UseCase, UseCaseError};


pub struct CreateIssue<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub todos: Vec<&'a str>,
    pub labels: Vec<&'a str>
}

pub fn create_issue_usecase<'a>() -> impl UseCase<&'a CreateIssue<'a>, ()> {
    return IssueUseCaseInteractor {}
}

struct IssueUseCaseInteractor {}

impl UseCase<&CreateIssue<'_>, ()> for IssueUseCaseInteractor {
    fn exec(&self, create_issue: &CreateIssue) -> Result<(), UseCaseError>{
        Issue::new(create_issue.title, create_issue.labels.clone());
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_create_issue() {
        let issue = CreateIssue {
            title: "title",
            description: "description",
            todos: vec!["todo1", "todo2"],
            labels: vec!("bug", "feature")
        } ;
        create_issue_usecase().exec(&issue);
    }
}
