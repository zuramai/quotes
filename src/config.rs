use std::net::IpAddr;

use envconfig::Envconfig;
#[derive(envconfig::Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "APP_HOST")]
    pub app_host: IpAddr,
    #[envconfig(from = "APP_PORT")]
    pub app_port: u16,
    #[envconfig(from = "APP_ENV")]
    pub app_env: String,
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
}