use axum::Router;

#[derive(Clone, Debug)]
pub struct WorthServerProjectedRouter {
    router: Router,
}

impl WorthServerProjectedRouter {
    pub(crate) fn new(router: Router) -> Self {
        Self { router }
    }

    pub fn clone_axum_router(&self) -> Router {
        self.router.clone()
    }
}
