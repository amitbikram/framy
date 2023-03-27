use wasm_bindgen;
use web_sys::{self, window};
use leptos_reactive;

fn main() {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let h1 = document.create_element("h1").unwrap();
    h1.set_text_content(Some("Hello WASM!"));
    body.append_child(&h1).unwrap();
}
