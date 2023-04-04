use std::{env, process};
use std::fmt::{Display, Formatter};
use clap::Parser;
use reqwest;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Simple Shodan CLI tool for fast check ip addresses", long_about = "None")]
struct Args {
    #[arg(short, long = "target", value_delimiter = ',')]
    targets: Vec<String>,
}

#[derive(Debug, Clone)]
struct Config {
    api_key: String,
    targets: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Answer {
    asn: String,
    city: String,
    country_code: String,
    country_name: String,
    ip: i64,
    ip_str: String,
    isp: String,
    last_update: String,
    latitude: f64,
    longitude: f64,
    org: String,
    os: Value,
    ports: Vec<i64>,
    region_code: String,
    tag: Option<Vec<Value>>,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\
        IP: {} \n\
        ASN: {} \n\
        Org: {} \n\
        Open Ports: {:?} \n\
        Last Update: {} \n",
        self.ip_str, self.asn, self.org, self.ports, self.last_update)
    }
}

impl Config {
    fn new(api_key: String, targets: Vec<String>) -> Self {
        Config {
            api_key,
            targets,
        }
    }

    fn get_api_key() -> String {
        if env::var("SHODAN_API_KEY").is_ok() {
            env::var("SHODAN_API_KEY").unwrap()
        } else {
            eprintln!("Please provide SHODAN_API_KEY env var");
            process::exit(1);
        }
    }
}

async fn get_data(cfg:Config, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url: String = format!(
        "https://api.shodan.io/shodan/host/{}?key={}", target.trim(), cfg.api_key
    );
    let response: Response = reqwest::get(url).await?;

    if response.status() == reqwest::StatusCode::OK {
        let body: Answer = response.json::<Answer>().await?;
        println!("{}", body);
    } else {
        println!("There are no entries for: {}", target);
    }

    Ok(())
}

async fn fetch_targets(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = cfg.targets.iter().map(|target| {
        get_data(cfg.clone(), target)
    });

    futures::future::join_all(tasks).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let api_key:String = Config::get_api_key();
    let cfg:Config = Config::new(api_key, args.targets.clone());

    fetch_targets(cfg).await?;
    Ok(())
}
