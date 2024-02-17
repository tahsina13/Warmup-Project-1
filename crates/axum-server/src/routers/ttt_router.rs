use axum::{
    response::Html,
    routing::{get, post},
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct StartGameForm {
    name: Option<String>,
    board: Option<String>,
}

pub fn new_ttt_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(get_form_handler))
        .route("/", post(post_form_handler))
}

async fn get_form_handler() -> Html<String> {
    Html(ui_components::ttt::get_form_html())
}

async fn post_form_handler(Form(form): Form<StartGameForm>) -> Html<String> {
    if form.name.is_some() {
        let board = form.board.unwrap_or_else(|| "".to_string());
        Html(ui_components::ttt::accept_from_html(
            form.name.unwrap(),
            board,
        ))
    } else {
        Html(ui_components::ttt::get_form_html())
    }
}
