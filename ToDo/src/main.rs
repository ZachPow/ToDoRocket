use axum::routing::any_service;
use axum::routing::get_service;
use axum::{response::Html, routing::get, Router};
use std::path::Path;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};
use tower_http::services::ServeDir;
use tower_http::services::ServeFile;

#[tokio::main]
async fn main() {
    let service = ServeDir::new("static");

    // build our application with a route
    let app = Router::new().nest_service("/", get_service(service).post(post_handle));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn post_handle(){
    println!("recieved post");
}
