use std::net::IpAddr;

use envconfig::Envconfig;
#[derive(envconfig::Envconfig)]
pub struct Config {
    #[envconfig(from = "APP_HOST")]
    pub app_host: IpAddr,
    #[envconfig(from = "APP_PORT")]
    pub app_port: u16,
    #[envconfig(from = "APP_ENV")]
    pub app_env: String,
    #[envconfig(from = "MONGO_INITDB_ROOT_USERNAME")]
    pub mongo_initdb_root_username: String,
    #[envconfig(from = "MONGO_INITDB_ROOT_PASSWORD")]
    pub mongo_initdb_root_password: String,
    #[envconfig(from = "MONGO_INITDB_DATABASE")]
    pub mongo_initdb_database: String,
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,
}