use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path, hooks::use_params_map};
use crate::components::{
    header::Header,
    todo_list::TodoList,
    footer::Footer,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="stylesheet" href="https://unpkg.com/todomvc-app-css@2.4.2/index.css"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Title text="TodoMVC - Leptos"/>
        <Router>
            <Routes fallback=|| view! { <p>"Not found"</p> }>
                <Route path=path!("/") view=TodoPage/>
                <Route path=path!("/:filter") view=TodoPage/>
            </Routes>
        </Router>
    }
}

#[component]
fn TodoPage() -> impl IntoView {
    let params = use_params_map();
    let filter = RwSignal::new("all".to_string());
    
    Effect::new(move |_| {
        let f = params.get().get("filter").map(|s| s.to_string()).unwrap_or_default();
        if f == "active" || f == "completed" {
            filter.set(f);
        } else {
            filter.set("all".to_string());
        }
    });
    
    let refresh = Trigger::new();
    let filter_signal = Signal::derive(move || filter.get());

    view! {
        <section class="todoapp">
            <Header refresh=refresh/>
            <TodoList filter=filter_signal/>
            <Footer filter=filter refresh=refresh/>
        </section>
    }
}
