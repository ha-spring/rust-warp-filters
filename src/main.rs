use warp::Filter;

const STATIC_FOLDER: &str = "static/";

#[tokio::main]
async fn main() {
    let hello = warp::path::end().and(warp::get()).map(|| "Hello!\n");

    let content = warp::fs::dir(STATIC_FOLDER);

    let routes = hello.or(content);

    println!("start server...");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
