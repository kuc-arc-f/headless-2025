use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Html, IntoResponse},
    routing::{get, post},
    Router,
};

use chrono::Utc;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::env;
use std::sync::Arc;
use tokio;
use tower_http::services::ServeDir;

mod handlers;
mod mod_hcm_data;
use handlers::*;
use mod_hcm_data::*;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}


async fn root() -> Html<&'static str> {
    let s1 = "<!doctype html>
<html>
  <head>
    <meta charset='UTF-8' />
    <meta name='viewport' content='width=device-width, initial-scale=1.0' />
    <title>welcome</title>
    <script src='https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4'></script>
  </head>
  <body>
    <div id='app'></div>
    <script type='module' src='/static/client.js'></script>
  <body>
</html>
";
  Html(&s1)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    // `public` フォルダのパス
    let public_dir = "public/static";

    // `ServeDir` ミドルウェアを初期化
    let serve_dir = ServeDir::new(public_dir);

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:admin@localhost/postgres")
    .await.expect("Failed to create pool");

    let state = AppState { pool };

    let app = Router::new()
        .nest_service("/static", serve_dir)
        .route("/api/list", get(get_todos))
        .route("/api/create", post(create_todo))
        .route("/api/delete", post(delete_todo))
        .route("/api/update", post(update_todo))  

        .route("/api/content/list", get(hcm_content_list))
        .route("/api/data/list", get(hcm_data_list))
        .route("/api/data/create", post(hcm_data_create))
        .route("/api/data/delete", post(hcm_data_delete))
        .route("/api/data/update", post(hcm_data_update))

        .route("/", get(root))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
