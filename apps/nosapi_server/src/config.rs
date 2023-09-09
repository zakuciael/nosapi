use std::net::{IpAddr, Ipv4Addr};

use rocket::config::LogLevel;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerConfig {
    port: u16,
    address: IpAddr,
    log_level: LogLevel,
}

impl Default for ServerConfig {
    fn default() -> Self {
        let port = std::env::var("PORT")
            .unwrap_or("8000".to_owned())
            .parse::<u16>()
            .expect("Invalid port specified");

        #[cfg(debug_assertions)]
        {
            ServerConfig {
                port,
                address: IpAddr::V4(Ipv4Addr::LOCALHOST),
                log_level: LogLevel::Debug,
            }
        }
        #[cfg(not(debug_assertions))]
        {
            ServerConfig {
                port,
                address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
                log_level: LogLevel::Normal,
            }
        }
    }
}
