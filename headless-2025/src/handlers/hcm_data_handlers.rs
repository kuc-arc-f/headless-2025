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

fn valid_authkey(req: &Request, sendkey: &str) -> Result<bool> {
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

pub async fn handle_list_content(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {    
    let db = ctx.env.d1("DB")?;
    let sendkey = ctx.env.var("API_KEY")?.to_string();
    let valid = valid_authkey(&req, &sendkey);
    match valid {
        Ok(false) => {
            console_log!("NG , auth");
            return Response::error("NG , auth", 401);
        },
        Ok(true) => {
            console_log!("OK , auth");
        },
        Err(e) => {
            console_log!("NG, nothing auth-key: {}", e);
            return Response::error("NG, nothing auth-key", 400);
        },
    }    

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
    let valid = valid_authkey(&req, &sendkey);
    match valid {
        Ok(false) => {
            console_log!("NG , auth");
            return Response::error("NG , auth", 401);
        },
        Ok(true) => {
            console_log!("OK , auth");
        },
        Err(e) => {
            console_log!("NG, nothing auth-key: {}", e);
            return Response::error("NG, nothing auth-key", 400);
        },
    }    

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


pub async fn handle_create_data(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let db = ctx.env.d1("DB")?;
    let sendkey = ctx.env.var("API_KEY")?.to_string();
    let valid = valid_authkey(&req, &sendkey);
    match valid {
        Ok(false) => {
            console_log!("NG , auth");
            return Response::error("NG , auth", 401);
        },
        Ok(true) => {
            console_log!("OK , auth");
        },
        Err(e) => {
            console_log!("NG, nothing auth-key: {}", e);
            return Response::error("NG, nothing auth-key", 400);
        },
    }

    let create_req: CreateItemRequest = req.json().await?;
    console_log!("{:?}", create_req);
    
    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
    
    let query = "INSERT INTO hcm_data (content, data, created_at, updated_at) VALUES (?, ?, ?, ?) RETURNING id, content, data, created_at, updated_at";
    let stmt = db.prepare(query)
        .bind(&[create_req.content.clone().into(), 
               create_req.data.clone().unwrap_or_default().into(),
               now.clone().into(),
               now.clone().into()])?;
    
    let result = stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(row) => {
            let todo = Item {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                content: row["content"].as_str().unwrap_or_default().to_string(),
                data: row["data"].as_str().map(|s| s.to_string()),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            Response::from_json(&ItemCreateResponse {
                status: 201,
                data: todo,
            })
        },
        None => {
            Response::from_json(&GenericResponse {
                status: 500,
                message: "Failed to create todo".to_string(),
            })
        }
    }
}

pub async fn handle_delete_data(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let sendkey = ctx.env.var("API_KEY")?.to_string();
    let valid = valid_authkey(&req, &sendkey);
    match valid {
        Ok(false) => {
            console_log!("NG , auth");
            return Response::error("NG , auth", 401);
        },
        Ok(true) => {
            console_log!("OK , auth");
        },
        Err(e) => {
            console_log!("NG, nothing auth-key: {}", e);
            return Response::error("NG, nothing auth-key", 400);
        },
    }

    let delete_req: DeleteItemRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;
    
    let query = "DELETE FROM hcm_data WHERE id = ?";
    let stmt = db.prepare(query).bind(&[delete_req.id.into()])?;
    
    let result = stmt.run().await?;
    Response::from_json(&GenericResponse {
        status: 200,
        message: "Todo item successfully".to_string(),
    })
}

pub async fn handle_update_todo(mut req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let sendkey = ctx.env.var("API_KEY")?.to_string();
    let valid = valid_authkey(&req, &sendkey);
    match valid {
        Ok(false) => {
            console_log!("NG , auth");
            return Response::error("NG , auth", 401);
        },
        Ok(true) => {
            console_log!("OK , auth");
        },
        Err(e) => {
            console_log!("NG, nothing auth-key: {}", e);
            return Response::error("NG, nothing auth-key", 400);
        },
    }

    let update_req: UpdateItemRequest = req.json().await?;
    let db = ctx.env.d1("DB")?;
    
    let now = js_sys::Date::new_0().to_iso_string().as_string().unwrap();
    
    // First check if the todo exists
    let check_query = "SELECT id FROM hcm_data WHERE id = ?";
    let check_stmt = db.prepare(check_query).bind(&[update_req.id.into()])?;
    let exists = check_stmt.first::<serde_json::Value>(None).await?;
    
    if exists.is_none() {
        return Response::from_json(&GenericResponse {
            status: 404,
            message: "item not found".to_string(),
        });
    }
    
    // Build dynamic update query
    let mut query_parts = Vec::new();
    let mut bindings = Vec::new();
    
    if let Some(content) = &update_req.content {
        query_parts.push("content = ?");
        bindings.push(content.clone().into());
    }
    
    if let Some(data) = &update_req.data {
        query_parts.push("data = ?");
        bindings.push(data.clone().into());
    }
    
    if query_parts.is_empty() {
        return Response::from_json(&GenericResponse {
            status: 400,
            message: "No fields to update".to_string(),
        });
    }
    
    query_parts.push("updated_at = ?");
    bindings.push(now.into());
    bindings.push(update_req.id.into());
    
    let query = format!("UPDATE hcm_data SET {} WHERE id = ? RETURNING id, content, data, created_at, updated_at", query_parts.join(", "));
    let stmt = db.prepare(&query).bind(&bindings)?;
    
    let result = stmt.first::<serde_json::Value>(None).await?;
    
    match result {
        Some(row) => {
            let todo = Item {
                id: row["id"].as_f64().unwrap_or(0.0) as u32,
                content: row["content"].as_str().unwrap_or_default().to_string(),
                data: row["data"].as_str().map(|s| s.to_string()),
                created_at: row["created_at"].as_str().unwrap_or_default().to_string(),
                updated_at: row["updated_at"].as_str().unwrap_or_default().to_string(),
            };
            
            Response::from_json(&ItemCreateResponse {
                status: 200,
                data: todo,
            })
        },
        None => {
            Response::from_json(&GenericResponse {
                status: 500,
                message: "Failed to update todo".to_string(),
            })
        }
    }
}