use leptos::prelude::*;
use crate::model::Todo;

#[cfg(feature = "ssr")]
pub async fn get_db() -> Result<sqlx::SqlitePool, ServerFnError> {
    use_context::<sqlx::SqlitePool>()
        .ok_or_else(|| ServerFnError::ServerError("No database pool in context".to_string()))
}

#[server(GetTodos, "/api")]
pub async fn get_todos() -> Result<Vec<Todo>, ServerFnError> {
    let pool = get_db().await?;
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, completed, sort_order FROM todos ORDER BY sort_order, id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::ServerError::<server_fn::error::NoCustomError>(e.to_string()))?;
    Ok(todos)
}
