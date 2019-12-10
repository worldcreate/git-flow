
use super::{Issue, IssueId};

pub trait IssueRepository {
    fn create_issue(&self, issue: &Issue) -> Result<IssueId, IssueRepositoryError>;
}

pub struct IssueRepositoryError {

}