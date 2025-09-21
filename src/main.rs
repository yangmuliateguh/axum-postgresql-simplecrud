mod db;
use axum::{
    routing::get,
    Router, Json, extract::{Path, State}
};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::net::SocketAddr;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: Uuid,
    title: String,
    completed: bool,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
struct CreateTodo {
    title: String,
}

#[derive(Deserialize, Debug)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

async fn get_todos(State(pool): State<PgPool>) -> Result<Json<Vec<Todo>>, (axum::http::StatusCode, String)> {
    let todos = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed, created_at FROM todos",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(todos))
}

async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, (axum::http::StatusCode, String)> {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed, created_at",
        payload.title
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(todo))
}

async fn get_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Todo>, (axum::http::StatusCode, String)> {
    let todo = sqlx::query_as!(
        Todo,
        "SELECT id, title, completed, created_at FROM todos WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((axum::http::StatusCode::NOT_FOUND, "Todo not found".to_string()))?;

    Ok(Json(todo))
}

async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTodo>
) -> Result<Json<Todo>, (axum::http::StatusCode, String)> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET title = COALESCE($1, title), completed = COALESCE($2, completed)
        WHERE id = $3
        RETURNING id, title, completed, created_at
        "#,
        payload.title,
        payload.completed,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((axum::http::StatusCode::NOT_FOUND, "Todo not found".to_string()))?;

    Ok(Json(todo))
}

async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>
) -> Result<Json<()>, (axum::http::StatusCode, String)> {
    let result = sqlx::query!(
        "DELETE FROM todos WHERE id = $1",
        id
    )
    .execute(&pool)
    .await
    .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((axum::http::StatusCode::NOT_FOUND, "Todo not found".to_string()));
    }

    Ok(Json(()))
}

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    env_logger::init();

    let pool = db::connect_db().await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(hello))
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", get(get_todo).put(update_todo).delete(delete_todo))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}