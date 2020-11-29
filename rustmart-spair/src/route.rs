#[derive(Debug, Clone)]
pub enum Route {
    ProductDetail(i32),
    HomePage,
}

impl spair::Routes<crate::App> for Route {
    fn url(&self) -> String {
        match self {
            Self::ProductDetail(id) => format!("#product/{}", id),
            Self::HomePage => "#".to_string(),
        }
    }

    fn routing(location: spair::web_sys::Location, comp: &spair::Comp<crate::App>) {
        const PRODUCT_HASH: &'static str = "#product/";
        let route = location
            .hash()
            .ok()
            .and_then(|hash| {
                hash.get(PRODUCT_HASH.len()..)
                    .and_then(|id_str| id_str.parse::<i32>().ok())
            })
            .map(|id| Self::ProductDetail(id))
            .unwrap_or_else(|| Self::HomePage);
        comp.callback_arg(crate::App::set_route)(route);
    }
}
