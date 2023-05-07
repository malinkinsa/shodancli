use reqwest;
use reqwest::Response;
use std::fmt::{Display, Formatter};

use crate::config::Config;
use crate::structs::host;

impl Display for host::ShodanHostData {
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

async fn fetch_host_data(cfg:Config, target: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url: String = format!(
        "https://api.shodan.io/shodan/host/{}?key={}", target.trim(), cfg.api_key
    );
    let response: Response = reqwest::get(url).await?;

    if response.status() == reqwest::StatusCode::OK {
        let body: host::ShodanHostData = response.json::<host::ShodanHostData>().await?;
        println!("{}", body);
    } else {
        println!("There are no entries for: {}", target);
    }

    Ok(())
}

pub async fn fetch_data_for_targets(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = cfg.targets.iter().map(|target| {
        fetch_host_data(cfg.clone(), target)
    });

    futures::future::join_all(tasks).await;
    Ok(())
}