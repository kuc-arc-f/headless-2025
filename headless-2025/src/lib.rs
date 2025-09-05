use serde::{Deserialize, Serialize};
use worker::*;
use base64::{engine::general_purpose, Engine as _};

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct UserRequest {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Todo {
    id: u32,
    title: String,
    description: Option<String>,
    completed: bool,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateTodoRequest {
    title: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteTodoRequest {
    id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct UpdateTodoRequest {
    id: u32,
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoListResponse {
    status: u16,
    data: Vec<Todo>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoCreateResponse {
    status: u16,
    data: Todo,
}
mod handlers; // handlers/mod.rs を指す


#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    //.get_async("/foo", handle_get)
    Router::new()
        .get_async("/api/admin/content_list", handlers::hcm_admin_handlers::handle_list_content)
        .get_async("/api/admin/data_list", handlers::hcm_admin_handlers::handle_list_data)

        .get_async("/api/content/list", handlers::hcm_data_handlers::handle_list_content)
        .get_async("/api/data/list", handlers::hcm_data_handlers::handle_list_data)
        .post_async("/api/data/create", handlers::hcm_data_handlers::handle_create_data)
        .post_async("/api/data/delete", handlers::hcm_data_handlers::handle_delete_data)
        .post_async("/api/data/update", handlers::hcm_data_handlers::handle_update_todo)
        .post_async("/api/login", handle_login_post)

        .get_async("/", handle_get) 
        .get_async("/data", handle_get) 
        .get_async("/login", handle_get)
        .get_async("/bar", handle_bar)
        .delete_async("/baz", handle_delete)
        .run(req, env)
        .await
}

pub async fn handle_get(mut req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let mut url = req.url()?;
    let path = url.path();
    if let Some(cookie_header) = req.headers().get("Cookie")? {
        let userid = parse_cookie(&cookie_header, "userid");
        if let Some(uid) = userid {
            console_log!("userid: {}", &uid);
        }else{
          console_log!("NG,  userid");
          console_log!("url: {}", url);
          console_log!("path: {}", path);
          if path != "/login" {
            url.set_path("/login");
            return Response::redirect(url);
          }
        }
    }    
    let cookie_header = req.headers().get("Cookie")?;
    if cookie_header.is_none() {
        if path != "/login" {
            url.set_path("/login");
            return Response::redirect(url);
        }
    }
    // HTML文字列を生成  
    let html = format!(
      "<!DOCTYPE html>
      <html>
        <head>
          <meta charset=\"utf-8\">
          <meta name='viewport' content='width=device-width, initial-scale=1.0' />
          <title>My Page</title>
          <script src='https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4'></script>
        </head>
        <body>
          <div id='app'></div>
          <script type='module' src='/client.js'></script>
        </body>
      </html>"
    );
    // Response::from_html を使うとヘッダー付きで便利
    let mut response = Response::from_html(html)?;
    // 必要に応じてヘッダーをカスタム可能
    response.headers_mut()
        .set("Content-Type", "text/html; charset=utf-8")?;
    Ok(response)
}

pub fn get_cookie(req: &Request) -> Option<User> {
    let cookie_header = req.headers().get("Cookie").ok().flatten()?;
    for cookie in cookie_header.split(';') {
        let cookie = cookie.trim();
        if cookie.starts_with("userid=") {
            let encoded = cookie.trim_start_matches("userid=");
            let decoded = general_purpose::STANDARD.decode(encoded).ok()?;
            let user: User = serde_json::from_slice(&decoded).ok()?;
            return Some(user);
        }
    }
    None
}
pub async fn handle_login_post(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    //let login_req: UserRequest = req.json().await?;
    //console_log!("{:?}", login_req);
    let env_username = ctx.env.var("USER_NAME")?.to_string();
    let env_password = ctx.env.var("PASSWORD")?.to_string();

    let body: serde_json::Value = req.json().await.map_err(|_| {
        Response::error("Invalid JSON", 400).unwrap_err()
    })?;

    let username = body["username"].as_str().unwrap_or_default();
    let password = body["password"].as_str().unwrap_or_default();
    if username == env_username && password == env_password {
        Response::from_json(&GenericResponse {
            status: 200,
            message: "OK".to_string(),
        })
    }else{
        Response::from_json(&GenericResponse {
            status: 400,
            message: "NG".to_string(),
        })
    }
}
pub async fn handle_bar(mut req: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    let sendkey = _ctx.env.var("API_KEY")?.to_string();
    //console_log!("sendkey={}", &sendkey);
    let valid = valid_authkey(req, &sendkey);
    console_log!("valid={:?}", valid);
    match valid {
        Ok(false) => {
            console_log!("NG , auth");
            return Response::error("NG , auth", 401);
        },
        Ok(true) => {
            console_log!("OK , auth");
            return Response::ok("OK , auth");
        },
        Err(e) => {
            console_log!("NG, nothing auth-key: {}", e);
            return Response::error("NG, nothing auth-key", 400);
        },
    }    

    Response::from_json(&GenericResponse {
        status: 200,
        message: "bar!".to_string(),
    })
}

fn valid_authkey(mut req: Request, sendkey: &str) -> Result<bool> {
    if let Some(api_key) = req.headers().get("Authorization")? {
        console_log!("API Key:{}", api_key);

        // APIキーを使った判定例
        if api_key == sendkey {
            console_log!("ok, auth-key");
            return Ok(true);
        } else {
            console_log!("NG, auth-key");
            return Ok(false);
        }
    } else {
        console_log!("NG, nothing auth-key");
        return Ok(false);
    }
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

pub async fn handle_post(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a POST route!".to_string(),
    })
}

pub async fn handle_delete(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a DELETE route!".to_string(),
    })
}
