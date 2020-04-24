use futures_signals::signal::{Signal, SignalExt};
use dominator::{routing, html};
use crate::app::App;
use crate::app::APP;
use wasm_bindgen::prelude::*;
mod app;
mod utils;





#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    APP.with(|x| dominator::append_dom(&dominator::body(), App::render(x.clone())));

    Ok(())
}