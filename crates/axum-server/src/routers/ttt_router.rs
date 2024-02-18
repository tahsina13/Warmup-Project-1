use axum::{
    response::Html,
    routing::{get},
    extract::{
        Query,
    }
};
use serde::Deserialize;

#[derive(Deserialize)]
struct StartGameForm {
    name: Option<String>,
    board: Option<String>,
}

pub fn new_ttt_router() -> axum::Router {
    axum::Router::new()
        .route("/", get(get_handler))
}

async fn get_handler(Query(query): Query<StartGameForm>) -> Html<String> {
    if query.name.is_some() {
        let board = query.board.unwrap_or_else(|| "".to_string());
        Html(ui_components::ttt::accept_from_html(query.name.unwrap(), board))
    } else {
        Html(ui_components::ttt::get_form_html())
    } 
}
