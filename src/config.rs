use envconfig::Envconfig;
#[derive(envconfig::Envconfig)]
pub struct Config {
    #[envconfig(from = "APP_HOST")]
    app_host: String,
    #[envconfig(from = "APP_PORT")]
    app_port: String,
    #[envconfig(from = "APP_ENV")]
    app_env: String,
    #[envconfig(from = "MONGO_INITDB_ROOT_USERNAME")]
    mongo_initdb_root_username: String,
    #[envconfig(from = "MONGO_INITDB_ROOT_PASSWORD")]
    mongo_initdb_root_password: String,
    #[envconfig(from = "MONGO_INITDB_DATABASE")]
    mongo_initdb_database: String,
    #[envconfig(from = "DATABASE_URL")]
    database_url: String,
}