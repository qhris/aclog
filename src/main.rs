#[macro_use]
extern crate clap;
extern crate chrono;
extern crate time;

use time::Duration;
// use chrono::{DateTime, UTC};
use chrono::prelude::*;

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

struct activity_data<'a> {
    activity: &'a str,
    date_range: (DateTime<UTC>, DateTime<UTC>),
}

fn create_activity(hours: i64, activity: &str) -> activity_data {
    let duration = Duration::hours(hours);
    let end = UTC::now();
    let start = end - duration;

    activity_data {
        activity,
        date_range: (start, end),
    }
}

#[test]
fn test_create_activity() {
    assert_eq!(create_activity(1, "test").activity, "test");
    let activity = create_activity(1, "test");
    assert_eq!(activity.date_range.1.signed_duration_since(activity.date_range.0),
               Duration::hours(1));
}

fn main() {
    let matches = parse_cmd_matches();
    let db_path = find_db_path();

    let activity = create_activity(1, matches.value_of("ACTIVITY").unwrap());

    println!("activity: {}", activity.activity);
    println!("path: {}", db_path.display());
}
