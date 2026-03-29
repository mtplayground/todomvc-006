use leptos::prelude::*;
use crate::model::Todo;
use crate::server::{DeleteTodo, UpdateTodo};

#[component]
pub fn TodoItem(todo: Todo, refresh: Trigger) -> impl IntoView {
    let (editing, set_editing) = signal(false);
    let (edit_title, set_edit_title) = signal(todo.title.clone());
    
    let delete_action = ServerAction::<DeleteTodo>::new();
    let update_action = ServerAction::<UpdateTodo>::new();
    
    Effect::new({
        let refresh = refresh;
        move |_| {
            if delete_action.version().get() > 0 || update_action.version().get() > 0 {
                refresh.notify();
            }
        }
    });

    let todo_id = todo.id;
    let todo_completed = todo.completed;
    let todo_title = todo.title.clone();
    let todo_title_display = todo.title.clone();

    view! {
        <li class:completed=todo_completed class:editing=move || editing.get()>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=todo_completed
                    on:change=move |_| {
                        update_action.dispatch(UpdateTodo {
                            id: todo_id,
                            title: edit_title.get_untracked(),
                            completed: !todo_completed,
                        });
                    }
                />
                <label on:dblclick=move |_| {
                    set_editing.set(true);
                    set_edit_title.set(todo_title.clone());
                }>
                    {todo_title_display}
                </label>
                <button
                    class="destroy"
                    on:click=move |_| {
                        delete_action.dispatch(DeleteTodo { id: todo_id });
                    }
                />
            </div>
            <Show when=move || editing.get()>
                <input
                    class="edit"
                    prop:value=move || edit_title.get()
                    on:input=move |ev| set_edit_title.set(event_target_value(&ev))
                    on:keydown=move |ev| {
                        if ev.key() == "Enter" {
                            let t = edit_title.get_untracked();
                            if t.trim().is_empty() {
                                delete_action.dispatch(DeleteTodo { id: todo_id });
                            } else {
                                update_action.dispatch(UpdateTodo {
                                    id: todo_id,
                                    title: t,
                                    completed: todo_completed,
                                });
                            }
                            set_editing.set(false);
                        } else if ev.key() == "Escape" {
                            set_editing.set(false);
                        }
                    }
                    on:blur=move |_| {
                        if editing.get_untracked() {
                            let t = edit_title.get_untracked();
                            if t.trim().is_empty() {
                                delete_action.dispatch(DeleteTodo { id: todo_id });
                            } else {
                                update_action.dispatch(UpdateTodo {
                                    id: todo_id,
                                    title: t,
                                    completed: todo_completed,
                                });
                            }
                            set_editing.set(false);
                        }
                    }
                />
            </Show>
        </li>
    }
}
