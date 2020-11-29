use spair::prelude::*;
pub struct Spinner;
impl spair::Render<crate::App> for Spinner {
    fn render(self, nodes: spair::Nodes<crate::App>) {
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
