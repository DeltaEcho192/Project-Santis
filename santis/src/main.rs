use askama::Template;
use axum::{
    routing::{get, post},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json, Router, extract::Path,
    extract::State
    
};
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
mod template_struct;
mod datastructs;
use template_struct::*;
use datastructs::*;
use uuid::Uuid;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, Row};

#[derive(Clone)]
struct Appstate {
    pool: SqlitePool
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let pool = SqlitePool::connect("santis.db").await.unwrap();
    let app = Appstate {pool: pool};
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/list", get(list))
        .route("/items/:id/edit", get(row_edit))
        .route("/items", post(add_item))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(app);

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

async fn add_item(State(state): State<Appstate>, Json(payload): Json<Item>) -> impl IntoResponse {
    let item_id = Uuid::new_v4();
    println!("New item id: {}", item_id);
    println!("Payload: {:?}", payload);
    let packed:i64 = match payload.packed {
        PackedDynamic::Int(packed) => packed.try_into().unwrap(),
        PackedDynamic::String(packed)  => {
            if packed == "on" {
                1 
            } else { 
                0
            }
        }
    };
    println!("Packed value {}", packed);
    let sql_query = "INSERT INTO items ('item_id', 'item_name', 'size', 'weight',
    'value', 'packed', 'category', 'sub_category') 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8);";
    let result = sqlx::query(sql_query)
        .bind(item_id)
        .bind(payload.item_name)
        .bind(payload.size)
        .bind(payload.weight)
        .bind(payload.value)
        .bind(packed)
        .bind(payload.category)
        .bind(payload.sub_category)
        .execute(&state.pool).await;

    let succ = match result {
        Ok(_) => "Success",
        Err(err) => {
            println!("Err: {}", err);
            "Not Successfull"
        }
    };
    let rt_mesg = EnterMessage  { status_message: succ };
    let render = rt_mesg.render().unwrap();
    (header_create(), render)
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

fn header_create() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charseet=utf-8".parse().unwrap());
    headers
}
