mod auto_key_map;
mod edit;
mod engine;
mod screen;
mod selection;
mod window;

use std::path::Path;

use log::LevelFilter;

use engine::Buffer;
use screen::Screen;

fn main() {
    use regex::Regex;

    simple_logging::log_to_file("my_log.txt", LevelFilter::Info).unwrap();
    let args = std::env::args().collect::<Vec<_>>();
    let filename = Path::new(args.get(1).unwrap());
    let content = std::fs::read_to_string(&filename).unwrap();
    let language = match filename.extension().unwrap().to_str().unwrap() {
        "js" | "jsx" => tree_sitter_javascript::language(),
        "ts" => tree_sitter_typescript::language_typescript(),
        "tsx" => tree_sitter_typescript::language_tsx(),
        "rs" => tree_sitter_rust::language(),
        "md" => tree_sitter_md::language(),
        _ => panic!("Unsupported file extension"),
    };

    let mut screen = Screen::new();
    screen.run(Buffer::new(language, &content)).unwrap();
}
