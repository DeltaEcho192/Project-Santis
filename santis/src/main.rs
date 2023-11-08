use askama::Template;
use axum::{
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json, Router, extract::Path,
    
};
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
mod template_struct;
use template_struct::*;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/list", get(list))
        .route("/items/:id/edit", get(row_edit))
        .nest_service("/assets", ServeDir::new("assets"));

    // run our app with hyper, listening globally on port 3000
    axum::Server::bind(&"0.0.0.0:2502".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    println!("Rendering root");
    let category_values = Vec::from(["KEEP-Store", "KEEP-Take", "SELL", "DONATE"]);
    let size_values = Vec::from(["SMALL", "MEDIUM", "LARGE", "EXTRA LARGE"]);
    let items_values = vec!["1", "2"];
    let root = RootTemplate {cats: category_values, items: items_values ,sizes: size_values, status_message: ""};
    let render = root.render().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charseet=utf-8".parse().unwrap());
    (headers, render)
}

async fn list() -> impl IntoResponse {
    println!("Rendering list");
    let item1 = Items { item_id:"1".into(), item_name:"First".into(), category:"Keep".into() };
    let items = vec![&item1];
    let list = ListTemplate { items: items };
    let render = list.render().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charseet=utf-8".parse().unwrap());
    (headers, render)
}

async fn row_edit(Path(item_id): Path<Uuid>) -> impl IntoResponse {
    println!("Rendering Table edit");
    let item1 = Items { item_id:"1".into(), item_name:"First".into(), category:"Keep".into() };
    let category_values = Vec::from(["KEEP-Store", "KEEP-Take", "SELL", "DONATE"]);
    let edit = TableEditTemplate { cats: category_values, item: &item1 };
    let render = edit.render().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charseet=utf-8".parse().unwrap());
    (headers, render)
}
