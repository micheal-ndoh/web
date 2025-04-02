use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::Row;
use sqlx::PgPool;

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