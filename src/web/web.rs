use dioxus::dioxus_core::Mutations;
use dioxus_ssr::render;
use rusqlite::Connection;
use serde::{Deserialize};
use std::sync::{Arc, Mutex};
use warp::Filter;
use crate::web::components;

#[derive(Debug, Deserialize)]
struct FileMetadata {
    path: String,
    name: String,
    modified_time: String,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    query: String,
}

async fn search_handler(
    query: String,
    conn: Arc<Mutex<Connection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn_guard = conn.lock().unwrap();
    let mut stmt = conn_guard
        .prepare("SELECT name, path FROM file_metadata WHERE name LIKE ?1")
        .expect("Failed to prepare statement");
    let search_pattern = format!("%{}%", query);
    let rows = stmt
        .query_map([&search_pattern], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .expect("Failed to execute query");
    let mut results = String::new();
    for row in rows {
        if let Ok((name, path)) = row {
            results.push_str(&format!("<p>{}: {}</p>", name, path));
        }
    }
    if results.is_empty() {
        Ok(warp::reply::html(format!(
            "<h1>No results found for '{}'</h1>",
            query
        )))
    } else {
        Ok(warp::reply::html(format!(
            "<h1>Results for '{}'</h1>{}",
            query, results
        )))
    }
}

async fn add_file_handler(
    file: FileMetadata,
    conn: Arc<Mutex<Connection>>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let conn = conn.lock().unwrap();
    match crate::database::insert_file_metadata(&conn, &file.path, &file.name, &file.modified_time)
    {
        Ok(_) => Ok(warp::reply::json(
            &serde_json::json!({"status": "success", "message": "File added to database"}),
        )),
        Err(e) => Ok(warp::reply::json(
            &serde_json::json!({"status": "error", "message": e.to_string()}),
        )),
    }
}

async fn dioxus_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let mut vdom = dioxus::dioxus_core::VirtualDom::new(components::app);
    let mut mutations = Mutations::default();
    vdom.rebuild(&mut mutations);
    let html = render(&vdom);
    Ok(warp::reply::html(html))
}

pub async fn start_server(conn: Connection) {
    let conn = Arc::new(Mutex::new(conn));
    let search_conn = Arc::clone(&conn);
    let search_route = warp::path("search")
        .and(warp::get())
        .and(warp::query::<SearchQuery>())
        .and(warp::any().map(move || Arc::clone(&search_conn)))
        .and_then(
            |search: SearchQuery, conn| async move { search_handler(search.query, conn).await },
        );
    let add_conn = Arc::clone(&conn);
    let add_file_route = warp::post()
        .and(warp::path("add"))
        .and(warp::body::json())
        .and(warp::any().map(move || Arc::clone(&add_conn)))
        .and_then(add_file_handler);
    let dioxus_route = warp::path::end().and_then(dioxus_handler);
    let routes = search_route.or(add_file_route).or(dioxus_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
