use tree_sitter::{Parser, Language};
use std::path::PathBuf;

extern "C" { fn tree_sitter_javascript() -> Language; }

fn main() {
    // get directory of treesitter backend
    let dir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();

    // build the treesitter backend
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-javascript");

    // get a parser from one of the languages
    let language = unsafe { tree_sitter_javascript() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    // parse some source code
    let source_code = "function test() { console.log('Hello, World!'); }";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    println!("{}", root_node.to_sexp());

    println!("Hello, world!");
}
