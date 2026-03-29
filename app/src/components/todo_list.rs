use leptos::prelude::*;
use crate::model::Todo;
use crate::server::{get_todos, ToggleAll};
use crate::components::todo_item::TodoItem;

#[component]
pub fn TodoList(filter: Signal<String>) -> impl IntoView {
    let refresh = Trigger::new();
    
    let todos = Resource::new(
        move || refresh.track(),
        |_| async move { get_todos().await.unwrap_or_default() }
    );
    
    let toggle_all_action = ServerAction::<ToggleAll>::new();
    Effect::new(move |_| {
        if toggle_all_action.version().get() > 0 {
            refresh.notify();
        }
    });

    view! {
        <Suspense fallback=|| view! { <p class="loading">"Loading..."</p> }>
            {move || {
                let all_todos = todos.get().unwrap_or_default();
                let filtered: Vec<Todo> = match filter.get().as_str() {
                    "active" => all_todos.iter().filter(|t| !t.completed).cloned().collect(),
                    "completed" => all_todos.iter().filter(|t| t.completed).cloned().collect(),
                    _ => all_todos.clone(),
                };
                let all_completed = !all_todos.is_empty() && all_todos.iter().all(|t| t.completed);
                
                view! {
                    <section class="main">
                        <input
                            id="toggle-all"
                            class="toggle-all"
                            type="checkbox"
                            prop:checked=all_completed
                            on:change=move |_| {
                                toggle_all_action.dispatch(ToggleAll { completed: !all_completed });
                            }
                        />
                        <label for="toggle-all">"Mark all as complete"</label>
                        <ul class="todo-list">
                            <For
                                each=move || filtered.clone()
                                key=|todo| todo.id
                                children=move |todo| {
                                    view! { <TodoItem todo=todo refresh=refresh/> }
                                }
                            />
                        </ul>
                    </section>
                }
            }}
        </Suspense>
    }
}
