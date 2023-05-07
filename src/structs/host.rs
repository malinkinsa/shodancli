use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShodanHostData {
    pub asn: String,
    pub city: String,
    pub country_code: String,
    pub country_name: String,
    pub data: Vec<ShodanPortData>,
    ip: i64,
    pub ip_str: String,
    pub isp: String,
    pub last_update: String,
    latitude: f64,
    longitude: f64,
    pub org: String,
    pub os: Value,
    pub ports: Vec<i64>,
    pub region_code: String,
    pub tags: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShodanPortData {
    #[serde(rename = "_shodan")]
    shodan: Shodan,
    cpe: Option<Vec<String>>,
    cpe23: Option<Vec<String>>,
    data: Option<String>,
    domains: Option<Vec<String>>,
    hash: i64,
    hostname: Option<Vec<String>>,
    info: Option<String>,
    pub product: Option<String>,
    pub port: i64,
    pub transport: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shodan {
    crawler: String,
    id: String,
    module: String,
    ptr: Option<bool>,
    region: String,
}