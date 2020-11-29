mod api;
mod app;
mod pages;
mod renders;
mod route;
mod types;

use app::App;
use spair::prelude::*;

impl spair::Application for App {
    fn with_comp(_: spair::Comp<Self>) -> Self {
        Self {
            data: app::Data {
                cart_products: Vec::new(),
                all_products: Vec::new(),
                selected_product: None,
            },
            error_message: None,
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::mount_to_body();
}
