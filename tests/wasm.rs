use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_parse_typescript() {
    let result = tree_sitter_typescript_wasm_repro::parse_typescript(
        "function main() { console.log(123); }",
    );
    assert!(result.contains("function_declaration"));
}

#[wasm_bindgen_test]
fn test_parse_tsx() {
    let result = tree_sitter_typescript_wasm_repro::parse_tsx(
        "function main() { console.log(<span>123</span>); }",
    );
    assert!(result.contains("function_declaration"));
}
