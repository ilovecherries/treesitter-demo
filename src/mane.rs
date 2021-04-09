use tree_sitter::{Parser, Language};

fn main() {
    extern "C" { fn tree_sitter_javascript() -> Language; }

    // get a parser from one of the languages
    let language = unsafe { tree_sitter_javascript() };
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();

    // parse some source code
    let source_code = "function test() { console.log('Hello, World!'); }";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    println!("{}", root_node.to_sexp());
}
