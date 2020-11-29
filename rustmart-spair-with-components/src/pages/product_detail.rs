use crate::renders::AtcButton;
use crate::renders::{Error, Spinner};
use crate::types::Product;
use spair::prelude::*;

pub struct ProductDetail {
    pub product: Option<Product>,
    pub parent_comp: spair::Comp<crate::App>,
    pub error_message: Option<spair::FetchError>,
}

impl ProductDetail {
    pub fn fetch_product(&mut self, id: i32) -> spair::Command<Self> {
        self.product = None;
        crate::api::get_product(id)
    }

    pub fn fetch_error(&mut self, e: spair::FetchError) {
        self.error_message = Some(e);
    }

    pub fn set_product(&mut self, p: Product) {
        self.product = Some(p);
    }
}

impl spair::Component for ProductDetail {
    type Routes = ();
    fn render(&self, element: spair::Element<Self>) {
        element.nodes().match_if(|arm| {
            match (self.error_message.as_ref(), self.product.as_ref()) {
                (Some(error_message), _) => arm
                    .render_on_arm_index(0)
                    .render(Error(error_message))
                    .done(),
                (None, Some(product)) => arm.render_on_arm_index(1).render(product).done(),
                (None, None) => arm.render_on_arm_index(2).render(Spinner).done(),
            }
        });
    }
}

impl spair::WithParentComp for ProductDetail {
    type Parent = crate::App;
    fn with_parent_and_comp(
        parent_comp: &spair::Comp<Self::Parent>,
        _comp: spair::Comp<Self>,
    ) -> Self {
        Self {
            product: None,
            parent_comp: parent_comp.clone(),
            error_message: None,
        }
    }
}

impl spair::Render<ProductDetail> for &Product {
    fn render(self, nodes: spair::Nodes<ProductDetail>) {
        let comp = nodes.comp();
        let p = self.clone();
        let handler = comp.handler(move |state| {
            let p = p.clone();
            spair::update_component(
                state
                    .parent_comp
                    .callback(move |state| state.add_to_cart(p.clone())),
            )
        });
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
                .render(AtcButton(handler));
        });
    }
}
