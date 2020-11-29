use crate::renders::Spinner;
use spair::prelude::*;

pub struct Home;
impl spair::Render<crate::App> for Home {
    fn render(self, nodes: spair::Nodes<crate::App>) {
        let state = nodes.state();
        nodes.match_if(|arm| {
            match (
                state.error_message.as_ref(),
                state.data.all_products.is_empty(),
            ) {
                (Some(error_message), _) => arm.render_on_arm_index(0).render(error_message).done(),
                (None, true) => arm.render_on_arm_index(1).render(Spinner).done(),
                (None, false) => arm
                    .render_on_arm_index(2)
                    .div(|d| {
                        d.static_attributes()
                            .class("product_card_list")
                            .list(&state.data.all_products, spair::ListElementCreation::Clone)
                    })
                    .done(),
            }
        });
    }
}
