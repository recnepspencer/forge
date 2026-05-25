use super::class::ForgeQueryLowerAuthorityRouteFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRouteSegment {
    family: ForgeQueryLowerAuthorityRouteFamily,
    reason: String,
}

impl ForgeQueryDeclarationRouteSegment {
    pub(crate) fn new(family: ForgeQueryLowerAuthorityRouteFamily, reason: String) -> Self {
        Self { family, reason }
    }

    pub fn family(&self) -> ForgeQueryLowerAuthorityRouteFamily {
        self.family
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRouteSet {
    routes: Vec<ForgeQueryDeclarationRouteSegment>,
    route_families: Vec<ForgeQueryLowerAuthorityRouteFamily>,
}

impl ForgeQueryDeclarationRouteSet {
    pub(crate) fn new(routes: Vec<ForgeQueryDeclarationRouteSegment>) -> Self {
        let route_families = routes.iter().map(|route| route.family()).collect();
        Self {
            routes,
            route_families,
        }
    }

    pub fn primary_route(&self) -> Option<&ForgeQueryDeclarationRouteSegment> {
        self.routes.first()
    }

    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    pub fn routes(&self) -> &[ForgeQueryDeclarationRouteSegment] {
        &self.routes
    }

    pub fn route_families(&self) -> &[ForgeQueryLowerAuthorityRouteFamily] {
        &self.route_families
    }
}
