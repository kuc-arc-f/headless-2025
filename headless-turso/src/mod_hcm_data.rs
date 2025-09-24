use anyhow::{ensure, Context, Result};
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
use libsql::Database;
use libsql::Builder;
use libsql::Connection;
use libsql::params;
use serde::{Deserialize, Serialize};
use serde_json::json;
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
    data: String,
    created_at: String,
    updated_at: String,
}


#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    content: String,
    data: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    content: String,
    id: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    id: i64,
    content: String,
    data: String,
}
#[derive(Deserialize, Debug)]
pub struct SearchParams {
    content: Option<String>,
    order: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct GetoneParams {
    content: Option<String>,
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
//State(state): State<super::AppState>,
pub async fn create_data(
    headers: HeaderMap,
    Json(payload): Json<CreateTodo>,
) -> anyhow::Result<Json<Item>, StatusCode> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let valid = valid_authkey(headers, &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");
    println!("TURSO_DATABASE_URL={}", url);
    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();    

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let content_str: &str = &payload.content;
    let sql = format!("INSERT INTO {} (data) VALUES ('{}')", content_str, &payload.data);
    tracing::info!(sql);
    let mut result = conn
        .execute(&sql, ())
        .await
        .unwrap();
        //  params![&payload.data]

    let todo = Item {
        id: 0,
        data: payload.data,
        content: payload.content,
        created_at: "".to_string(),
        updated_at: "".to_string(),        
    };

    Ok(Json(todo))
}


/**
*
* @param
*
* @return
*/
pub async fn getone_data(
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
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");
    println!("TURSO_DATABASE_URL={}", url);
    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();    

    let content_str: &str = params.content.as_deref().unwrap_or("");
    let id_str: &str = params.id.as_deref().unwrap_or("");
    let sql = format!("SELECT id, data ,created_at, updated_at 
    FROM {}
    WHERE id= {} ;
    "
    , content_str , id_str
   );
    tracing::info!("sql={}", &sql);
    let mut rows = conn.query(&sql,
        (),  // 引数なし
    ).await.unwrap();    

    let mut todos: Vec<Item> = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        let id: i64 = row.get(0).unwrap();
        let content: String = content_str.to_string();
        let data: String = row.get(1).unwrap();
        println!("{}: {} {}", id, content, data);
        todos.push(Item {
            id: id,
            content: content,
            data: data,
            created_at: row.get(2).unwrap(),
            updated_at: row.get(3).unwrap(),        
        });        
    }         
    Ok(Json(todos))
}

pub async fn list_data(
    Query(params): Query<SearchParams>,
    headers: HeaderMap
) -> anyhow::Result<Json<Vec<Item>>, StatusCode> {
    let order = format!("#order={:?}", params.order);
    tracing::info!("order={}", order);

    let api_key = env::var("API_KEY")
      .expect("API_KEY must be set");

    let valid = valid_authkey(headers , &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");
    println!("TURSO_DATABASE_URL={}", url);
    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();    

    let mut get_order = &params.order;
    let mut order_sql = "ORDER BY created_at ASC";

    let order_str: &str = params.order.as_deref().unwrap_or("asc");
    let content_str: &str = params.content.as_deref().unwrap_or("asc");

    if order_str != "asc".to_string() {
        order_sql = "ORDER BY created_at DESC";
    }
    tracing::info!("order_sql={}", order_sql);

    let sql = format!("SELECT id, data ,created_at, updated_at 
    FROM {}
    {}
    "
    , content_str , order_sql
   );
    tracing::info!("sql={}", &sql);
    let mut rows = conn.query(&sql,
        (),  // 引数なし
    ).await.unwrap();

    let mut todos: Vec<Item> = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        let id: i64 = row.get(0).unwrap();
        let content: String = content_str.to_string();
        let data: String = row.get(1).unwrap();
        println!("{}: {} {}", id, content, data);
        todos.push(Item {
            id: id,
            content: content,
            data: data,
            created_at: row.get(2).unwrap(),
            updated_at: row.get(3).unwrap(),        
        });        
    }        
    //tracing::info!("todo {:?}", todos);

    Ok(Json(todos))
}

pub async fn delete_data(
    headers: HeaderMap,
    Json(payload): Json<DeleteTodo>,
) -> anyhow::Result<Json<serde_json::Value>, StatusCode> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let valid = valid_authkey(headers, &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");
    println!("TURSO_DATABASE_URL={}", url);
    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();

    let sql = format!("DELETE FROM {} WHERE id = {}", &payload.content, &payload.id);
    tracing::info!(sql);
    let mut result = conn
        .execute(&sql, ())
        .await
        .unwrap();

    Ok(Json(json!({
        "message": "deleted successfully",
        "id": payload.id
    })))
}


pub async fn update_data(
    headers: HeaderMap,
    Json(payload): Json<UpdateTodo>,
) -> anyhow::Result<Json<Item>, StatusCode> {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    let valid = valid_authkey(headers, &api_key);
    if valid == false {
        tracing::info!("NG , authkey");
        return Err(StatusCode::BAD_REQUEST);
    }
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");
    println!("TURSO_DATABASE_URL={}", url);
    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let sql = format!("UPDATE {} SET data = '{}' WHERE id = {}", &payload.content, &payload.data, &payload.id);
    tracing::info!(sql);

    let mut result = conn
        .execute(&sql, ())
        .await
        .unwrap();    

    let todo = Item {
        id: payload.id,
        content: payload.content,
        data: payload.data,
        created_at: "".to_string(),
        updated_at: "".to_string(),
    };

    Ok(Json(todo))
}
