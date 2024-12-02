use axum::{
    Router,
    extract::Path,
    response::{Html, Response, IntoResponse},
    routing::get,
    http::{StatusCode, header},
};

use tokio_postgres::{NoTls, Error};

use std::fs;

use rust_server::user::User;

use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

use time::Duration;

struct RouterFabric {}

impl RouterFabric {
    fn new(session_layer: SessionManagerLayer<MemoryStore>) -> Router {
        Router::new()
        .route("/", get(root))
        .route("/get_plugins", get(get_plugins))
        .route("/*file", get(root_file))
        .fallback(not_found)
        .layer(session_layer)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect("postgres://pguser:234234@postgres/lms", NoTls).await?;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let surname =  String::from("Глухов");
    let name = String::from("Александр");
    let patronymic = Some(String::from("Владимирович"));
    let email = String::from("prib-222_897175@volru.ru");
    let password = String::from("5743");
    let group_id = 1;

    //let message = User::register(&client, name, surname, patronymic, email, password, group_id).await;

    //println!("{:?}", message);

    User::login(&client, &email, &password).await;

    let app = RouterFabric::new(session_layer);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn root() -> Html<String> {
    Html(fs::read_to_string("/files/blocks/index.html").unwrap())
}

async fn get_plugins() -> Response {
    let pathes = fs::read_dir("/files/blocks").expect("Can't read the path");
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
        Err(_) => not_found().await.into_response()
    }
}

async fn not_found() -> Html<String> {
    Html(fs::read_to_string("/files/blocks/404.html").unwrap())
}