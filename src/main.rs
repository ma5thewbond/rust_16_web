use axum::{response::IntoResponse, routing::get, Router};
use rust_16_web::tpl_listusers::ListUsersTemplate;
use rust_16_web::HtmlTemplate;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn list_users() -> impl IntoResponse {
    let template = ListUsersTemplate {
        users: vec![vec!["name".into()]], //get_users_from_db(), //vec![vec!["name".into()]]
    };
    HtmlTemplate::new(template)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(list_users));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    let listener: TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
