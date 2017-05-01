extern crate clap;
extern crate git2;
extern crate regex;

use clap::{Arg, App};
use std::process;

pub mod git;

fn main() {
    let matches = App::new("addupstream")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("default_remote_name")
                 .help("The name of your default remote. Set this if not using origin")
                 .long("remote_name")
                 .short("r")
                 .takes_value(true)
                 .required(false))
        .get_matches();

    let remote_name = matches.value_of("default_remote_name")
        .unwrap_or("origin"); // Default is for the default branch is origin

    let local_repo_name = match git::get_repo_from_current_folder(remote_name) {
        Ok(r) => r, 
        Err(e) => {
            println!("Error accessing local repo name: {}", e);
            process::exit(1);
        }
    };

    println!("Finding upstream for {}...", local_repo_name);

    // let upstream_url = find_upstream(local_repo_name);
    // println!("I found this upstream: {}. Do You want to add it? (y,n)",
    //          upstream_url);



}
