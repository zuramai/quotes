use sqlx::{Postgres, postgres::PgPoolOptions, Pool};

use crate::error::Error;


pub struct DB {
    pub conn: Pool<Postgres>
}

impl DB {
    pub async fn init() -> Result<Self, Error> {
        let database_url = std::env::var("DATABASE_URL").ok().unwrap();

        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await {
                Ok(pool) => {
                    tracing::info!("Database connected!");
                    pool
                },
                Err(e) => {
                    println!("Error connecting to database: {:?}", e);
                    std::process::exit(1);
                }
            };
        
        Ok(DB {
            conn: pool
        })
    }
}