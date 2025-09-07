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
    order: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct GetoneParams {
    id: Option<String>,
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
/**
*
* @param
*
* @return
*/
#[tracing::instrument]
pub async fn handle_list_content(
    State(pool): State<Arc<SqlitePool>>, headers: HeaderMap
) -> Result<Json<Vec<String>>, StatusCode> {
    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    //tracing::info!("valid={}", valid);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }

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
    //tracing::info!("todo {:?}", todos);

    Ok(Json(todos))
}

/**
*
* @param
*
* @return
*/
pub async fn getone_data(
    State(pool): State<Arc<SqlitePool>>,
    Query(params): Query<GetoneParams>,
    headers: HeaderMap
) -> Result<Json<Vec<Item>>, StatusCode> {
    let id = format!("#id={:?}", params.id);
    tracing::info!("id={}", id);

    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let rows = sqlx::query("SELECT * FROM hcm_data WHERE id= ? ;
    ")
    .bind(&params.id)
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
    Ok(Json(todos))
}

/**
*
* @param
*
* @return
*/
#[tracing::instrument]
pub async fn list_data(
    State(pool): State<Arc<SqlitePool>>,
    Query(params): Query<SearchParams>,
    headers: HeaderMap
) -> Result<Json<Vec<Item>>, StatusCode> {
    let order = format!("#order={:?}", params.order);
    tracing::info!("order={}", order);

    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let mut get_order = &params.order;
    let mut order_sql = "ORDER BY created_at ASC";

    let order_str: &str = params.order.as_deref().unwrap_or("asc");
    let content_str: &str = params.content.as_deref().unwrap_or("asc");

    if order_str != "asc".to_string() {
        order_sql = "ORDER BY created_at DESC";
    }
    tracing::info!("order_sql={}", order_sql);

    let sql = format!("SELECT id, content, data ,created_at, updated_at 
    FROM hcm_data
    WHERE content = '{}'
    {}
    "
    , content_str , order_sql
   );
    tracing::info!("sql={}", &sql);
    //.bind(&params.content)
    let rows = sqlx::query(&sql)
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

/**
*
* @param
*
* @return
*/
pub async fn create_data(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Item>, StatusCode> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let valid = valid_authkey(headers, &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let result =
        sqlx::query("INSERT INTO hcm_data (content, data , created_at, updated_at) VALUES (?, ?, ?, ?)")
            .bind(&payload.content)
            .bind(&payload.data)
            .bind(&now)
            .bind(&now)
            .execute(pool.as_ref())
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo_id = result.last_insert_rowid();

    let todo = Item {
        id: todo_id,
        data: payload.data,
        content: payload.content,
        created_at: Some(now.clone()),
        updated_at: Some(now),
    };

    Ok(Json(todo))
}

/**
*
* @param
*
* @return
*/
pub async fn delete_data(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<DeleteTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let valid = valid_authkey(headers, &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let result = sqlx::query("DELETE FROM hcm_data WHERE id = ?")
        .bind(payload.id)
        .execute(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(json!({
        "message": "deleted successfully",
        "id": payload.id
    })))
}

pub async fn update_data(
    State(pool): State<Arc<SqlitePool>>,
    headers: HeaderMap,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Item>, StatusCode> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let valid = valid_authkey(headers, &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let result = sqlx::query(
        "UPDATE hcm_data SET data = ?, content = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&payload.data)
    .bind(&payload.content)
    .bind(&now)
    .bind(payload.id)
    .execute(pool.as_ref())
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let row = sqlx::query("SELECT id, data, content, created_at, updated_at FROM hcm_data WHERE id = ?")
        .bind(payload.id)
        .fetch_one(pool.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo = Item {
        id: row.get("id"),
        content: row.get("content"),
        data: row.get("data"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    };

    Ok(Json(todo))
}
