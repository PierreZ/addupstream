use git2::{Error, Repository};
use std::env;
use std::string;
use regex::Regex;
use hyper::Client;
use hyper::Url;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::UserAgent;
use std::process;
use serde_json;
use std::io::Read;
use serde_json::Value;


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

    let re = Regex::new(r":.*.git$").unwrap();
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
pub fn add_remote(remote_name: string::String, upstream_custom: string::String) {

    // Get current path
    let current_path = env::current_dir().unwrap();

    // Open current directory as a git repo
    let repo = match Repository::open(current_path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}, not a git repo, exiting", e),
    };

    match repo.remote(upstream_custom.as_str(), remote_name.as_str()) {
        Ok(r) => r,
        Err(e) => panic!("failed to add remote: {}, exiting", e),
    };
}

pub fn find_upstream(repo_name: string::String) -> string::String {

    let mut url = String::from("https://api.github.com/repos/");
    url.push_str(repo_name.as_str());

    let uri = Url::parse(url.as_str()).ok().expect("malformed url");

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let mut res = match client.get(uri)
              .header(UserAgent("PierreZ/addupstream".to_string()))
              .send() {
        Ok(res) => res,
        Err(e) => {
            println!("Error accessing github API: {}", e);
            process::exit(1);
        }
    };

    if !res.status.is_success() {
        println!("Error accessing github API, status code is {}", res.status);
        process::exit(1);
    }

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let v: Value = serde_json::from_str(body.as_str()).unwrap();

    return String::from(v["parent"]["clone_url"].as_str().unwrap_or(""));
}