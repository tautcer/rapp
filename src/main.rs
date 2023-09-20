use std::sync::{Arc, Mutex};

use crate::handlers::*;
use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;

pub struct AppState {
    todos: Mutex<Vec<String>>,
    nav_items: Mutex<NavItems>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "with_axum_htmx_askama=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_level(true) // don't include levels in formatted output
                .with_target(true) // don't include targets
                .with_thread_ids(false) // include the thread ID of the current thread
                .with_thread_names(true)
                .compact(),
        ) // include the name of the current thread
        .init();
    info!("Initialising router....");

    let app_state = Arc::new(AppState {
        todos: Mutex::new(vec![]),
        nav_items: Mutex::new(NavItems::default()),
    });

    let assets_path = std::env::current_dir().unwrap();

    let api_router = Router::new()
        .route("/hello", get(hello_from_the_server))
        .route("/todos", post(add_todo))
        .route("/todos", get(get_todos))
        .route("/nav_items", get(navbar_items))
        .with_state(app_state);

    let router = Router::new()
        .nest("/api", api_router)
        .route("/", get(hello))
        .route("/another-page", get(another_page))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );

    let port = 8000_u16;

    info!("Router initialised, now listening on port {}", port);

    axum::Server::bind(&format!("127.0.0.0:{}", port).as_str().parse().unwrap())
        .serve(router.into_make_service())
        .await
        .context("error while start server")?;

    Ok(())
}

async fn hello_from_the_server() -> &'static str {
    "hello!"
}
