use axum::{
    body::{Bytes, Body},
    extract::{FromRequest, Request, State},
    response::Html,
    routing::{get, post},
    Form,
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use http_body_util::BodyExt;
use serde::Deserialize;

#[derive(Deserialize, TryFromMultipart)]
struct StartGameForm {
    name: String,
    board: Option<String>,
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
    let (parts, body) = req.into_parts();

    let bytes = body.collect().await.unwrap().to_bytes();
    let body_1 = Body::from(bytes.clone());
    let body_2 = Body::from(bytes.clone());

    let req = Request::from_parts(parts.clone(), body_1);

    let req_2 = Request::from_parts(parts, body_2);

    if let Ok(Form(form)) = Form::<StartGameForm>::from_request(req, &()).await {
        let board = form.board.unwrap_or("".to_owned());
        Html(ui_components::connect::accept_from_html(form.name, board))
    } else if let Ok(TypedMultipart(form)) = TypedMultipart::<StartGameForm>::from_request(req_2, &()).await {
        let board = form.board.unwrap_or("".to_owned());
        Html(ui_components::connect::accept_from_html(form.name, board))
    } else {
        Html("Failed to parse request body".into())
    }
}

