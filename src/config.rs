use std::{env, process};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, name = "ShodanCLI", version, about = "Simple Shodan CLI tool for fast check ip addresses", long_about = r#"
ShodanCLI

Author: Sergey Malinkin <malinkinsa@gmail.com>

This is a simple command-line tool for interacting with the Shodan API to quickly check IP addresses.
You can find the source code of this tool in the GitHub repository: https://github.com/malinkinsa/shodancli"#)]
pub struct ShodanCliArgs {
    #[arg(short, long = "targets", value_delimiter = ',')]
    /// Target IP addresses separated by comma
    pub targets: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub api_key: String,
    pub targets: Vec<String>,
}


impl Config {
    pub fn new(api_key: String, targets: Vec<String>) -> Self {
        Config {
            api_key,
            targets,
        }
    }

    pub fn get_shodan_api_key() -> String {
        if env::var("SHODAN_API_KEY").is_ok() {
            env::var("SHODAN_API_KEY").unwrap()
        } else {
            eprintln!("Please set the SHODAN_API_KEY environment variable to your Shodan API key. You can obtain an API key from your account on the Shodan website (https://account.shodan.io/).");
            process::exit(1);
        }
    }
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
}