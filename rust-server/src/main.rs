use axum::{
    Router,
    extract::Path,
    response::{Html, Response, IntoResponse},
    routing::get,
    http::{StatusCode, header}
};

use std::fs;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/get_plugins", get(get_plugins))
        .route("/*file", get(root_file))
        .fallback(not_found);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> Html<String> {
    Html(fs::read_to_string("/files/blocks/index.html").unwrap())
}

async fn get_plugins() -> Response {
    let pathes = fs::read_dir("/files/blocks").expect("Can't read path!");
    let mut plugins = String::from("[");
    for path in pathes {
        let element = path
            .unwrap()
            .path()
            .to_str()
            .unwrap()
            .split("/")
            .last()
            .unwrap()
            .to_owned();
        if element.starts_with("plugin") {
            plugins = format!("{} \"{}\",", plugins, element);
        }
    }
    let mut chars = plugins.chars();
    chars.next_back();
    let content = format!("{} ]", chars.as_str());
    (StatusCode::OK,
    [(header::CONTENT_TYPE, "application/json")],
    content
    ).into_response()
}

async fn root_file(Path(file): Path<String>) -> Response {
    let file_path = format!("/files/blocks/{file}");
    match fs::read(&file_path) {
        Ok(content) => {
            let mime_type = mime_guess::from_path(&file_path).first_or_text_plain();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime_type.as_ref())],
                content,
            ).into_response()
        }
        Err(_) => not_found().await,
    }
}

async fn not_found() -> Response {
    fs::read_to_string("/files/blocks/404.html").unwrap().into_response()
}