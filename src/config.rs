use std::{env, net::SocketAddr};

use anyhow::Context;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: SocketAddr,
    pub vocab_path: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let app_host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let app_port = env::var("APP_PORT").unwrap_or_else(|_| "7222".to_string());
        let addr: SocketAddr = format!("{app_host}:{app_port}")
            .parse()
            .context("APP_HOST or APP_PORT is invalid")?;

        let vocab_path = env::var("VOCAB_PATH").unwrap_or_else(|_| "/data/words.txt".to_string());

        Ok(Self { addr, vocab_path })
    }
}
