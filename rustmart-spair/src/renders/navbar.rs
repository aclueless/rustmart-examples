use spair::prelude::*;

pub struct Navbar;
impl spair::Render<crate::App> for Navbar {
    fn render(self, nodes: spair::Nodes<crate::App>) {
        nodes.div(|d| {
            d.static_attributes()
                .class("navbar")
                .nodes()
                .div(|d| {
                    d.static_attributes().class("navbar_title").nodes().a(|a| {
                        a.static_attributes()
                            .href(&crate::route::Route::HomePage)
                            .nodes()
                            .r#static("RustMart");
                    });
                })
                .div(|d| {
                    let state = d.state();
                    d.static_attributes()
                        .class("navbar_cart_value")
                        .nodes()
                        .render(&format!("${:.2}", state.cart_value()));
                });
        });
    }
}
