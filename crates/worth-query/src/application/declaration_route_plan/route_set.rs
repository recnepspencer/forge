use super::class::WorthQueryLowerAuthorityRouteFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationRouteSegment {
    family: WorthQueryLowerAuthorityRouteFamily,
    reason: String,
}

impl WorthQueryDeclarationRouteSegment {
    pub(crate) fn new(family: WorthQueryLowerAuthorityRouteFamily, reason: String) -> Self {
        Self { family, reason }
    }

    pub fn family(&self) -> WorthQueryLowerAuthorityRouteFamily {
        self.family
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationRouteSet {
    routes: Vec<WorthQueryDeclarationRouteSegment>,
    route_families: Vec<WorthQueryLowerAuthorityRouteFamily>,
}

impl WorthQueryDeclarationRouteSet {
    pub(crate) fn new(routes: Vec<WorthQueryDeclarationRouteSegment>) -> Self {
        let route_families = routes.iter().map(|route| route.family()).collect();
        Self {
            routes,
            route_families,
        }
    }

    pub fn primary_route(&self) -> Option<&WorthQueryDeclarationRouteSegment> {
        self.routes.first()
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn routes(&self) -> &[WorthQueryDeclarationRouteSegment] {
        &self.routes
    }

    pub fn route_families(&self) -> &[WorthQueryLowerAuthorityRouteFamily] {
        &self.route_families
    }
}
