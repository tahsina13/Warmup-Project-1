use axum::{
    body::Body, 
    extract::{FromRequest, Request}, 
    response::Html, 
    routing::{get, post}, 
    Form
};
use serde::Deserialize;
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};

#[derive(Debug, Clone, Deserialize, TryFromMultipart)]
struct GameForm{
    name: String,
    #[serde(default)]
    #[form_data(default)]
    board: String,
}

pub fn new_connect_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(get_form_handler))
        .route("/", post(post_form_handler))
}

async fn get_form_handler() -> Html<String> {
    Html(ui_components::connect::get_form_html())
}

async fn post_form_handler(req: Request<Body>) -> Html<String> {
    if let Some(content_type) = req.headers().get("content-type") {
        let content_type = content_type.to_str().unwrap();
        if content_type.contains("application/x-www-form-urlencoded") {
            let Form(form) = Form::<GameForm>::from_request(req, &()).await.unwrap();
            Html(ui_components::connect::accept_from_html(form.name, form.board))
        } else if content_type.contains("multipart/form-data") {
            let TypedMultipart(form) = TypedMultipart::<GameForm>::from_request(req, &()).await.unwrap();
            Html(ui_components::connect::accept_from_html(form.name, form.board))
        } else {
            Html("Unsupported content type".to_string())
        }
    } else {
        Html("No content type".to_string())
    }
}

