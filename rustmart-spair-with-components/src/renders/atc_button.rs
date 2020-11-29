use spair::prelude::*;

pub struct AtcButton<F>(pub F);
impl<C: spair::Component, F: spair::Click> spair::Render<C> for AtcButton<F> {
    fn render(self, nodes: spair::Nodes<C>) {
        nodes.button(|b| {
            b.static_attributes()
                .class("product_atc_button")
                .on_click(self.0)
                .nodes()
                .r#static("Add To Cart");
        });
    }
}
