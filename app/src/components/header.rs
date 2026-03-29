use leptos::prelude::*;
use crate::server::AddTodo;

#[component]
pub fn Header(refresh: Trigger) -> impl IntoView {
    let add_todo_action = ServerAction::<AddTodo>::new();
    
    Effect::new(move |_| {
        if add_todo_action.version().get() > 0 {
            refresh.notify();
        }
    });

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <ActionForm action=add_todo_action>
                <input
                    class="new-todo"
                    name="title"
                    placeholder="What needs to be done?"
                    autofocus=true
                />
            </ActionForm>
        </header>
    }
}
