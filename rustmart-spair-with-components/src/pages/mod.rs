mod home;
mod product_detail;

pub use home::Home;
pub use product_detail::ProductDetail;

pub enum Pages {
    None,
    Home(spair::ChildComp<Home>),
    ProductDetail(spair::ChildComp<ProductDetail>),
}

impl Pages {
    pub fn home(comp: &spair::Comp<crate::App>) -> Self {
        let home = spair::ChildComp::with_parent(comp);
        spair::update_component(home.comp().callback(crate::pages::Home::fetch_all_products));
        Self::Home(home)
    }

    pub fn product_detail(comp: &spair::Comp<crate::App>, id: i32) -> Self {
        let detail = spair::ChildComp::with_parent(comp);
        spair::update_component(
            detail
                .comp()
                .callback(move |state: &mut ProductDetail| state.fetch_product(id)),
        );
        Self::ProductDetail(detail)
    }
}
