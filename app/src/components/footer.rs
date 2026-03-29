use leptos::prelude::*;
use crate::server::{get_todos, ClearCompleted};

#[component]
pub fn Footer(filter: RwSignal<String>, refresh: Trigger) -> impl IntoView {
    let todos = Resource::new(
        move || refresh.track(),
        |_| async move { get_todos().await.unwrap_or_default() }
    );
    
    let clear_action = ServerAction::<ClearCompleted>::new();
    Effect::new(move |_| {
        if clear_action.version().get() > 0 {
            refresh.notify();
        }
    });

    view! {
        <Suspense>
            {move || {
                let all_todos = todos.get().unwrap_or_default();
                let active_count = all_todos.iter().filter(|t| !t.completed).count();
                let completed_count = all_todos.iter().filter(|t| t.completed).count();
                
                if all_todos.is_empty() {
                    return view! { <></> }.into_any();
                }
                
                view! {
                    <footer class="footer">
                        <span class="todo-count">
                            <strong>{active_count}</strong>
                            {if active_count == 1 { " item left" } else { " items left" }}
                        </span>
                        <ul class="filters">
                            <li>
                                <a
                                    href="/"
                                    class:selected=move || filter.get() == "all"
                                    on:click=move |_| filter.set("all".to_string())
                                >"All"</a>
                            </li>
                            <li>
                                <a
                                    href="/active"
                                    class:selected=move || filter.get() == "active"
                                    on:click=move |_| filter.set("active".to_string())
                                >"Active"</a>
                            </li>
                            <li>
                                <a
                                    href="/completed"
                                    class:selected=move || filter.get() == "completed"
                                    on:click=move |_| filter.set("completed".to_string())
                                >"Completed"</a>
                            </li>
                        </ul>
                        <Show when=move || (completed_count > 0)>
                            <button
                                class="clear-completed"
                                on:click=move |_| {
                                    clear_action.dispatch(ClearCompleted {});
                                }
                            >
                                "Clear completed"
                            </button>
                        </Show>
                    </footer>
                }.into_any()
            }}
        </Suspense>
    }
}
