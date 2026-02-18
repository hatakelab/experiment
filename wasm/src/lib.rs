use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement};

#[wasm_bindgen]
pub fn get_input_value(id: &str) -> String {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    // IDから要素を取得し、HtmlInputElementにキャスト
    let input = document
        .get_element_by_id(id)
        .expect("should have input element")
        .dyn_into::<HtmlInputElement>()
        .expect("element should be an HtmlInputElement");

    input.value() // 入力された値を取得
}
