#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarPredicateAuthorityPosture {
    Certified,
    PredicateUncertain,
}

impl PlanarPredicateAuthorityPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::PredicateUncertain => "predicate-uncertain",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarPredicateAuthorityDenial {
    CertifiedZeroDeniedBeforeRepair,
}

impl PlanarPredicateAuthorityDenial {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedZeroDeniedBeforeRepair => "certified-zero-denied-before-repair",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarPredicateEvaluationFailureKind {
    NonFiniteProjectedPoint2,
    CertifiedPredicateMathFailure,
}

impl PlanarPredicateEvaluationFailureKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonFiniteProjectedPoint2 => "non-finite-projected-point2",
            Self::CertifiedPredicateMathFailure => "certified-predicate-math-failure",
        }
    }
}
