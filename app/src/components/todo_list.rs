use leptos::prelude::*;

#[component]
pub fn TodoList(filter: Signal<String>) -> impl IntoView {
    let _ = filter;
    view! {
        <section class="main"></section>
    }
}
