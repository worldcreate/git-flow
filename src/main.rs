
mod git_flow;

use git_flow::GitFlowError;
use std::fs::OpenOptions;

fn main() {
    let mut settings = config::Config::default();

    let mut path = dirs::home_dir().ok_or(GitFlowError::NotFound)
        .expect("ホームディレクトリの取得に失敗しました");
    path.push(".gitflowrc.yml");

    if !path.exists() {
        OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path.clone()).map_err(|e| {
            GitFlowError::NotFound
        }).expect(&format!("{:?} の作成に失敗しました", path.clone()));
    }

    settings
        .merge(config::File::from(path)).expect("");

    let usecase = git_flow::create_issue_usecase();

    std::process::exit(match git_flow::GitFlow::new(&usecase) {
        Ok(gitflow) => {
            0
        },
        Err(_) => {
            -1
        }
    })
}
