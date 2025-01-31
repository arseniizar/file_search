use crate::indexing;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 && args[1] == "index" {
        indexing::index_files("./sample_directory");
    } else if args.len() > 1 && args[1] == "search" {
        if args.len() < 3 {
            println!("Please provide a search query");
            return;
        }
        let results = indexing::search(&args[2]);
        for result in results {
            println!("{}", result);
        }
    } else {
        println!("Usage:");
        println!("  index - Index files in the directory");
        println!("  search <query> - Search indexed files");
    }
}
