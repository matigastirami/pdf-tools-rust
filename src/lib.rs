use wasm_bindgen::prelude::*;
use lopdf::Document;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
}

#[wasm_bindgen]
pub fn replace_text_pdf(file_path: &str, tag: &str, text_to_replace: &str) {
    log(&format!("Starting PDF editing process for file {}", file_path));
    let mut doc = Document::load(file_path).unwrap();

    doc.version = "1.4".to_string();
    doc.replace_text(1, tag, text_to_replace);
    // Store file in current working directory.
    // Note: Line is excluded when running tests
    if false {
        doc.save("modified.pdf").unwrap();
    }
}