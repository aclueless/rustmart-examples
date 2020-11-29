use spair::prelude::*;
pub struct Error<'a>(pub &'a spair::FetchError);
impl<'a, C: spair::Component> spair::Render<C> for Error<'a> {
    fn render(self, nodes: spair::Nodes<C>) {
        nodes.div(|d| {
            d.nodes()
                .span(|s| s.nodes().render(&self.0.to_string()).done());
        });
    }
}
