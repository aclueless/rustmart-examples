use crate::renders::AtcButton;
use crate::route::Route;
use crate::types::Product;
use spair::prelude::*;

impl spair::ListItem<crate::App> for Product {
    const ROOT_ELEMENT_TAG: &'static str = "div";
    fn render(&self, element: spair::Element<crate::App>) {
        element
            .static_attributes()
            .class("product_card_container")
            .nodes()
            .a(|a| {
                a.href(&Route::ProductDetail(self.id))
                    .static_attributes()
                    .class("product_card_anchor")
                    .nodes()
                    .img(|i| {
                        i.src(&self.image)
                            .static_attributes()
                            .class("product_card_image");
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
            .render(AtcButton(self.clone()));
    }
}
