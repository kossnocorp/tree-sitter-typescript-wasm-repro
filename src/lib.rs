use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_typescript(source: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    tree.root_node().to_sexp()
}

#[wasm_bindgen]
pub fn parse_tsx(source: &str) -> String {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TSX.into())
        .unwrap();
    let tree = parser.parse(source, None).unwrap();
    tree.root_node().to_sexp()
}
