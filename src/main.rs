extern crate clap;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate reqwest;
#[macro_use]
extern crate prettytable;

use clap::App;

pub static URL: &'static str = "http://readrust.net/rust2018/feed.json";

#[derive(Debug, Deserialize, Serialize)]
struct Feed {
    version: String,
    title: String,
    home_page_url: String,
    feed_url: String,
    description: String,
    author: Author,
    items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    id: String,
    title: String,
    content_text: String,
    url: String,
    date_published: String,
    author: Author,
}

fn get_feed() -> Feed {
    let client = reqwest::Client::new();

    let mut resp = client.get(URL).send().unwrap();

    assert!(resp.status().is_success());

    let data = resp.text().unwrap();

    serde_json::from_str::<Feed>(&data).unwrap()
}

fn print_count(feed: &Feed) {
    println!("Number of posts: {}", feed.items.len());
}

fn print_feed_table<I: Iterator<Item=Item>>(items: I) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Title", "Author", "Link"]);

    for item in items {
        let title = if item.title.len() >= 50 {
                        &item.title[0..50]
                    } else {
                        &item.title
                    };

        table.add_row(row![title, item.author.name, item.url]);
    }

    table.printstd();
}

fn main() {
    let matches = App::new("readrust")
                          .version("0.1")
                          .author("Florian G. <florian.gilcher@asquera.de>")
                          .about("Reads readrust.net")
                          .args_from_usage(
                              "-n, --number=[NUMBER] 'Number of posts'
                              -c, --count            'Just counts the number'")
                          .get_matches();

    let feed = get_feed();

    if matches.is_present("count") {
        print_count(&feed);
    } else {
        let iter = feed.items.into_iter();

        if let Some(string) = matches.value_of("number") {
            let number = string.parse().unwrap();
            print_feed_table(iter.take(number))
        } else {
            print_feed_table(iter)
        }
    }
}
