use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use serde_json::json;
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
struct TaskStatus {
    id: i64,
    file_name: String,
    status: String,
}

/// Endpoint to check the status of a compression task
pub async fn get_task_status(
    Path(task_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, TaskStatus>(
        "SELECT id, file_name, status FROM compression_tasks WHERE id = $1",
    )
    .bind(task_id)
    .fetch_one(&pool)
    .await
    {
        Ok(task) => (
            StatusCode::OK,
            Json(json!({
                "id": task.id,
                "file_name": task.file_name,
                "status": task.status
            })),
        ),
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Task not found"})),
        ),
        Err(e) => {
            eprintln!("Database error: {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Internal server error"})),
            )
        }
    }
}
