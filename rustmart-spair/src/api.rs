use spair::prelude::*;

pub fn get_products() -> spair::Command<crate::App> {
    spair::Request::get("/products/products.json")
        .text_mode()
        .response()
        .json(crate::App::set_all_products, crate::App::fetch_error)
}

pub fn get_product(id: i32) -> spair::Command<crate::App> {
    spair::Request::get(format!("/products/{}.json", id))
        .text_mode()
        .response()
        .json(crate::App::set_selected_product, crate::App::fetch_error)
}
