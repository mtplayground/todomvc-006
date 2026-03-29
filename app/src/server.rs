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

#[server(AddTodo, "/api")]
pub async fn add_todo(title: String) -> Result<(), ServerFnError> {
    let pool = get_db().await?;
    let title = title.trim().to_string();
    if title.is_empty() {
        return Err(ServerFnError::ServerError("Title cannot be empty".to_string()));
    }
    let max_order: Option<i64> = sqlx::query_scalar("SELECT MAX(sort_order) FROM todos")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError::<server_fn::error::NoCustomError>(e.to_string()))?;
    let next_order = max_order.unwrap_or(0) + 1;
    sqlx::query("INSERT INTO todos (title, completed, sort_order) VALUES (?, FALSE, ?)")
        .bind(&title)
        .bind(next_order)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError::<server_fn::error::NoCustomError>(e.to_string()))?;
    Ok(())
}

#[server(DeleteTodo, "/api")]
pub async fn delete_todo(id: i64) -> Result<(), ServerFnError> {
    let pool = get_db().await?;
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::ServerError::<server_fn::error::NoCustomError>(e.to_string()))?;
    Ok(())
}
