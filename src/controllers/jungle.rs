use axum::http::StatusCode;
use axum::response::Html;

pub async fn get_status() -> (StatusCode, Html<String>) {
    let response = ureq::get("ai.private.dev.cs2024.one").call();

    if let Ok(data) = response {
        if let Ok(str_data) = data.into_string() {
            return (StatusCode::OK, Html(format!("<h1>Received: {str_data}<h1/>", )));
        }
    }

    (StatusCode::INTERNAL_SERVER_ERROR, Html("<h1>Call failed<h1/>".parse().unwrap()))
}
