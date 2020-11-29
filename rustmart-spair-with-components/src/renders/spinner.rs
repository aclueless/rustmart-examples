use spair::prelude::*;
pub struct Spinner;
impl<C: Component> spair::Render<C> for Spinner {
    fn render(self, nodes: spair::Nodes<C>) {
        nodes.div(|d| {
            d.static_attributes()
                .class("loading_spinner_container")
                .nodes()
                .div(|d| d.static_attributes().class("loading_spinner").done())
                .div(|d| {
                    d.static_attributes()
                        .class("loading_spinner_text")
                        .nodes()
                        .r#static("Loading ...");
                });
        });
    }
}
