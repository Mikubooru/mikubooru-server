use axum::Router;
use axum::routing::get;
use color_eyre::Result;
use sqlx::sqlite::SqlitePoolOptions;

mod db;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePoolOptions::new()
        .connect("./development.sqlite")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res: db::types::User = sqlx::query_as("SELECT * FROM user")
        .fetch_one(&pool)
        .await?;

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
