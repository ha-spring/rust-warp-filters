mod security;
mod todo_rest;

use crate::todo_rest::todos_filter;
use warp::Filter;

const STATIC_FOLDER: &str = "static/";

#[tokio::main]
async fn main() {
    // APIs
    let hello = warp::path::end().and(warp::get()).map(|| "Hello!\n");
    let apis = hello.or(todos_filter());

    // Static content
    let content = warp::fs::dir(STATIC_FOLDER);
    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", STATIC_FOLDER)));
    let static_content = content.or(root);

    let routes = apis.or(static_content);

    println!("start server...");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
