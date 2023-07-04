mod config;
mod handlers;
mod structs;

use clap::Parser;
use crate::config::Config;
use crate::config::ShodanCliArgs;
use crate::handlers::host;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: ShodanCliArgs = ShodanCliArgs::parse();
    let api_key:String = Config::get_shodan_api_key();
    let cfg:Config = Config::new(api_key, args.targets.clone());

    host::fetch_data_for_targets(cfg).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_targets() {
        let api_key = "my_api_key".to_string();
        let targets = vec!["127.0.0.1".to_string(), "0.0.0.0".to_string()];
        let config = Config::new(api_key, targets);
        let result = host::fetch_data_for_targets(config).await;

        assert!(result.is_ok());
    }
}