use axum::{
    extract::{Form, Path, State},
    http::{header, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use tokio_postgres::{Error, NoTls};

use std::{fs, sync::Arc};

use rust_server::group::Group;
use rust_server::news::News;
use rust_server::user::User;

use tower_sessions::{Expiry, MemoryStore, Session, SessionManagerLayer};

use serde::Deserialize;
use time::Duration;

use plugin_system::{
    PluginSystem,
    AppState,
};

struct RouterFabric {}

impl RouterFabric {
    fn new(session_layer: SessionManagerLayer<MemoryStore>, state: Arc<AppState>) -> Router {
        Router::new()
            .route("/", get(root))
            .route("/get_windows", get(get_windows))
            .route("/get_groups", get(get_groups))
            .route("/get_user", get(get_user))
            .route("/get_news", get(get_news))
            .route("/login", post(login))
            .route("/register", post(register))
            .route("/logout", post(logout))
            .route("/*file", get(root_file))
            .fallback(not_found)
            .layer(session_layer)
            .with_state(state)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("postgres://pguser:234234@postgres/lms", NoTls).await?;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::minutes(30)));

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let state = Arc::new(AppState { client });

    let app = RouterFabric::new(session_layer, state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn root() -> Html<String> {
    Html(fs::read_to_string("/files/blocks/index.html").unwrap())
}

async fn get_plugins() -> Response {
    let paths = fs::read_dir("/files/blocks").expect("Can't read the path");
    let mut plugins = String::from("[");
    for path in paths {
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
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json")],
        content,
    )
        .into_response()
}

async fn get_windows(session: Session) -> Response {
    if let Some(_) = session.get::<i32>("user_id").await.unwrap() {
        get_plugins().await
    } else {
        Json(vec!["login", "registration"]).into_response()
    }
}

async fn root_file(Path(file): Path<String>, State(state): State<Arc<AppState>>) -> Response {
    let file_path = format!("/files/blocks/{file}");
    if file.ends_with(".lua") {

        let script = fs::read_to_string(&file_path).unwrap();
        PluginSystem::execute(state, &script);

        return Html("Скрипт успешно выполнен").into_response();
    }
    match fs::read(&file_path) {
        Ok(content) => {
            let mime_type = mime_guess::from_path(&file_path).first_or_text_plain();
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, mime_type.as_ref())],
                content,
            )
                .into_response()
        }
        Err(_) => not_found().await,
    }
}

async fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        [(header::CONTENT_TYPE, "text/html")],
        fs::read_to_string("/files/blocks/404.html").unwrap(),
    )
        .into_response()
}

async fn get_user(State(state): State<Arc<AppState>>, session: Session) -> Response {
    let user_id: i32 = session.get("user_id").await.unwrap().unwrap_or(1);
    let user = User::get_by_id(&state.client, user_id).await;
    if let Some(user) = user {
        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            format!(
                "{} {} {}",
                user.get::<&str, String>("surname"),
                user.get::<&str, String>("name"),
                user.get::<&str, String>("patronymic")
            ),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            String::from("Пользователь не найден"),
        )
    }
    .into_response()
}

async fn get_groups(State(state): State<Arc<AppState>>) -> Response {
    Json(
        &Group::get_all(&state.client)
            .await
            .iter()
            .map(|row| Group {
                id: row.get::<_, i32>("id"),
                name: row.get::<_, String>("name"),
            })
            .collect::<Vec<_>>(),
    )
    .into_response()
}

#[derive(Deserialize)]
struct RegistrationParams {
    name: String,
    surname: String,
    patronymic: String,
    email: String,
    password: String,
    password2: String,
    group_id: i32,
}

async fn register(
    State(state): State<Arc<AppState>>,
    Form(params): Form<RegistrationParams>,
) -> Response {
    let patronymic = if !params.patronymic.is_empty() {
        Some(&*params.patronymic)
    } else {
        None
    };

    if params.password != params.password2 {
        return (
            StatusCode::UNAUTHORIZED,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            "Пароли не совпадают",
        )
            .into_response();
    }

    if !User::register(
        &state.client,
        &params.name,
        &params.surname,
        patronymic,
        &params.email,
        &params.password,
        params.group_id,
    )
    .await
    .is_ok()
    {
        return (
            StatusCode::UNAUTHORIZED,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            "Ошибка при регистрации",
        )
            .into_response();
    }

    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        format!("Пользователь {} успешно зарегистрирован", &params.email),
    )
        .into_response()
}

#[derive(Deserialize)]
struct LoginParams {
    email: String,
    password: String,
}

async fn login(
    State(state): State<Arc<AppState>>,
    session: Session,
    Form(params): Form<LoginParams>,
) -> Response {
    if User::approve_login(&state.client, &params.email, &params.password).await {
        session
            .insert(
                "user_id",
                User::get_by_email(&state.client, &params.email)
                    .await
                    .unwrap()
                    .get::<_, i32>("id"),
            )
            .await
            .unwrap();
        return (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
            format!("Пользователь {} успешно авторизован", &params.email),
        )
            .into_response();
    };
    println!("WTF");
    (
        StatusCode::UNAUTHORIZED,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        "Проиозшла ошибка при попытке авторизации",
    )
        .into_response()
}

async fn get_news(State(state): State<Arc<AppState>>, session: Session) -> Response {
    let user_id: i32 = session.get("user_id").await.unwrap().unwrap_or(1);
    let records = News::get_by_user_id(&state.client, user_id).await;
    Json(
        records
            .iter()
            .map(|row| News {
                id: row.get::<_, i32>("id"),
                text: row.get::<_, String>("text"),
            })
            .collect::<Vec<_>>(),
    )
    .into_response()
}

async fn logout(session: Session) -> Response {
    session.remove::<i32>("user_id").await.unwrap();
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        "Вы вышли из системы",
    )
        .into_response()
}
