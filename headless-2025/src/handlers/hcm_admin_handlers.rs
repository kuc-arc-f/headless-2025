use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    id: u32,
    content: String,
    data: Option<String>,
    created_at: String,
    updated_at: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Content {
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct CreateItemRequest {
    content: String,
    data: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DeleteItemRequest {
    id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct UpdateItemRequest {
    id: u32,
    content: Option<String>,
    data: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TodoListResponse {
    status: u16,
    data: Vec<Item>,
}
#[derive(Debug, Deserialize, Serialize)]
struct ContentListResponse {
    status: u16,
    data: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ItemCreateResponse {
    status: u16,
    data: Item,
}

pub async fn handle_list_content(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {    
    let db = ctx.env.d1("DB")?;

    let query = "SELECT distinct content FROM hcm_data;";
    console_log!("query: {}", query);

    let stmt = db.prepare(query);
    let result = stmt.all().await?;
    
    let mut todos: Vec<String> = Vec::new();
    if let Ok(results) = result.results::<serde_json::Value>() {
        for row in results {
            let mut content =  row["content"].as_str().unwrap_or_default().to_string();
            todos.push(content.to_string());
        }
    }
    Response::from_json(&ContentListResponse {
        status: 200,
        data: todos,
    })
}

pub async fn handle_list_data(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;
    let sendkey = ctx.env.var("API_KEY")?.to_string();

    // URL 全体を取得
    let url = req.url()?;
    
    // Query string 全体
    let query = url.query().unwrap_or("");
    console_log!("query string: {}", query);

    // 特定のキーを取得
    let params: Vec<(_, _)> = url.query_pairs().collect();
    let contentTmp = params.iter().find(|(k, _)| k == "content").map(|(_, v)| v.to_string());
    let content = contentTmp.unwrap_or("".into());
    console_log!("content: {}", content);
    let sqlWhere = format!(" WHERE content = '{}'" ,  content);
    console_log!("sqlWhere: {}", sqlWhere);

    let query = format!("SELECT id, content, data, created_at, updated_at 
    FROM hcm_data {}
    ORDER BY created_at DESC", sqlWhere);
    console_log!("query: {}", query);

    let stmt = db.prepare(query);
    let result = stmt.all().await?;
    
    let mut todos = Vec::new();
    
    if let Ok(results) = result.results::<serde_json::Value>() {
        for row in results {
            let todo = Item {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                content: row["content"].as_str().unwrap_or_default().to_string(),
                data: row["data"].as_str().map(|s| s.to_string()),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            todos.push(todo);
        }
    }
    
    Response::from_json(&TodoListResponse {
        status: 200,
        data: todos,
    })
}

