use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::Row;
use sqlx::PgPool;

use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct StatusResponse {
    pub task_id: i32,
    pub file_name: String,
    pub status: String,
}

#[derive(ToSchema)]
pub struct ErrorResponse {
    pub message: String,
}

#[utoipa::path(
    get,
    path = "/check/{task_id}",
    params(
        ("task_id" = i32, Path, description = "Task ID to check")
    ),
    responses(
        (status = 200, description = "Status found", body = StatusResponse),
        (status = 404, description = "Task not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "file-service"
)]

pub async fn check_status(
    Path(task_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    match sqlx::query(
        "SELECT file_name, status FROM compression_tasks WHERE id = $1"
    )
    .bind(task_id)
    .fetch_one(&pool)
    .await {
        Ok(record) => { 
            let file_name: String = record.get("file_name");
            let status: String = record.get("status");
            return (StatusCode::OK,
            format!(
                "Task {}: {} (status: {})",
                task_id, file_name, status
            ));
        },
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            format!("Task with ID {} not found", task_id),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        ),
    }
}