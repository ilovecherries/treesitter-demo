use std::path::PathBuf;

fn main() {
    // get directory of treesitter backend
    let dir: PathBuf = ["tree-sitter-javascript", "src"].iter().collect();

    // build the treesitter backend
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-javascript");
}