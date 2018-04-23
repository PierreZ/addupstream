#[macro_use]
extern crate log;
extern crate git2;
#[macro_use]
extern crate structopt;
extern crate regex;
extern crate reqwest;
extern crate serde_json;

use git2::Repository;
use regex::Regex;
use serde_json::Value;
use std::error::Error;
use std::io;
use std::process;
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

fn main() {
    let opt = Opt::from_args();
    let origin_url = match get_origin_remote(&opt.origin) {
        Ok(origin_url) => origin_url,
        Err(error) => {
            error!("There was a problem to find the origin: {:?}", error);
            process::exit(1);
        }
    };

    let repo_name = match get_repo_name(origin_url.as_str()) {
        Ok(repo_name) => repo_name,
        Err(error) => {
            error!("There was a problem to find repo name: {:?}", error);
            process::exit(1);
        }
    };
    println!("Finding upstream for {}...", repo_name);

    let remote_url = match get_fork_remote_url(&repo_name) {
        Ok(remote_url) => remote_url,
        Err(error) => {
            error!("There was a problem to find repo name: {:?}", error);
            process::exit(1);
        }
    };

    let mut input = String::new();

    println!(
        "I found this upstream: {}. Do You want to add it?",
        remote_url
    );
    println!("(y,n)");

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

    process::exit(0);
}

// Returns for example 'git@github.com:PierreZ/addupstream.git'
fn get_origin_remote(_origin_name: &str) -> Result<String, Box<Error>> {
    let repo = Repository::open_from_env()?;
    let remote = repo.find_remote(_origin_name)?;
    match remote.url() {
        Some(remote_url) => Ok(String::from(remote_url)),
        None => panic!("Could not find an remote called '{}'", _origin_name),
    }
}

fn get_repo_name(url: &str) -> Result<String, Box<Error>> {
    let re = Regex::new(r":.*.git$").unwrap();

    let caps = match re.captures(url) {
        Some(matches) => matches,
        None => panic!("Could not find an repo name on '{}'", url),
    };

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

    let clone_url = match json["parent"]["clone_url"].as_str() {
        Some(clone_url) => clone_url,
        None => panic!("Couldn't find any parent. Did you fork the project?"),
    };

    Ok(String::from(clone_url))
}

fn add_remote(remote_url: String, upstream_name: String) -> Result<(), Box<Error>> {
    let repo = Repository::open_from_env()?;
    repo.remote(upstream_name.as_str(), remote_url.as_str())?;
    Ok(())
}
