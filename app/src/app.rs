use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

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
    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
            </header>
        </section>
    }
}
