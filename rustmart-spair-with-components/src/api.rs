use spair::prelude::*;

pub fn get_products() -> spair::Command<crate::pages::Home> {
    spair::Request::get("/products/products.json")
        .into_fetch_args()
        .json_response(
            crate::pages::Home::set_all_products,
            crate::pages::Home::fetch_error,
        )
}

pub fn get_product(id: i32) -> spair::Command<crate::pages::ProductDetail> {
    spair::Request::get(format!("/products/{}.json", id))
        .into_fetch_args()
        .json_response(
            crate::pages::ProductDetail::set_product,
            crate::pages::ProductDetail::fetch_error,
        )
}
