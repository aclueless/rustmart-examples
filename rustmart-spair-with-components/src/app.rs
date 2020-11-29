use crate::pages::Pages;
use crate::renders::Navbar;
use crate::route::Route;
use crate::types::{CartProduct, Product};
use spair::prelude::*;

pub struct Data {
    pub cart_products: Vec<CartProduct>,
}

pub struct App {
    pub data: Data,
    comp: spair::Comp<Self>,
    page: Pages,
}

impl App {
    pub fn new(comp: spair::Comp<Self>) -> Self {
        Self {
            data: Data {
                cart_products: Vec::new(),
            },
            comp,
            page: Pages::None,
        }
    }

    pub fn set_route(&mut self, route: Route) {
        match route {
            Route::HomePage => {
                self.page = Pages::home(&self.comp);
            }
            Route::ProductDetail(id) => {
                self.page = Pages::product_detail(&self.comp, id);
            }
        }
    }

    pub fn add_to_cart(&mut self, p: Product) {
        let cart_product = self
            .data
            .cart_products
            .iter_mut()
            .find(|cp: &&mut CartProduct| cp.product.id == p.id);

        if let Some(cp) = cart_product {
            cp.quantity += 1;
        } else {
            self.data.cart_products.push(CartProduct {
                product: p.clone(),
                quantity: 1,
            })
        }
    }

    pub fn cart_value(&self) -> f64 {
        self.data.cart_products.iter().fold(0f64, |sum, cp| {
            sum + (cp.quantity as f64) * cp.product.price
        })
    }
}

impl spair::Component for App {
    type Routes = Route;
    fn render(&self, element: spair::Element<Self>) {
        element.nodes().render(Navbar).div(|d| match &self.page {
            Pages::Home(comp) => d.component(comp),
            Pages::ProductDetail(comp) => d.component(comp),
            Pages::None => {}
        });
    }
}
