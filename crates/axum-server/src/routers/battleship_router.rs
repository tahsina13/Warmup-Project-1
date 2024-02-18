use axum::{response::Html, routing::get};
use serde::Deserialize;
use tower_sessions::Session;

use axum::{body::Body, extract::Request};
use axum_typed_multipart::TryFromMultipart;

use ui_components::battleship::*;

use crate::lib::parse_form;

#[derive(Debug, Clone, Deserialize, TryFromMultipart)]
struct GameForm {
    name: Option<String>,
    r#move: Option<String>,
    play_again: Option<String>,
}

pub fn new_battleship_router() -> axum::Router {
    axum::Router::new().route("/", get(get_form_handler).post(post_form_handler))
}

async fn get_form_handler() -> Html<&'static str> {
    Html(BATTLESHIP_GET_PAGE)
}

const NAME_KEY: &str = "name";
const MOVES_KEY: &str = "moves_left";
const BOARD_KEY: &str = "board";

async fn post_form_handler(session: Session, req: Request<Body>) -> Html<String> {
    let form: GameForm = match parse_form(req).await {
        Ok(form) => form,
        Err(err_page) => return err_page,
    };

    // process name
    if let Some(name) = form.name {
        session.insert(NAME_KEY, name).await.unwrap();
    } else if form.play_again.is_some() {
        // must reset move and board before loading it for rendering
        let _: Option<i32> = session.remove(MOVES_KEY).await.unwrap();
        let _: Option<Vec<Vec<Tile>>> = session.remove(BOARD_KEY).await.unwrap();
    }

    let mut moves_left = session
        .get(MOVES_KEY)
        .await
        .unwrap()
        .unwrap_or(((COLS as f64) * (ROWS as f64) * 0.60).ceil() as i32);

    let mut board = session
        .get(BOARD_KEY)
        .await
        .unwrap()
        .unwrap_or(create_battleship_game(ROWS, COLS, &SHIPS));

    // make move
    if let Some(move_str) = form.r#move {
        let mut it = move_str.split(',');
        let (i, j): (usize, usize) = (
            it.next().unwrap().parse().unwrap(),
            it.next().unwrap().parse().unwrap(),
        );
        match board[i][j] {
            Ship => board[i][j] = Hit,
            Untried => board[i][j] = Miss,
            _ => {}
        }
        moves_left -= 1;
    }

    // update session
    session.insert(MOVES_KEY, moves_left).await.unwrap();
    session.insert(BOARD_KEY, &board).await.unwrap();

    let name = session
        .get(NAME_KEY)
        .await
        .unwrap()
        .unwrap_or("".to_owned());

    Html(make_board_page(name, board, moves_left))
}
