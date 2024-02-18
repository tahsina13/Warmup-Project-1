use axum::{body::Body, extract::Request, response::Html, routing::get};
use axum_typed_multipart::TryFromMultipart;
use serde::Deserialize;

use crate::lib::parse_form;
use ui_components::connect;

#[derive(Debug, Clone, Deserialize, TryFromMultipart)]
struct GameForm {
    name: String,
    #[serde(default)]
    #[form_data(default)]
    board: String,
}

pub fn new_connect_router() -> axum::Router {
    axum::Router::new().route("/", get(get_form_handler).post(post_form_handler))
}

async fn get_form_handler() -> Html<String> {
    Html(connect::get_form_html())
}

async fn post_form_handler(req: Request<Body>) -> Html<String> {
    match parse_form::<GameForm>(req).await {
        Ok(form) => Html(connect::accept_from_html(form.name, form.board)),
        Err(error_page) => error_page,
    }
}
