use axum::http::StatusCode;
use axum::response::Html;
use serde::Deserialize;

#[derive(Deserialize)]
struct Step {
    name: String,
    status: String
}

pub async fn get_status() -> (StatusCode, Html<String>) {
    let response = ureq::get("http://ai.private.dev.cs2024.one/jungle").call();

    return match response {
        Ok(data) => {
            if let Ok(str_data) = data
                .into_string()
                .map_err(|_| ())
                .and_then(|e| serde_json::from_str::<Vec<Step>>(e.as_str()).map_err(|_| ())) {

                let page = str_data
                    .iter()
                    .map(|step| format!("Name: {name}, Result: {status} <br>", name=step.name, status=step.status))
                    .collect::<Vec<_>>()
                    .join("");

                return (StatusCode::OK, Html(format!("<h1>Received:</h1>{page}")));
            }
            (StatusCode::INTERNAL_SERVER_ERROR, Html("<h1>Call failed!!</h1>".parse().unwrap()))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Html(format!("<h1>Call failed with reason: </h1><p>{err}</p>")))
        }
    }
}
