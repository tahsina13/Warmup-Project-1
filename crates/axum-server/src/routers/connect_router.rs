use axum::{response::Html, routing::{get, post}, Form};
use serde::Deserialize;

#[derive(Deserialize)]
struct StartGameForm {
    name: String,
}

pub fn new_connect_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(get_form_handler)) 
        .route("/", post(post_form_handler))
}

async fn get_form_handler() -> Html<String> {
    Html( ui_components::connect::get_form_html() )
}

async fn post_form_handler(
    Form(form): Form<StartGameForm>
) -> Html<String> {
    Html( ui_components::connect::accept_from_html(form.name) )
}