use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path::end().and(warp::get()).map(|| "Hello!\n");

    println!("start server...");
    warp::serve(hello).run(([127, 0, 0, 1], 8080)).await;
}
