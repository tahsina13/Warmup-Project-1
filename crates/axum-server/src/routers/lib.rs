use axum::{
    body::Body,
    extract::{FromRequest, Request},
    response::Html,
    Form,
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};

pub async fn parse_form<T>(req: Request<Body>) -> Result<T, Html<String>>
where
    T: TryFromMultipart,
    Form<T>: FromRequest<()>,
{
    match req.headers().get("content-type") {
        Some(content_type) => {
            let content_type = content_type
                .to_str()
                .or(Err(Html("Failed to parse content-type".to_owned())))?;
            if content_type.contains("application/x-www-form-urlencoded") {
                match Form::<T>::from_request(req, &()).await {
                    Ok(Form(form)) => Ok(form),
                    Err(_) => Err(Html("Failed to parse form".to_owned())),
                }
            } else if content_type.contains("multipart/form-data") {
                match TypedMultipart::from_request(req, &()).await {
                    Ok(TypedMultipart::<T>(form)) => Ok(form),
                    Err(_) => Err(Html("Failed to parse multipart".to_owned())),
                }
            } else {
                Err(Html("Unsupported content-type".to_owned()))
            }
        }
        None => Err(Html("No content-type".to_owned())),
    }
}
