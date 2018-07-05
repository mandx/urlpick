extern crate dialoguer;
extern crate failure;
extern crate url;

use std::collections::VecDeque;

use dialoguer::Select;
use failure::Error;
use url::{Url, ParseError};

fn sub_parse(url: &str) -> Option<Url> {
    match Url::parse(url) {
        Ok(parsed) => {
            Some(parsed)
        },
        Err(_) => None,
    }
}

fn main() {

    let urls: Vec<Url> = Vec::new();
    let queue: VecDeque<Url> = VecDeque::new();

    std::env::args().skip(1).for_each(|arg| match Url::parse(&arg) {
        Ok(url) => queue.push(),
        Err(_) => {
            println!("Could not parse {:?} as URL", arg);
            None
        },
    });

    println!("{:?}", urls.collect::<Vec<Url>>());
}
