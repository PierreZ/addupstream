use git2::{Error, Repository};
use std::env;
use std::string;
use regex::Regex;

/// if repo name is not setted in cli, we need to get it from git
pub fn get_repo_from_current_folder(remote_name: &str) -> Result<string::String, Error> {
    // Get current repo
    let current_path = env::current_dir().unwrap();
    let repo = match Repository::open(current_path) {
        Ok(repo) => repo,
        Err(e) => return Err(e),
    };

    match repo.find_remote(remote_name) {
        Ok(r) => {
            let url = String::from(r.url().unwrap());
            return Ok(get_repo_from_url(url));
        }
        Err(e) => return Err(e),
    };
}

/// get_repo_from_url is transforming a full repo url into user/repo
pub fn get_repo_from_url(url: string::String) -> string::String {
    let re = Regex::new(r":[a-zA-Z0-9]*/[a-zA-Z0-9]*.git$").unwrap();
    let mut user_repo = String::new();

    for cap in re.captures_iter(url.as_str()) {
        user_repo = String::from(&cap[0]);
    }

    user_repo.remove(0); // removing :
    let (first, _) = user_repo.split_at(user_repo.len() - 4);

    return String::from(first);
}

#[test]
fn test_get_repo_from_url() {
    assert_eq!("PierreZ/addupstream",
               get_repo_from_url(String::from("git@github.com:PierreZ/addupstream.git")));
}

/// add_remote is adding the remote to the current directory
pub fn add_remote(remote_name: string::String) {

    // Get current path
    let current_path = env::current_dir().unwrap();

    // Open current directory as a git repo
    let repo = match Repository::open(current_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}, not a git repo, exiting", e),
    };

    match repo.remote("upstream", remote_name.as_str()) {
        Ok(r) => r,
        Err(e) => panic!("failed to add remote: {}, exiting", e),
    };
}

// pub fn find_upstream(repo_name: string::String) -> string::String {

// }