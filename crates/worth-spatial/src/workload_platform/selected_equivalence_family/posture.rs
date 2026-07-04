#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialOrderingNoisePosture {
    ExactOrderingRequired,
    DeclaredBenignOrderingNoiseAllowed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialFreshnessRequirementPosture {
    SameAdmittedAuthorityAndLocalityRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialRenderedOutputComparisonPosture {
    NotPartOfBasis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpatialCompatibilityPosture {
    DistinctFromEquivalence,
}
