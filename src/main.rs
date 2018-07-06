extern crate dialoguer;
extern crate failure;
extern crate open;
extern crate url;

use std::collections::VecDeque;
use std::io;

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

    if urls.is_empty() {
        eprintln!("No URLs detected");
        return Err(io::Error::from_raw_os_error(1).into());
    }

    let options = urls.iter().map(|url| url.as_str()).collect::<Vec<_>>();
    let selected_index = Select::new().items(&options).interact()?;

    match options.get(selected_index) {
        Some(selection) => open::that(selection)
            .and_then(|status| match status.code() {
                Some(code) => if code == 0 {
                    Ok(())
                } else {
                    Err(io::Error::from_raw_os_error(code))
                },
                None => Err(io::Error::from_raw_os_error(1)),
            })
            .or_else(|error| Err(error.into())),
        None => Err(io::Error::from_raw_os_error(1).into()),
    }
}
