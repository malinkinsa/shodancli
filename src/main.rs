use std::{env, process};
use std::fmt::{Display, Formatter};
use clap::Parser;
use reqwest;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(author, name = "ShodanCLI", version, about = "Simple Shodan CLI tool for fast check ip addresses", long_about = r#"
ShodanCLI

Author: Sergey Malinkin <malinkinsa@gmail.com>

This is a simple command-line tool for interacting with the Shodan API to quickly check IP addresses.
You can find the source code of this tool in the GitHub repository: https://github.com/malinkinsa/shodancli"#)]
struct ShodanCliArgs {
    #[arg(short, long = "targets", value_delimiter = ',')]
    /// Target IP addresses separated by comma
    targets: Vec<String>,
}

#[derive(Debug, Clone)]
struct Config {
    api_key: String,
    targets: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ShodanHostData {
    asn: String,
    city: String,
    country_code: String,
    country_name: String,
    data: Vec<ShodanPortData>,
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
    tags: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct ShodanPortData {
    #[serde(rename = "_shodan")]
    shodan: Shodan,
    cpe: Option<Vec<String>>,
    cpe23: Option<Vec<String>>,
    data: Option<String>,
    domains: Option<Vec<String>>,
    hash: i64,
    hostname: Option<Vec<String>>,
    info: Option<String>,
    product: Option<String>,
    port: i64,
    transport: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Shodan {
    crawler: String,
    id: String,
    module: String,
    ptr: Option<bool>,
    region: String,
}

impl Display for ShodanHostData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ports = {
            let mut ports = self.ports.clone();
            ports.sort();
            ports.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ")
        };

        let mut shodan_ports_data: String = String::new();
        for port_data in &self.data {
            shodan_ports_data.push_str(&format!(
                "Port: {}/{} | {}\n",
                port_data.port,
                port_data.transport,
                port_data.product.as_ref().map(|v| v.to_string())
                    .unwrap_or_else(|| String::from("None")),
            ));
        }

        let tags = if let Some(tags) = &self.tags {
            if tags.is_empty() {
                String::from("None")
            } else {
                tags.iter().map(|v|v.to_string()).collect::<Vec<_>>().join(", ")
            }
        } else {
            String::from("None")
        };

        write!(
            f,
            "IP:           {}\n\
            ASN:          {}\n\
            City:         {}\n\
            Country code: {}\n\
            Country name: {}\n\
            ISP:          {}\n\
            Last update:  {}\n\
            Organization: {}\n\
            OS:           {}\n\
            Port list:    {}\n\
            Comprehensive information about ports:\n\
                          {}\n\
            Region code:  {}\n\
            Tags:         {}\n\
            Link:         {}\n\
            ",
            self.ip_str,
            self.asn,
            self.city,
            self.country_code,
            self.country_name,
            self.isp,
            self.last_update,
            self.org,
            self.os,
            ports,
            shodan_ports_data,
            self.region_code,
            tags,
            format!("https://www.shodan.io/host/{}", self.ip_str),
        )
    }
}

impl Config {
    fn new(api_key: String, targets: Vec<String>) -> Self {
        Config {
            api_key,
            targets,
        }
    }

    fn get_shodan_api_key() -> String {
        if env::var("SHODAN_API_KEY").is_ok() {
            env::var("SHODAN_API_KEY").unwrap()
        } else {
            eprintln!("Please set the SHODAN_API_KEY environment variable to your Shodan API key. You can obtain an API key from your account on the Shodan website (https://account.shodan.io/).");
            process::exit(1);
        }
    }
}

async fn fetch_host_data(cfg:Config, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url: String = format!(
        "https://api.shodan.io/shodan/host/{}?key={}", target.trim(), cfg.api_key
    );
    let response: Response = reqwest::get(url).await?;

    if response.status() == reqwest::StatusCode::OK {
        let body: ShodanHostData = response.json::<ShodanHostData>().await?;
        println!("{}", body);
    } else {
        println!("There are no entries for: {}", target);
    }

    Ok(())
}

async fn fetch_data_for_targets(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = cfg.targets.iter().map(|target| {
        fetch_host_data(cfg.clone(), target)
    });

    futures::future::join_all(tasks).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: ShodanCliArgs = ShodanCliArgs::parse();
    let api_key:String = Config::get_shodan_api_key();
    let cfg:Config = Config::new(api_key, args.targets.clone());

    fetch_data_for_targets(cfg).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let api_key = "my_api_key".to_string();
        let targets = vec!["127.0.0.1".to_string(), "0.0.0.0".to_string()];
        let config = Config::new(api_key.clone(), targets.clone());

        assert_eq!(config.api_key, api_key);
        assert_eq!(config.targets, targets);
    }

    #[test]
    fn test_config_get_api_key() {
        env::set_var("SHODAN_API_KEY", "my_api_key");

        assert_eq!(Config::get_shodan_api_key(), "my_api_key".to_string());
    }

    #[tokio::test]
    async fn test_fetch_targets() {
        let api_key = "my_api_key".to_string();
        let targets = vec!["127.0.0.1".to_string(), "0.0.0.0".to_string()];
        let config = Config::new(api_key, targets);
        let result = fetch_data_for_targets(config).await;

        assert!(result.is_ok());
    }
}