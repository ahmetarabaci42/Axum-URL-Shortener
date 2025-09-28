use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    db: Pool<Sqlite>,
}

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    code: String,
    short_url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ✅ SQLite connection string doğru hale getirildi
    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://shortener.db")
        .await?;

    // ✅ tablo yoksa oluştur
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id TEXT PRIMARY KEY,
            url TEXT NOT NULL
        )",
    )
    .execute(&db)
    .await?;

    let state = AppState { db };

    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/:code", get(resolve))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Running on http://{addr}");
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;
    Ok(())
}

async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, axum::http::StatusCode> {
    let code = Uuid::new_v4().to_string()[..8].to_string();

    sqlx::query("INSERT INTO urls (id, url) VALUES (?, ?)")
        .bind(&code)
        .bind(&payload.url)
        .execute(&state.db)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ShortenResponse {
        code: code.clone(),
        short_url: format!("http://localhost:3000/{code}"),
    }))
}

async fn resolve(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, axum::http::StatusCode> {
    let row = sqlx::query_as::<_, (String,)>("SELECT url FROM urls WHERE id = ?")
        .bind(code)
        .fetch_one(&state.db)
        .await
        .map_err(|_| axum::http::StatusCode::NOT_FOUND)?;

    Ok(Redirect::to(&row.0))
}
