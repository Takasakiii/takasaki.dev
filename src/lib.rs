mod pages;

use pages::App;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(data: &str);
}

#[allow(unused_unsafe, dead_code)]
pub fn console_log(data: &str) {
    unsafe { log(data) }
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::start_app::<App>();
}
