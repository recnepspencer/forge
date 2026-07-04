#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictIndependencePlannerRouteFamily {
    ConflictRoute,
    IndependenceRoute,
}

impl ConflictIndependencePlannerRouteFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConflictRoute => "conflict-route",
            Self::IndependenceRoute => "independence-route",
        }
    }
}
