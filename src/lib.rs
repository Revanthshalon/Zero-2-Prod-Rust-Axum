use axum::{routing::get, Router};

pub async fn health_check() -> String {
    String::from("Hello World")
}

pub async fn run() -> Result<(), std::io::Error> {
    let router = Router::new().route("/health_check", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Error Binding");

    axum::serve(listener, router).await
}
