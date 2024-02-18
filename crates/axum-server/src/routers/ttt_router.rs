use axum::{extract::Query, response::Html, routing::get};
use serde::Deserialize;

#[derive(Deserialize)]
struct StartGameForm {
    name: Option<String>,
    board: Option<String>,
}

pub fn new_ttt_router() -> axum::Router {
    axum::Router::new().route("/", get(get_handler))
}

async fn get_handler(Query(query): Query<StartGameForm>) -> Html<String> {
    match query.name {
        Some(name) => {
            let board = query.board.unwrap_or_default();
            Html(ui_components::ttt::accept_from_html(name, board))
        }
        None => Html(ui_components::ttt::get_form_html()),
    }
}
