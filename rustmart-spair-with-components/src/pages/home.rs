use crate::renders::{Error, Spinner};
use crate::types::Product;
use spair::prelude::*;

pub struct Home {
    pub all_products: Vec<Product>,
    pub parent_comp: spair::Comp<crate::App>,
    pub error_message: Option<spair::FetchError>,
}

impl Home {
    pub fn fetch_all_products(&mut self) -> spair::Command<Self> {
        crate::api::get_products()
    }

    pub fn fetch_error(&mut self, e: spair::FetchError) {
        self.error_message = Some(e);
    }

    pub fn set_all_products(&mut self, all: Vec<Product>) {
        self.all_products = all;
    }
}

impl spair::Component for Home {
    type Routes = ();

    fn render(&self, c: spair::Context<Self>) {
        let (_comp, element) = c.into_parts();
        element.nodes().match_if(|arm| {
            match (self.error_message.as_ref(), self.all_products.is_empty()) {
                (Some(error_message), _) => arm
                    .render_on_arm_index(0)
                    .render(Error(error_message))
                    .done(),
                (None, true) => arm.render_on_arm_index(1).render(Spinner).done(),
                (None, false) => arm
                    .render_on_arm_index(2)
                    .div(|d| {
                        d.static_attributes()
                            .class("product_card_list")
                            .list(&self.all_products, spair::ListElementCreation::Clone)
                    })
                    .done(),
            }
        });
    }
}

impl spair::WithParentComp for Home {
    type Parent = crate::App;
    fn with_parent_and_comp(
        parent_comp: &spair::Comp<Self::Parent>,
        _comp: spair::Comp<Self>,
    ) -> Self {
        Self {
            all_products: Vec::new(),
            parent_comp: parent_comp.clone(),
            error_message: None,
        }
    }
}
