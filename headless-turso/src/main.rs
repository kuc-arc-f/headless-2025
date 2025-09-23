use axum::{
    extract::State,
    http::HeaderMap,
    http::Request,
    http::StatusCode,
    middleware::Next,
    routing::{get, post}, 
    Router,
    response::Redirect,
    response::{Html, IntoResponse, Json},
};
use axum_extra::{
    extract::cookie::{CookieJar, Cookie},
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
use tokio::sync::Mutex;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    db: Arc<Mutex<Database>>,
}
//mod mod_hcm_admin;
mod mod_hcm_data;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: i64,
    title: String,
    content: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    title: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DeleteTodo {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    id: i64,
    title: String,
    content: Option<String>,
}
#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // ログの初期化
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    dotenv().ok();

    // `public` フォルダのパス
    let public_dir = "public/static";

    // `ServeDir` ミドルウェアを初期化
    let serve_dir = ServeDir::new(public_dir);
    
    let app = Router::new()
        .nest_service("/static", serve_dir)
        .route("/api/login", post(handle_login_post))
        /*
        .route("/api/admin/content_list", get(mod_hcm_admin::handle_list_content))
        .route("/api/admin/data_list", get(mod_hcm_admin::list_admin))        
        .route("/api/content/list", get(mod_hcm_data::handle_list_content))
        */
        .route("/api/data/list", get(mod_hcm_data::list_data))
        .route("/api/data/create", post(mod_hcm_data::create_data))
        .route("/api/data/delete", post(mod_hcm_data::delete_data))
        .route("/api/data/update", post(mod_hcm_data::update_data));
        /*
        .route("/api/data/getone", get(mod_hcm_data::getone_data))
        */
        //.route("/", get(root))
        //.route("/login", get(root))
        //.route("/data", get(root))
        //.route("/foo", get(get_foo));

    info!("start server http://localhots:3000"); 

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    let addr = "127.0.0.1:3000";
    tracing::info!("Listening on {}", addr);

}

async fn handle_login_post(
    Json(payload): Json<LoginRequest>
) -> Result<String , StatusCode> {
    tracing::info!("payload={:?}", payload);
    tracing::info!("name={}", payload.username);
    tracing::info!("password={}", payload.password);
    let username = payload.username;
    let password = payload.password;

    let env_username = env::var("USER_NAME").expect("USER_NAME must be set");
    let env_password = env::var("PASSWORD").expect("PASSWORD must be set");
    //Ok("OK".to_string())
    if username == env_username && password == env_password {
        return Ok("OK".to_string())
    }else{
        return Err(StatusCode::BAD_REQUEST);
    }
}

async fn root(
  headers: HeaderMap
) -> Html<&'static str> {
    let login_htm = "<!doctype html>
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
    <script type='module' src='/static/move_login.js'></script>
  <body>
</html>
";
    let has_session_cookie = headers
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .map(|cookies| {
            cookies
                .split(';')
                .any(|cookie| cookie.trim().starts_with("userid="))
        })
        .unwrap_or(false);

    if has_session_cookie == false {
        return Html(&login_htm);
    };

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

async fn get_foo(headers: HeaderMap) -> impl IntoResponse {
    let has_session_cookie = headers
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .map(|cookies| {
            cookies
                .split(';')
                .any(|cookie| cookie.trim().starts_with("userid="))
        })
        .unwrap_or(false);

    if has_session_cookie == false {
        tracing::info!("NG, cookie: userid not found");
        "userid cookie not found".into_response();
        Redirect::to("/login").into_response()
    } else {
        tracing::info!("ok, cookie: userid found, redirecting to /bar");
        "OK , userid cookie".into_response()
    }
}


// リダイレクト用ハンドラ
async fn redirect_handler() -> Redirect {
    Redirect::to("/login")  // 302 Found (デフォルト)
}

fn parse_cookie(cookies: &str, key: &str) -> Option<String> {
    for cookie in cookies.split(';') {
        let mut parts = cookie.trim().splitn(2, '=');
        let k = parts.next()?.trim();
        let v = parts.next()?.trim();
        if k == key {
            return Some(v.to_string());
        }
    }
    None
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

