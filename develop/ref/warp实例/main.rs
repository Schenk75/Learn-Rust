use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use warp::{http, Filter};

/*
    设计比较有特色 String
    如果您的公司经营微服务，那就更容易了
    Rust非常适合替换此类服务，您可以在几天之内重写它
    // parking_lot为您的本地存储创建一个ReadWriteLock
    “过滤器”可以选择从请求中提取一些数据它与其他的，改变它，并返回一些值作为一个应答。
*/
type Items = HashMap<String, i32>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Id {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
    name: String,
    quantity: i32,
}

#[derive(Clone)]
struct Store {
    shop_list: Arc<RwLock<Items>>,
}

impl Store {
    fn new() -> Self {
        Store {
            shop_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

async fn get_shop_list(store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let r = store.shop_list.read();

    for (key, value) in r.iter() {
        result.insert(key, value);
    }

    Ok(warp::reply::json(&result))
}

async fn post_shop_list(item: Item, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.shop_list.write().insert(item.name, item.quantity);

    Ok(warp::reply::with_status(
        "Added items to the grocery list",
        http::StatusCode::CREATED,
    ))
}

async fn delete_shop_list_item(id: Id, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    store.shop_list.write().remove(&id.name);

    Ok(warp::reply::with_status(
        "Removed item from grocery list",
        http::StatusCode::OK,
    ))
}

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 24).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 24).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let add_items = warp::post()
        .and(warp::path("shop"))
        .and(post_json())
        .and(store_filter.clone())
        .and_then(post_shop_list);

    let get_items = warp::get()
        .and(warp::path("shop"))
        .and(store_filter.clone())
        .and_then(get_shop_list);

    let delete_item = warp::delete()
        .and(warp::path("shop"))
        .and(delete_json())
        .and(store_filter.clone())
        .and_then(delete_shop_list_item);
    let routes = add_items.or(get_items).or(delete_item);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
