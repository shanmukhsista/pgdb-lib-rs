# Readme

This is a simple Rust Wrapper over SQLX and Postgres. 

To connect to Postgres, use the following 

```rust
DATABASE_URL=postgres://postgres:postgres@localhost:5432/fenvoxdb?currentSchema=app
async fn main(){
    let db = Database::new().await.expect("Database connection Failed");
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(db.get_pool()).await.expect("error occured ");
}
```