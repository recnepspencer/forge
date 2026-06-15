#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarAdmissionClass {
    Admitted,
    Denied,
    Unsupported,
    PolicyRequired,
    PredicateUncertainReserved,
}

impl PlanarAdmissionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
            Self::Unsupported => "unsupported",
            Self::PolicyRequired => "policy-required",
            Self::PredicateUncertainReserved => "predicate-uncertain-reserved",
        }
    }

    pub const fn admits_runtime(self) -> bool {
        matches!(self, Self::Admitted)
    }
}
