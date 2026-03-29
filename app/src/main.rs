#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use app::App;

    let conf = leptos::config::get_configuration(None).expect("Failed to get leptos configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(feature = "ssr")]
fn shell(_options: leptos::config::LeptosOptions) -> impl leptos::IntoView {
    use app::App;
    leptos::view! { <App/> }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
