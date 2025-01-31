use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{doc, schema::*, Document, Index};

fn map_io_error<E>(err: E) -> rusqlite::Error
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    rusqlite::Error::ToSqlConversionFailure(err.into())
}

pub fn create_index(index_path: &str) -> Index {
    let path = Path::new(index_path);

    if !path.exists() {
        fs::create_dir_all(path).expect("Failed to create Tantivy index directory");
    }

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("content", TEXT | STORED);
    let schema = schema_builder.build();

    match Index::open_in_dir(index_path) {
        Ok(index) => {
            println!("Opened existing Tantivy index at '{}'", index_path);
            index
        }
        Err(_) => {
            println!(
                "No valid Tantivy index found at '{}'. Creating a new one.",
                index_path
            );
            Index::create_in_dir(index_path, schema).expect("Failed to create new index")
        }
    }
}

pub fn index_files(directory: &str, conn: &Connection, index: &Index) -> rusqlite::Result<()> {
    if !Path::new(directory).exists() {
        return Err(rusqlite::Error::ToSqlConversionFailure(Box::new(
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory '{}' does not exist", directory),
            ),
        )));
    }

    let schema = index.schema();
    let text_field = schema.get_field("content").unwrap();
    let mut writer = index.writer(50_000_000).unwrap();

    let paths = fs::read_dir(directory).map_err(map_io_error)?;

    for entry in paths {
        let path = entry.map_err(map_io_error)?.path();

        if path.is_file() {
            let metadata = fs::metadata(&path).map_err(map_io_error)?;

            let modified_time = metadata
                .modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string();

            let path_str = path.to_string_lossy().to_string();

            let file_name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            conn.execute(
                "INSERT INTO file_metadata (path, name, modified_time) VALUES (?1, ?2, ?3)",
                params![path_str, file_name, modified_time],
            )?;

            if let Ok(content) = fs::read_to_string(&path) {
                let document = doc!(text_field => content);
                writer
                    .add_document(document)
                    .expect("Failed to index document");
            }
        }
    }

    writer.commit().expect("Failed to commit Tantivy index");

    Ok(())
}

pub fn search(index_path: &str, query: &str) -> Vec<String> {
    let index = Index::open_in_dir(index_path).expect("Failed to open Tantivy index");
    let schema = index.schema();
    let text_field = schema.get_field("content").unwrap();

    let reader = index.reader().unwrap();
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![text_field]);

    let query = query_parser.parse_query(query).unwrap();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();

    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc::<TantivyDocument>(doc_address).unwrap();
        let named_doc = retrieved_doc.to_named_doc(&schema);
        let doc_json = serde_json::to_string(&named_doc).unwrap();
        results.push(doc_json);
    }

    results
}
