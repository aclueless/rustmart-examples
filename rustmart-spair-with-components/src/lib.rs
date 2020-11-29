mod api;
mod app;
mod pages;
mod renders;
mod route;
mod types;

use app::App;
use spair::prelude::*;

impl spair::Application for App {
    fn with_comp(comp: spair::Comp<Self>) -> Self {
        Self::new(comp)
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::mount_to_body();
}
