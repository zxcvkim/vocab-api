use std::{env, net::SocketAddr};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let app_port = env::var("APP_PORT").unwrap_or_else(|_| "7222".to_string());
        let addr: SocketAddr = format!("{app_host}:{app_port}")
            .parse()
            .context("APP_HOST or APP_PORT is invalid")?;

        Ok(Self { addr })
    }
}
