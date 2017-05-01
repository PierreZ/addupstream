extern crate clap;
extern crate git2;
extern crate regex;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate hyper_native_tls;

// #[macro_use]
// extern crate serde_derive;

use clap::{Arg, App};
use std::process;
use std::io;

mod git;

fn main() {
    let matches = App::new("addupstream")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("origin_overwrite")
                 .help("The name of your default remote. Set this if not using origin")
                 .long("overwrite_origin")
                 .short("o")
                 .takes_value(true)
                 .required(false))
                         .arg(Arg::with_name("upstream_overwrite")
                 .help("The name of your default remote. Set this if you don't want upstream as the remote name")
                 .long("upstream_origin")
                 .short("u")
                 .takes_value(true)
                 .required(false))
        .get_matches();

    let remote_name = matches.value_of("origin_overwrite")
        .unwrap_or("origin"); // Default is for the default branch is origin

    let upstream = matches.value_of("upstream_overwrite")
        .unwrap_or("upstream"); // Default is for the upstream remote is upstream

    let local_repo_name = match git::get_repo_from_current_folder(remote_name) {
        Ok(r) => r, 
        Err(e) => {
            println!("Error accessing local repo name: {}", e);
            process::exit(1);
        }
    };

    println!("Finding upstream for {}...", local_repo_name);

    let upstream_url = git::find_upstream(local_repo_name);
    if upstream_url.len() == 0 {
        println!("Couldn't find any parent. Did you fork the project?");
        process::exit(1);
    }

    let mut input = String::new();

    println!("I found this upstream: {}. Do You want to add it? (y,n)",
             upstream_url);

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input == "y\n" {
        git::add_remote(upstream_url, String::from(upstream));
        println!("upstream added");
        process::exit(0);
    }

    println!("Exiting");
    process::exit(0);
}
