use mongodb::{Collection, bson::Document, options::{ClientOptions, ServerApi, ServerApiVersion}, error::Error, Client, Database};

use crate::quote::model::Quote;


pub struct DB {
    pub conn: Database
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        let database_url = std::env::var("DATABASE_URL").ok().unwrap();
        let mut  client_options = ClientOptions::parse(database_url).await?;

        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);

        let client = Client::with_options(client_options)?;

        let db = client
            .database(std::env::var("MONGO_INITDB_DATABASE").ok().unwrap().as_str());
        
        tracing::info!("MongoDB database connected!");
        Ok(DB {
            conn: db
        })
    }
}