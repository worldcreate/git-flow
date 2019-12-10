extern crate config;

extern crate domain;

use std::fs::OpenOptions;
use std::path::Path;

pub use domain::issue::{self, create_issue_usecase};

pub struct GitFlow<'a> {
    issue_usecase: &'a dyn issue::IssueUseCase
}

#[derive(Debug)]
pub enum GitFlowError {
    NotFound,
    ConfigError
}

impl std::convert::From<config::ConfigError> for GitFlowError {
    fn from(e: config::ConfigError) -> Self {
        GitFlowError::ConfigError
    }
}

impl std::fmt::Display for GitFlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            GitFlowError::NotFound => f.write_str("NotFound"),
            GitFlowError::ConfigError => f.write_str("ConfigError")
        }
    }
}

impl std::error::Error for GitFlowError {
}

impl<'a> GitFlow<'a> {
    pub fn new(issue_usecase: &'a dyn issue::IssueUseCase) -> Result<GitFlow<'a>, GitFlowError> {
        Ok(GitFlow {
            issue_usecase
        })
    }
}


#[cfg(test)]
mod test {
    use crate::git_flow::*;

    #[test]
    fn test_new() {
        let usecase = issue::create_issue_usecase();
        assert!(match GitFlow::new(&usecase) {
            Ok(_) => true,
            Err(e) => {
                eprintln!("{}", e);
                false
            }
        })
    }
}