use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::git_flow::issue::{Issue, IssueId};
use crate::git_flow::issue::repository::{IssueRepository, IssueRepositoryError};

struct GitlabIssueRepository {
    token: String
}

impl IssueRepository for GitlabIssueRepository {
    fn create_issue(&self, issue: &Issue) -> Result<IssueId, IssueRepositoryError> {

        let client = reqwest::Client::builder()
            .gzip(true)
            .danger_accept_invalid_certs(true)
            .build()
            .map_err(|e| IssueRepositoryError{})?;

        let mut params = HashMap::new();
        params.insert("title", issue.title.clone());

        let mut res = client.post("https://gitlab2.zyyx.jp/api/v4/projects/723/issues")
            .header("PRIVATE-TOKEN", "zkyA9csYPfhxKm8waQkz")
            .form(&params)
            .send()
            .map_err(|e| IssueRepositoryError{})?;

        let json: ResponseCreateIssue = res.json().map_err(|e| IssueRepositoryError{})?;

        Ok(IssueId {id: json.iid})
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct ResponseCreateIssue {
    iid: i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_issue() {
        let issue = Issue::new("test", vec!("bug", "feature"));
        assert!(match (GitlabIssueRepository {token: "".to_string()}).create_issue(&issue) {
            Ok(_) => true,
            Err(_) => false
        });

    }
}