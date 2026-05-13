#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalEquivalenceBasis {
    ExactCanonicalBasis,
    DeclaredAspectEquivalence,
    CompatibilityLoweredNativeEquivalence,
    ProjectionEquivalent,
    DigestEquivalent,
}
