use std::net::{SocketAddr, IpAddr};
use std::sync::Arc;


use clap::{Parser};
use dotenv::dotenv;
use envconfig::Envconfig;
use quotes::database::seeder::Seeder;
use quotes::services::quote::repository::QuoteRepository;
use quotes::services::user::repository::UserRepository;
use quotes::{config, error, db};
use quotes::app;


#[derive(Parser)]
struct Args {
    pub command: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), quotes::error::Error> {
    tracing_subscriber::fmt().init();
    dotenv().ok();
    
    let config = config::Config::init_from_env().map_err(|err| {
        tracing::error!("Failed to load environment variables {:?}",err);
        error::Error::Internal("Failed to load environment variables".into())
    })?;    

    let ip: IpAddr = config.app_host;
    let port: u16 = config.app_port;
    let host = SocketAddr::new(ip, port);
    

    // Run DB
    let db = Arc::new(db::DB::init().await?);

    // Parse arguments
    let args = Args::parse();
    if let Some(command) = args.command {
        if command == "db:seed" {
            let seeder = Seeder::new(
                UserRepository { db: db.clone() },
                QuoteRepository { db: db.clone() }
            );
            seeder.seed().await?;
            return Ok(())
        }
    } 

    app::serve(host, config, db).await.unwrap();

    Ok(())
        
}
