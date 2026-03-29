use leptos::prelude::*;
use crate::model::Todo;

#[component]
pub fn TodoItem(todo: Todo, refresh: Trigger) -> impl IntoView {
    let _ = refresh;
    view! {
        <li>{todo.title}</li>
    }
}
