use leptos::prelude::*;

#[component]
pub fn Footer(filter: RwSignal<String>, refresh: Trigger) -> impl IntoView {
    let _ = filter;
    let _ = refresh;
    view! {
        <footer class="footer"></footer>
    }
}
