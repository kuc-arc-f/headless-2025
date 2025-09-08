use axum::{
    extract::State,
    extract::Query,
    http::HeaderMap,
    http::StatusCode,
    response::{Json, Html, IntoResponse},
    routing::{get, post},
    Router,
};

//use chrono::Utc;
use chrono::{DateTime, Utc};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::env;
use std::sync::Arc;
use tokio;
use tower_http::services::ServeDir;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize , Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}
//#[derive(Debug, Serialize, Deserialize, FromRow)]
#[derive(Debug, Serialize , FromRow)]
pub struct Item {
    id: i32,
    content: String,
    data: Option<String>,
    created_at: Option<String>, 
    updated_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SearchParams {
    content: Option<String>,
    order: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    content: String,
    data: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    id: i32,
    content: String,
    data: String,
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

pub async fn hcm_content_list(
    State(state): State<super::AppState>,
    headers: HeaderMap
) -> Result<String, StatusCode> {
    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    //println!("valid={}", valid);
    if valid == false {
        println!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }

    // 5) 構造体へマッピングして一覧取得
    let todoItems: Vec<Todo> = sqlx::query_as::<_, Todo>("SELECT distinct content
    FROM hcm_data;")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //println!("Mapped structs > {:?}", todoItems);

    let out = serde_json::to_string(&todoItems).unwrap();
    Ok(out.to_string())
}

pub async fn hcm_data_list(
    State(state): State<super::AppState>,
    Query(params): Query<SearchParams>,
    headers: HeaderMap
) -> Result<String, StatusCode> {
    //let order = format!("#order={:?}", params.order);
    //println!("order={}", order);
    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    if valid == false {
        println!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut get_order = &params.order;
    let mut order_sql = "ORDER BY created_at ASC";

    let order_str: &str = params.order.as_deref().unwrap_or("asc");
    let content_str: &str = params.content.as_deref().unwrap_or("asc");

    if order_str != "asc".to_string() {
        order_sql = "ORDER BY created_at DESC";
    }
    let sql = format!("SELECT id, content, data ,
    to_char(created_at, 'YYYY-MM-DD\"T\"HH24:MI:SS.US\"Z\"') AS created_at ,
    to_char(updated_at, 'YYYY-MM-DD\"T\"HH24:MI:SS.US\"Z\"') AS updated_at
    
    FROM hcm_data
    WHERE content = '{}'
    {}
    "
    , content_str , order_sql
    );
    println!("sql={}", sql);
    // 5) 構造体へマッピングして一覧取得
    let rows = sqlx::query(&sql)
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let todoItems: Vec<Item> = rows
        .into_iter()
        .map(|row| Item {
            id: row.get("id"),
            content: row.get("content"),
            data: row.get("data"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();
    let out = serde_json::to_string(&todoItems).unwrap();
    Ok(out.to_string())
}


pub async fn hcm_data_create(
    State(state): State<super::AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    if valid == false {
        println!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = sqlx::query(
        "INSERT INTO hcm_data (content, data) VALUES ($1, $2) RETURNING id",
    )
    .bind(&payload.content)
    .bind(&payload.data)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //Ok("OK".to_string())
    Ok(Json(json!({
        "ret": 200,
        "message": "Todo created successfully",
    })))
}


pub async fn hcm_data_delete(
    State(state): State<super::AppState>,
    headers: HeaderMap,
    Json(payload): Json<DeleteTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    if valid == false {
        println!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }

    println!("# /api/delete");
    println!("{:?}", payload);
    println!("id={}", &payload.id);
    let sql = format!("DELETE FROM hcm_data WHERE id = {}" , &payload.id);
    println!("sql={}", &sql);

    let result = sqlx::query(&sql)
        //.bind(&payload.id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({
        "message": "Todo deleted successfully",
        "id": payload.id
    })))
}


pub async fn hcm_data_update(
    State(state): State<super::AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    if valid == false {
        println!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }

    println!("# /api/update");
    println!("{:?}", payload);

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let sql = format!("UPDATE hcm_data 
    SET content = '{}', data = '{}' WHERE id = {}"
    ,&payload.content, &payload.data , &payload.id);
    println!("sql={}", &sql);
    
    let result = sqlx::query(&sql)
    //.bind(&payload.content)
    //.bind(&payload.data)
    //.bind(&payload.id)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("# /api/update END");

    Ok(Json(json!({
        "message": "Todo update successfully",
        "id": payload.id
    })))
}
