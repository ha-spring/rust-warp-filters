use crate::security::{check_auth, UserCtx};
use serde_json::{json, Value};
use warp::{reply::Json, Filter};

pub fn todos_filter() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let todos_base = warp::path("todos");

    // List todos
    let list = todos_base
        .and(warp::get())
        .and(warp::path::end())
        .and(check_auth())
        .and_then(todo_list);

    let get = todos_base
        .and(warp::get())
        .and(check_auth())
        .and(warp::path::param()) // e.g. /todos/2
        .and_then(todo_get);

    let create = todos_base
        .and(warp::post())
        .and(check_auth())
        .and(warp::body::json())
        .and_then(todo_create);

    list.or(get).or(create)
}

async fn todo_list(user_ctx: UserCtx) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    let todos = json!([
      {"id": 1, "title": "todo 1", "user_id": user_ctx.user_id},
      {"id": 2, "title": "todo 2", "user_id": user_ctx.user_id},
    ]);

    let todos = warp::reply::json(&todos);

    Ok(todos)
}

async fn todo_get(_user_ctx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
    // TODO - get from DB
    let todo = json!(
      {"id": id, "title": format!("todo {}", id)});

    let todo = warp::reply::json(&todo);

    Ok(todo)
}

async fn todo_create(_user_ctx: UserCtx, data: Value) -> Result<Json, warp::Rejection> {
    // TODO - write to DB
    let todo = data;

    let todo = warp::reply::json(&todo);

    Ok(todo)
}
