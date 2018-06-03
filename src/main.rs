extern crate git2;
extern crate log;
#[macro_use]
extern crate structopt;
extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate simple_error;

use git2::Repository;
use regex::Regex;
use serde_json::Value;
use simple_error::SimpleError;
use std::error::Error;
use std::io;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "addupstream",
    about = "A small cli to automatically add upstream remotes to a git project. "
)]
struct Opt {
    #[structopt(
        short = "o",
        long = "origin",
        default_value = "origin",
        help = "The name of your default remote. Set this if not using origin"
    )]
    origin: String,

    /// Output file
    #[structopt(
        short = "u",
        long = "upstream",
        default_value = "upstream",
        help = "The name of your default remote. Set this if you don't want upstream as the remote name"
    )]
    upstream: String,
}

fn main() -> Result<(), Box<Error>> {
    let opt = Opt::from_args();
    let origin_url = get_origin_remote(&opt.origin)?;

    let repo_name = get_repo_name(origin_url.as_str())?;
    println!("Finding upstream for {}...", repo_name);

    let remote_url = get_fork_remote_url(&repo_name)?;

    let mut input = String::new();
    println!(
        "I found this upstream: {}. Do You want to add it? (y,n)",
        remote_url
    );

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.as_ref() {
        "yes\n" | "y\n" => {
            add_remote(remote_url, opt.upstream).unwrap();
            println!("Done!");
        }
        _ => println!("Exiting"),
    }

    Ok(())
}

// Returns for example 'git@github.com:PierreZ/addupstream.git'
fn get_origin_remote(_origin_name: &str) -> Result<String, Box<Error>> {
    let repo = Repository::open_from_env()?;
    let remote = repo.find_remote(_origin_name)?;
    match remote.url() {
        Some(remote_url) => Ok(String::from(remote_url)),
        None => Err(Box::new(SimpleError::new(format!(
            "Could not find an remote called '{}'",
            _origin_name
        )))),
    }
}

fn get_repo_name(url: &str) -> Result<String, Box<Error>> {
    let re = Regex::new(r":.*.git$").unwrap();

    let caps = re.captures(url)
        .ok_or_else(|| SimpleError::new(format!("Could not find an repo name on '{}'", url)))?;

    let mut user_repo = caps.get(0).unwrap().as_str().to_string();

    user_repo.remove(0); // removing :
    let (first, _) = user_repo.split_at(user_repo.len() - 4); // removing .git

    Ok(String::from(first))
}

#[test]
fn test_get_repo_from_url() {
    assert_eq!(
        "PierreZ/addupstream",
        get_repo_name(&String::from("git@github.com:PierreZ/addupstream.git")).unwrap()
    );
    assert_eq!(
        "PierreZ/warp10-platform",
        get_repo_name(&String::from("git@github.com:PierreZ/warp10-platform.git")).unwrap()
    );
}

fn get_fork_remote_url(repo: &str) -> Result<String, Box<Error>> {
    let url = format!("https://api.github.com/repos/{}", repo);
    let body = reqwest::get(&url)?.text()?;

    let json: Value = serde_json::from_str(body.as_str())?;

    let clone_url = json["parent"]["clone_url"]
        .as_str()
        .ok_or_else(|| SimpleError::new("Couldn't find any parent. Did you fork the project?"))?;

    Ok(String::from(clone_url))
}

fn add_remote(remote_url: String, upstream_name: String) -> Result<(), Box<Error>> {
    let repo = Repository::open_from_env()?;
    repo.remote(upstream_name.as_str(), remote_url.as_str())?;
    Ok(())
}
