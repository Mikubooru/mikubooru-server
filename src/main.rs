use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::signal::unix::signal;
use tokio::task::AbortHandle;
use tower_sessions::{ExpiredDeletion, Expiry, Session, SessionManager, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

use crate::db::types::{NewUser, UserRole, UserVocaDBId};
use crate::db::user::add_user;

mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .connect("./development.sqlite")
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let session_store = SqliteStore::new(pool.clone());
    session_store.migrate().await?;

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );

    let session_layer =
        SessionManagerLayer::new(session_store).with_secure(!cfg!(debug_assertions));

    let app = Router::new()
        .route("/login", get(login))
        .route("/", get(|| async { "Hello, World!" }))
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await?;

    Ok(())
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handle")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort()},
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
