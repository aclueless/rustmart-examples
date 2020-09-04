use crate::route::Route;
use crate::types::Product;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

type Anchor = RouterAnchor<Route>;

pub fn product_card(product: &Product, atc_button: Html) -> Html {
    html! {
        <div class="product_card_container">
            <Anchor route=Route::ProductDetail(product.id) classes="product_card_anchor">
                <img class="product_card_image" src={&product.image}/>
                <div class="product_card_name">{&product.name}</div>
                <div class="product_card_price">{"$"}{&product.price}</div>
            </Anchor>
            { atc_button }
        </div>
    }
}
