use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, Html, IntoResponse},
    routing::{get, post},
    Router,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use std::sync::Arc;
use tokio;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}
/*
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}
*/

#[derive(Debug, Serialize , Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
pub struct DeleteTodo {
    id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    id: i32,
    title: String,
    content: Option<String>,
}
pub async fn get_todos(State(state): State<super::AppState>) -> Result<String, StatusCode> {

    // 5) 構造体へマッピングして一覧取得
    let todoItems: Vec<Todo> = sqlx::query_as::<_, Todo>("SELECT id, title, content FROM todos ORDER BY id DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    //println!("Mapped structs > {:?}", todoItems);

    let out = serde_json::to_string(&todoItems).unwrap();
    Ok(out.to_string())
}


pub async fn create_todo(
    State(state): State<super::AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query(
        "INSERT INTO todos (title, content) VALUES ($1, $2) RETURNING id",
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //Ok("OK".to_string())
    Ok(Json(json!({
        "ret": 200,
        "message": "Todo created successfully",
    })))
}

pub async fn delete_todo(
    State(state): State<super::AppState>,
    Json(payload): Json<DeleteTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/delete");
    println!("{:?}", payload);

    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(&payload.id)
        .execute(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    //println!("# /api/delete END");
    Ok(Json(json!({
        "message": "Todo deleted successfully",
        "id": payload.id
    })))
}


pub async fn update_todo(
    State(state): State<super::AppState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    println!("# /api/update");
    println!("{:?}", payload);

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    let result = sqlx::query(
        "UPDATE todos SET title = $1, content = $2 WHERE id = $3"
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.id)
    .execute(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("# /api/update END");

    //Ok(Json(todo))
    Ok(Json(json!({
        "message": "Todo update successfully",
        "id": payload.id
    })))
}