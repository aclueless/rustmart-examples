use spair::prelude::*;
impl spair::Render<crate::App> for &spair::FetchError {
    fn render(self, nodes: spair::Nodes<crate::App>) {
        nodes.div(|d| {
            d.nodes()
                .span(|s| s.nodes().render(&self.to_string()).done());
        });
    }
}
