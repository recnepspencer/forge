#[cfg_attr(not(test), allow(dead_code))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IdentityEvolutionSyntheticScenario {
    Standard,
    BranchLocalDivergence,
    BranchCrossingLineageDenied,
    BranchLocalComparison,
    AmbiguousCorrespondence,
    IdentityBreak,
    UnsupportedLineageTraversal,
    UnsupportedComparisonFamily,
    BroadLineageScanDenied,
    ComplexityContractViolationDenied,
    LineageToCorrespondenceFallbackDenied,
    AdvisoryAsAuthoritativeDenied,
}
