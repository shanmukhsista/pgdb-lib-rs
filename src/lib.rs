extern crate core;

use std::fmt::{Debug, Display, Formatter};
use sqlx::{PgPool, Pool, Postgres};
use thiserror::Error;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Error, Debug)]
enum DatabaseConnectionError {
    #[error("Invalid Connection String ")]
    InvalidConnectionString(dotenvy::Error),
    #[error("Unable to connect to the database. ")]
    ConnectionError(#[from] sqlx::Error),
}



struct Database {
    underlying: Pool<Postgres>,
}

impl Database {
    pub async fn new() -> Result<Self, DatabaseConnectionError> {
        let database_url = dotenvy::var("DATABASE_URL").map_err(DatabaseConnectionError::InvalidConnectionString)?;
        let pool = PgPool::connect(&database_url).await.map_err(DatabaseConnectionError::ConnectionError)?;
        Ok(Database {
            underlying: pool
        })
    }
    // Pool internally uses a Clone, so this is not an expensive operation.
    pub fn get_pool(&self) -> &Pool<Postgres>{
        &self.underlying
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_database_connection_get() {
        tokio_test::block_on(async {
            let db = Database::new().await.expect("Database connection expected");
            let row: (i64,) = sqlx::query_as("SELECT $1")
                .bind(150_i64)
                .fetch_one(db.get_pool()).await.expect("error occured ");

            assert_eq!(row.0, 150);
        });
    }
}
