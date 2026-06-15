#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarPredicateKind {
    Orient2d,
}

impl PlanarPredicateKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Orient2d => "orient2d",
        }
    }
}
