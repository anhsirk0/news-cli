use std::env;
use colored::*;
use serde_derive::{Deserialize, Serialize};
use reqwest::Url;
use exitfailure::ExitFailure;

#[derive(Serialize, Deserialize, Debug)]
pub struct News {
    section: String,
    subsection: String,
    title: String,
    r#abstract: String,
    url: String,
    byline: String,
    published_date: String,
}

impl News {
    pub fn print(&self, num: usize) {
        println!("\n{}. {}", num, self.title.blue());
        println!("{}",self.r#abstract.bright_green());
        println!("{}\n",self.url.yellow());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    status: String,
    section: String,
    pub num_results: usize,
    pub results: Vec<News>,
}

impl Response {
    pub async fn new(section: String) -> Result<Self, ExitFailure> {
        let api_key: String = env::var("NEWS_API_KEY").unwrap();
        let url = format!(
            "https://api.nytimes.com/svc/topstories/v2/{}.json?api-key={}",
            section, api_key);
        let url = Url::parse(&*url)?;
        let response = reqwest::get(url).await?.json().await?;
        Ok(response)
    }
}

pub struct Query {
    pub section: String,
    pub limit: usize
}

pub fn parse(args: Vec<String>) -> Query {
    let categories: Vec<String> = vec![
        "arts".to_string(),
        "automobiles".to_string(),
        "books".to_string(),
        "business".to_string(),
        "fashion".to_string(),
        "food".to_string(),
        "health".to_string(),
        "home".to_string(),
        "insider".to_string(),
        "magazine".to_string(),
        "movies".to_string(),
        "nyregion".to_string(),
        "obituaries".to_string(),
        "opinion".to_string(),
        "politics".to_string(),
        "realestate".to_string(),
        "science".to_string(),
        "sports".to_string(),
        "sundayreview".to_string(),
        "technology".to_string(),
        "theater".to_string(),
        "t-magazine".to_string(),
        "travel".to_string(),
        "upshot".to_string(),
        "us".to_string(),
        "world".to_string()
        ];
                                                                                                           
    let mut query: Query = Query{section: "world".to_string(), limit: 99};
    let len: usize = args.len();

    if len >= 3 {
        match args[2].parse::<usize>() {
            Ok(v) => query.limit = v,
            _ => query.limit = 0,
        }
    }
    
    if len >= 2 {
        if args[1] == "help" {
            query.limit = 0
        } else if args[1] == "list" {
            for category in categories {
                println!("{}", category.bright_green())
            }
            query.section = "none".to_string();
        } else {
            for category in categories {
                if category == args[1] {
                    query.section = category;
                }
            }
        }
    }
    query
}
