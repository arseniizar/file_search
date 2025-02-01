use file_search::{database, indexing, web};

#[tokio::main]
async fn main() {
    let db_path = "file_index.db";

    let conn = database::setup(db_path).expect("Failed to connect to the database");

    database::insert_file_metadata_if_not_exists(&conn, "/example.txt", "example.txt" , "1672531200")
        .expect("Failed to insert file metadata");

    let index_path = "./tantivy_index";
    let index = indexing::create_index(index_path);
    indexing::index_files("./sample_directory", &conn, &index)
        .expect("Failed to index files");

    println!("Server running at http://127.0.0.1:3030/");
    web::start_server(conn).await;
}
