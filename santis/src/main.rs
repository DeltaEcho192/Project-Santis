use askama::Template;
use axum::{
    routing::{get, post, put, delete},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    Json, Router, extract::Path,
    extract::{State, path},
    Form
    
};
use tower_http::services::ServeDir;
mod template_struct;
mod datastructs;
use template_struct::*;
use datastructs::*;
use uuid::Uuid;
use sqlx::{SqlitePool, Row};

#[derive(Clone)]
struct Appstate {
    pool: SqlitePool
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("santis.db").await.unwrap();
    let app = Appstate {pool: pool};
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/list", get(list))
        .route("/items", post(add_item))
        .route("/item/:id/edit", get(edit_get))
        .route("/item/:id", put(edit_put))
        .route("/item/:id", delete(edit_delete))
        .route("/search", post(search_list))
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

async fn add_item(State(state): State<Appstate>, Form(payload): Form<Item>) -> impl IntoResponse {
    let item_id = Uuid::new_v4();
    println!("New item id: {}", item_id);
    println!("Payload: {:?}", payload);
    let sql_query = "INSERT INTO items ('item_id', 'item_name', 'size', 'weight',
    'value', 'packed', 'category', 'sub_category', 'box_num') 
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9);";
    let result = sqlx::query(sql_query)
        .bind(item_id.to_string())
        .bind(payload.item_name)
        .bind(payload.size)
        .bind(payload.weight)
        .bind(payload.value)
        .bind(payload.packed.unwrap_or(0))
        .bind(payload.category)
        .bind(payload.sub_category)
        .bind(payload.box_num.unwrap_or(0))
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

async fn edit_get(State(state): State<Appstate>, Path(id): Path<Uuid> ) -> impl IntoResponse {
    println!("{}", id);
    let mut category_values = Vec::from(["KEEP-Store", "KEEP-Take", "SELL", "DONATE"]);
    let sql_query = "SELECT item_id, item_name, category, box_num FROM items WHERE item_id=$1";
    //let sql_query = "SELECT item_id, item_name, category FROM items";
    let result = sqlx::query_as::<_, ItemEdit>(sql_query)
        .bind(id.to_string())
        .fetch_one(&state.pool).await.unwrap();

    println!("{:?}", result.category);
    let idx = category_values.iter().position(|&x| x == result.category ).unwrap();
    category_values.remove(idx);
    category_values.insert(0, result.category.as_str());
    let edit = TableEditTemplate { cats: category_values, item: &result};
    let render = edit.render().unwrap();
    (header_create(), render)
}


async fn edit_put(State(state): State<Appstate>, Path(id): Path<Uuid>, Form(payload): Form<ItemSave>) -> impl IntoResponse {
    println!("{}", id);
    println!("payload: {:?}", payload);
    let sql_query = "Update items set item_name=$1, category=$2, box_num=$3 WHERE item_id=$4";
    let result = sqlx::query(sql_query)
        .bind(&payload.item_name)
        .bind(&payload.category)
        .bind(payload.box_num)
        .bind(id.to_string())
        .execute(&state.pool).await.unwrap();

    println!("{:?}", result);

    let item = ItemEdit { item_id: id.to_string(), item_name: payload.item_name, category: payload.category, box_num: payload.box_num};
    let row = ItemRowTemplate { item: &item};
    let render = row.render().unwrap();
    (header_create(), render)
}

async fn edit_delete(State(state): State<Appstate>, Path(id): Path<Uuid>) -> impl IntoResponse {
    println!("{}", id);
    let sql_query = "DELETE FROM items WHERE item_id=$1";
    let result = sqlx::query(sql_query)
        .bind(id.to_string())
        .execute(&state.pool).await.unwrap();

    println!("{:?}", result);
    (header_create(), "")
}

async fn list(State(state): State<Appstate>) -> impl IntoResponse {
    println!("Rendering list");
    let box_sql_query = "SELECT box_num FROM items GROUP BY box_num";
    let boxes:Vec<i64> = sqlx::query(box_sql_query).fetch_all(&state.pool).await.unwrap().iter().map(|item_row| {item_row.get(0)}).collect();
    println!("{:?}", boxes);
    let sql_query = "SELECT item_id, item_name, category, box_num FROM items";
    let result:Vec<ItemEdit> = sqlx::query_as::<_, ItemEdit>(sql_query).fetch_all(&state.pool).await.unwrap()
        .iter().map(|item_row| ItemEdit {
            item_id: String::from(&item_row.item_id),
            item_name: String::from(&item_row.item_name),
            category: String::from(&item_row.category),
            box_num: item_row.box_num 
        }).collect();
    let list = ListTemplate { boxs: boxes, items: result};
    let render = list.render().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charseet=utf-8".parse().unwrap());
    (headers, render)
}


async fn search_list(State(state): State<Appstate>, Form(payload): Form<Search>) -> impl IntoResponse {
    println!("Searching List");

    let sql_query = format!("SELECT item_id, item_name, category, box_num FROM items WHERE item_name LIKE '%{}%' and box_num=$1", payload.search);
    let result:Vec<ItemEdit> = sqlx::query_as::<_, ItemEdit>(sql_query.as_str()).bind(payload.box_num).fetch_all(&state.pool).await.unwrap()
        .iter().map(|item_row| ItemEdit {
            item_id: String::from(&item_row.item_id),
            item_name: String::from(&item_row.item_name),
            category: String::from(&item_row.category),
            box_num: item_row.box_num 
        }).collect();
    let list = SearchTemplate { items: result };
    let render = list.render().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charseet=utf-8".parse().unwrap());
    (headers, render)
}

//async fn sort_list(State(state): State<Appstate>, Form(payload): Form<Sort>) -> impl IntoResponse {


//}

fn header_create() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "text/html; charseet=utf-8".parse().unwrap());
    headers
}
