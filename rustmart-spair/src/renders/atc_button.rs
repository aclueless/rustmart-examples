use spair::prelude::*;
use crate::types::Product;

pub struct AtcButton(pub Product);
impl spair::Render<crate::App> for AtcButton {
    fn render(self, nodes: spair::Nodes<crate::App>) {
        let comp = nodes.comp();
        nodes.button(|b| {
            b.static_attributes()
                .class("product_atc_button")
                .on_click(comp.handler(move |state| state.add_to_cart(self.0.clone())))
                .nodes()
                .r#static("Add To Cart");
        });
    }
}
