use tantivy::{Index, schema::*, doc};
use serde_json;

pub fn create_index() -> Index {
    let mut schema_builder = Schema::builder();
    let text_field = schema_builder.add_text_field("content", TEXT | STORED);
    let schema = schema_builder.build();
    Index::create_in_ram(schema)
}

pub fn index_files(directory: &str) {
    let index = create_index();
    let schema = index.schema();
    let text_field = schema.get_field("content").unwrap();
    let mut writer = index.writer(50_000_000).unwrap();

    let paths = std::fs::read_dir(directory).unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let content = std::fs::read_to_string(&path).unwrap_or_default();
            writer.add_document(doc!(text_field => content)).expect("Error adding document: ");
        }
    }
    writer.commit().unwrap();
}

pub fn search(query: &str) -> Vec<String> {
    let index = create_index();
    let schema = index.schema();
    let text_field = schema.get_field("content").unwrap();

    let reader = index.reader().unwrap();
    let searcher = reader.searcher();
    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![text_field]);
    let query = query_parser.parse_query(query).unwrap();
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(10)).unwrap();

    let mut results = Vec::new();
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc::<TantivyDocument>(doc_address).unwrap();
        let named_doc = retrieved_doc.to_named_doc(&schema);
        let doc_json = serde_json::to_string(&named_doc).unwrap();
        results.push(doc_json);
    }
    results
}
