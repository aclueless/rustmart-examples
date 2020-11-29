use crate::renders::AtcButton;
use crate::renders::Spinner;
use crate::types::Product;
use spair::prelude::*;

pub struct ProductDetail;
impl spair::Render<crate::App> for ProductDetail {
    fn render(self, nodes: spair::Nodes<crate::App>) {
        let state = nodes.state();
        nodes.match_if(|arm| {
            match (
                state.error_message.as_ref(),
                state.data.selected_product.as_ref(),
            ) {
                (Some(error_message), _) => arm.render_on_arm_index(0).render(error_message).done(),
                (None, Some(product)) => arm.render_on_arm_index(1).render(product).done(),
                (None, None) => arm.render_on_arm_index(2).render(Spinner).done(),
            }
        });
    }
}

impl spair::Render<crate::App> for &Product {
    fn render(self, nodes: spair::Nodes<crate::App>) {
        nodes.div(|d| {
            d.static_attributes()
                .class("product_detail_container")
                .nodes()
                .img(|i| {
                    i.src(&self.image)
                        .static_attributes()
                        .class("product_detail_image");
                })
                .div(|d| {
                    d.static_attributes()
                        .class("product_card_name")
                        .nodes()
                        .render(&self.name);
                })
                .div(|d| {
                    d.static_attributes()
                        .style("margin: 10px 0; line-height: 24px;")
                        .nodes()
                        .render(&self.description);
                })
                .div(|d| {
                    d.static_attributes()
                        .class("product_card_price")
                        .nodes()
                        .r#static("$")
                        .render(self.price);
                })
                .render(AtcButton(self.clone()));
        });
    }
}
