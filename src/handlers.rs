use std::sync::Arc;

use askama::Template;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Form,
};
use serde::Deserialize;
use tracing::debug;

use crate::AppState;

pub async fn hello() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate("hello", template)
}

pub async fn another_page() -> impl IntoResponse {
    let template = AnotherPageTemplate {};
    HtmlTemplate("another-page", template)
}

pub async fn navbar_items(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    debug!("rendering navbar");
    let _ = state.page_data.lock().unwrap().nav_items.items;
    let lock = state.page_data.lock().unwrap();
    let items = lock.clone();
    let template = NavItems { items: items.nav_items.items };
    // render!("navbar", template, state.page_data)
    HtmlTemplate("navbar", template)
}

#[derive(Template, Clone)]
#[template(path = "navbar.html")]
pub struct NavItems {
    items: Vec<(String, String)>,
}

impl Default for NavItems {
    fn default() -> Self {
        NavItems {
            items: vec![
                (
                    String::from("/"),
                     String::from("Home"),
                ),
                (
                    String::from("/another-page"),
                    String::from("Another page"),
                ),
                (
                    String::from("/about"),
                    String::from("About"),
                ),
                (
                    String::from("/contact"),
                    String::from("Contact"),
                ),
            ],
        }
    }
}

#[derive(Template)]
#[template(path = "todo-list.html")]
struct TodoList {
    todos: Vec<String>,
}

#[derive(Deserialize)]
pub struct TodoRequest {
    todo: String,
}

pub async fn add_todo(
    State(state): State<Arc<AppState>>,
    Form(todo): Form<TodoRequest>,
) -> impl IntoResponse {
    debug!("add-todo called");
    let mut lock = state.page_data.lock().unwrap();
    lock.todos.push(todo.todo);

    let template = TodoList {
        todos: lock.todos.clone(),
    };

    HtmlTemplate("add-todo", template)
}

pub async fn get_todos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lock = state.page_data.lock().unwrap();

    let template = TodoList {
        todos: lock.todos.clone(),
    };

    HtmlTemplate("todos", template)
}

#[derive(Template)]
#[template(path = "another-page.html")]
struct AnotherPageTemplate;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate;

pub async fn global_not_found() -> impl IntoResponse {
    let template = ErrorTemplate {};
    HtmlTemplate("error", template)
}

struct HtmlTemplate<'a, T>(&'a str, T);

impl<T> IntoResponse for HtmlTemplate<'_, T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.1.render() {
            Ok(html) => {
                debug!("rendered: {}", self.0);
                Html(html).into_response()
            },
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error {}", err),
            )
                .into_response(),
        }
    }
}
