use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use chrono::{TimeZone, Utc};
use chrono_tz::Asia::Tokyo;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use axum::http::{header, Method};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tower_http::cors::{CorsLayer, AllowOrigin};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/todos/:id", get(find_todo).patch(update_todo));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// root ハンドラ
async fn root() -> &'static str {
    "Hello, World!"
}

// find todoハンドラ
async fn find_todo(Path(id): Path<u64>) -> Response {

    match id {
        404 => {
            println!("--------------------404");
            println!("{}", id);
            (StatusCode::NOT_FOUND,  Json(NotFoundError::new("見つかりませんでした".to_string()))).into_response()
        },
        500 => {
            println!("--------------------500");
            println!("{}", id);
            let mut f = File::open("/not-existed").await.unwrap();
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer).await.unwrap();
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(UnknownError::new("Hello".to_string()))
            ).into_response()
        },
        _ => {
            println!("--------------------200");
            println!("{}", id);
            let todo = TodoResponse::new(id, "Hello".to_string(), false);
            (StatusCode::OK, Json(todo)).into_response()
        }
    }
}

// update todoハンドラ
async fn update_todo(
    Path(id): Path<u64>,
    Json(payload): Json<TodoRequest>,
) -> Response {
    match id {
        404 => (
            StatusCode::NOT_FOUND,
            Json(NotFoundError::new("見つかりませんでした".to_string())),
        ).into_response(),
        500 => {
            panic!("Hello");
        },
        _ => {
            let todo = TodoResponse::new(id, payload.title, payload.is_completed);
            (StatusCode::OK, Json(todo)).into_response()
        },
    }
}

// リクエスト用のTodo
#[derive(Deserialize)]
struct TodoRequest {
    title: String,
    is_completed: bool,
}

// レスポンス用のTodo
#[derive(Serialize)]
struct TodoResponse {
    id: u64,
    title: String,
    is_completed: bool,
    created_at: String,
    updated_at: String,
}
impl TodoResponse {
    fn new(id: u64, title: String, is_completed: bool) -> Self {
        let jst = Tokyo.from_utc_datetime(&Utc::now().naive_utc());
        Self {
            id,
            title,
            is_completed,
            created_at: jst.to_rfc3339(),
            updated_at: jst.to_rfc3339(),
        }
    }
}

// 500エラー
#[derive(Serialize)]
struct UnknownError {
    code: String,
    message: String,
}
impl UnknownError {
    fn new(message: String) -> Self {
        Self {
            code: "500".to_string(),
            message,
        }
    }
}

// 404エラー
#[derive(Serialize)]
struct NotFoundError {
    code: String,
    message: String,
}
impl NotFoundError {
    fn new(message: String) -> Self {
        Self {
            code: "404".to_string(),
            message,
        }
    }
}
