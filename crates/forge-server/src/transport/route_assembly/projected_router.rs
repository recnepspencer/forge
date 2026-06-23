use axum::Router;

#[derive(Clone, Debug)]
pub struct ForgeServerProjectedRouter {
    router: Router,
}

impl ForgeServerProjectedRouter {
    pub(crate) fn new(router: Router) -> Self {
        Self { router }
    }

    pub fn clone_axum_router(&self) -> Router {
        self.router.clone()
    }
}
