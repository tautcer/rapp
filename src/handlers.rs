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
    let lock = state.nav_items.lock().unwrap();
    let items = lock.clone();
    let template = NavItems { items: items.items };
    HtmlTemplate("navbar", template)
}

#[derive(Clone)]
struct NavItem {
    href: String,
    name: String,
}

#[derive(Template, Clone)]
#[template(path = "navbar.html")]
pub struct NavItems {
    items: Vec<NavItem>,
}

impl Default for NavItems {
    fn default() -> Self {
        NavItems {
            items: vec![
                NavItem {
                    href: String::from("/"),
                    name: String::from("Home"),
                },
                NavItem {
                    href: String::from("/another-page"),
                    name: String::from("Another page"),
                },
                NavItem {
                    href: String::from("/about"),
                    name: String::from("About"),
                },
                NavItem {
                    href: String::from("/contact"),
                    name: String::from("Contact"),
                },
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
    let mut lock = state.todos.lock().unwrap();
    lock.push(todo.todo);

    let template = TodoList {
        todos: lock.clone(),
    };

    HtmlTemplate("add-todo", template)
}

pub async fn get_todos(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lock = state.todos.lock().unwrap();

    let template = TodoList {
        todos: lock.clone(),
    };

    HtmlTemplate("todos", template)
}

#[derive(Template)]
#[template(path = "another-page.html")]
struct AnotherPageTemplate;

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate;

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
