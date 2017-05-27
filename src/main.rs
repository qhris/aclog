#[macro_use]
extern crate clap;

use std::env;
use std::path::{Path, PathBuf};

fn parse_cmd_matches<'a>() -> clap::ArgMatches<'a> {
    clap_app!(app =>
        (version: "0.1.0")
        (author: "Christoffer H. <b.christoffer.hjalmarsson@gmail.com>")
        (about: "Personal activity logger")
        (@arg time: -t --time +takes_value "Specifies the time the activity took")
        (@arg ACTIVITY: +required "The activity that will be logged.")
    ).get_matches()
}

fn find_db_path() -> PathBuf {
    // $HOME/.aclog/aclog_db.json
    let mut path = PathBuf::from(env::home_dir().unwrap());
    path.push(".aclog");
    path.push("aclog_db.json");

    path
}

fn main() {
    let matches = parse_cmd_matches();
    let db_path = find_db_path();

    println!("activity: {}", matches.value_of("ACTIVITY").unwrap());
    println!("path: {}", db_path.display());
}
