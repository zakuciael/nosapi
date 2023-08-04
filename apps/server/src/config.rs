use std::net::{IpAddr, Ipv4Addr};
use std::num::ParseIntError;

use rocket::config::LogLevel;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ServerConfig {
    port: u16,
    address: IpAddr,
    log_level: LogLevel,
}

impl ServerConfig {
    fn debug_default() -> Self {
        ServerConfig {
            port: ServerConfig::port().expect("Invalid port specified"),
            address: IpAddr::V4(Ipv4Addr::LOCALHOST),
            log_level: LogLevel::Debug,
        }
    }

    fn release_default() -> Self {
        ServerConfig {
            port: ServerConfig::port().expect("Invalid port specified"),
            address: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            log_level: LogLevel::Normal,
        }
    }

    fn port() -> Result<u16, ParseIntError> {
        std::env::var("PORT")
            .unwrap_or("8000".to_string())
            .parse::<u16>()
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        {
            ServerConfig::debug_default()
        }
        #[cfg(not(debug_assertions))]
        {
            ServerConfig::release_default()
        }
    }
}
