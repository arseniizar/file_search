#[cfg(test)]
mod tests {
    use file_search::{database, indexing};
    #[test]
    fn test_database() {
        let conn = database::setup().unwrap();
        assert!(conn.execute("SELECT 1", []).is_ok());
    }

    #[test]
    fn test_indexing() {
        indexing::index_files("./sample_directory");
        let results = indexing::search("test");
        assert!(results.len() > 0);
    }

    #[test]
    fn test_web() {}
}
