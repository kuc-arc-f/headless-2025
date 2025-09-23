use axum::{
    extract::State,
    extract::Query,
    http::HeaderMap,
    http::StatusCode,
    routing::{get, post}, 
    Router,
    response::{Html, IntoResponse, Json},
};
use chrono::Utc;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{sqlite::SqlitePool, Row};
use std::env;
use std::sync::Arc;
use tracing::{info, warn};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: i64,
    title: String,
    content: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    id: i64,
    content: String,
    data: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    content: String,
    data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    id: i64,
    content: Option<String>,
    data: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct SearchParams {
    content: Option<String>,
}

fn valid_authkey(headers: HeaderMap, sendkey: &str) -> bool {
    if let Some(auth) = headers.get("Authorization") {
        let value = auth.to_str().unwrap_or("");
        tracing::info!("auth={}", value);
        if value == sendkey {
            tracing::info!("ok, auth-key");
            return true;
        } else {
            tracing::info!("NG, auth-key");
            return false;
        }        
    } else {
        return false;
    }
}

#[tracing::instrument]
pub async fn handle_list_content(
    State(pool): State<Arc<SqlitePool>>, headers: HeaderMap
) -> Result<Json<Vec<String>>, StatusCode> {

    let rows = sqlx::query("SELECT distinct content 
    FROM hcm_data;
    ")
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut todos: Vec<String> = Vec::new();
    for row in rows {
        let mut content: String =  row.get("content");
        todos.push(content.to_string());
    }

    Ok(Json(todos))
}

#[tracing::instrument]
pub async fn list_admin(
    State(pool): State<Arc<SqlitePool>>,
    Query(params): Query<SearchParams>,
    headers: HeaderMap
) -> Result<Json<Vec<Item>>, StatusCode> {
    let content = format!("#content={:?}", params.content);
    //tracing::info!("{}", content);

    let rows = sqlx::query("SELECT id, content, data ,created_at, updated_at 
    FROM hcm_data
    WHERE content = ? ORDER BY created_at ASC
    ")
        .bind(&params.content)
        .fetch_all(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos: Vec<Item> = rows
        .into_iter()
        .map(|row| Item {
            id: row.get("id"),
            content: row.get("content"),
            data: row.get("data"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();
    //tracing::info!("todo {:?}", todos);

    Ok(Json(todos))
}

