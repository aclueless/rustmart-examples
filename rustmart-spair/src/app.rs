use crate::pages::{Home, ProductDetail};
use crate::renders::Navbar;
use crate::route::Route;
use crate::types::{CartProduct, Product};
use spair::prelude::*;

pub struct Data {
    pub cart_products: Vec<CartProduct>,
    pub all_products: Vec<Product>,
    pub selected_product: Option<Product>,
}

pub struct App {
    pub data: Data,
    pub error_message: Option<spair::FetchError>,
}

impl App {
    pub fn set_route(&mut self, route: Route) -> spair::Checklist<Self> {
        match route {
            Route::HomePage => {
                self.data.selected_product = None;
                crate::api::get_products().into()
            }
            Route::ProductDetail(id) => {
                let mut checklist = Self::default_checklist();
                if self.data.all_products.is_empty() {
                    checklist.add_command(crate::api::get_products());
                }
                checklist.add_command(crate::api::get_product(id));
                checklist
            }
        }
    }

    pub fn fetch_error(&mut self, e: spair::FetchError) {
        self.error_message = Some(e);
    }

    pub fn set_all_products(&mut self, all: Vec<Product>) {
        self.data.all_products = all;
    }

    pub fn set_selected_product(&mut self, p: Product) {
        self.data.selected_product = Some(p);
    }

    pub fn add_to_cart(&mut self, product: Product) {
        let cart_product = self
            .data
            .cart_products
            .iter_mut()
            .find(|cp: &&mut CartProduct| cp.product.id == product.id);

        if let Some(cp) = cart_product {
            cp.quantity += 1;
        } else {
            self.data.cart_products.push(CartProduct {
                product,
                quantity: 1,
            });
        }
    }

    pub fn cart_value(&self) -> f64 {
        self.data.cart_products.iter().fold(0f64, |acc, cp| {
            acc + cp.quantity as f64 * cp.product.price
        })
    }
}

impl spair::Component for App {
    type Routes = Route;
    fn render(&self, element: spair::Element<Self>) {
        element
            .nodes()
            .render(Navbar)
            .match_if(|arm| match self.data.selected_product.is_some() {
                false => arm.render_on_arm_index(0).render(Home).done(),
                true => arm.render_on_arm_index(1).render(ProductDetail).done(),
            });
    }
}
