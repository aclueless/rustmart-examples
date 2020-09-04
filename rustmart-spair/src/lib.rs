mod api;
mod app;
mod pages;
mod renders;
mod route;
mod types;

use app::App;
use spair::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::mount_to_body();
}
