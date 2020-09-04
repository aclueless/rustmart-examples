use crate::types::CartProduct;
use yew::prelude::*;

pub fn nav_bar(cart_products: &Vec<CartProduct>) -> Html {
    let cart_value = cart_products
        .iter()
        .fold(0.0, |acc, cp| acc + (cp.quantity as f64 * cp.product.price));

    html! {
        <div class="navbar">
            <div class="navbar_title">{"RustMart"}</div>
            <div class="navbar_cart_value">{format!("${:.2}", cart_value)}</div>
        </div>
    }
}
