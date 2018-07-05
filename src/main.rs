extern crate dialoguer;
extern crate failure;
extern crate url;

use std::collections::VecDeque;

use dialoguer::Select;
use failure::Error;
use url::Url;

fn main() -> Result<(), Error> {
    let mut urls: Vec<Url> = Vec::new();
    let mut queue: VecDeque<Url> = VecDeque::new();

    std::env::args()
        .skip(1)
        .for_each(|arg| match Url::parse(&arg) {
            Ok(url) => queue.push_back(url),
            Err(_) => println!("Could not parse {:?} as URL", arg),
        });

    while let Some(url) = queue.pop_front() {
        for (_, value) in url.query_pairs() {
            if let Ok(url) = Url::parse(&value) {
                queue.push_back(url);
            }
        }
        urls.push(url);
    }

    if urls.len() < 1 {
        eprintln!("No URLs detected");
    }

    let options = urls.iter().map(|url| url.as_str()).collect::<Vec<_>>();
    let selection = Select::new().items(&options).interact()?;

    println!("Chosen {:?}", options[selection]);

    Ok(())
}
