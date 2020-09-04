use crate::renders::AtcButton;
use crate::route::Route;
use crate::types::Product;
use spair::prelude::*;

impl spair::ListItem<crate::pages::Home> for Product {
    const ROOT_ELEMENT_TAG: &'static str = "div";
    fn render(&self, element: spair::Element<crate::pages::Home>) {
        let comp = element.comp();
        let p = self.clone();
        let handler = comp.handler(move |state| {
            let p = p.clone();
            spair::update_component(
                state
                    .parent_comp
                    .callback(move |state| state.add_to_cart(p.clone())),
            )
        });
        element
            .static_attributes()
            .class("product_card_container")
            .nodes()
            .a(|a| {
                a.static_attributes()
                    .class("product_card_anchor")
                    .attributes()
                    // a workaround, because href only accept App::Routes, but we are actually in Home
                    .href_str(&Route::ProductDetail(self.id).url())
                    .nodes()
                    .img(|i| {
                        i.static_attributes()
                            .class("product_card_image")
                            .attributes()
                            .src(&self.image);
                    })
                    .div(|d| {
                        d.static_attributes()
                            .class("product_card_name")
                            .nodes()
                            .render(&self.name);
                    })
                    .div(|d| {
                        d.static_attributes()
                            .class("product_card_price")
                            .nodes()
                            .r#static("$")
                            .render(self.price);
                    });
            })
            .render(AtcButton(handler));
    }
}
