#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum IdentityEvolutionSyntheticScenario {
    Standard,
    BranchLocalDivergence,
    BranchCrossingLineageDenied,
    #[cfg(test)]
    BranchLocalComparison,
    AmbiguousCorrespondence,
    IdentityBreak,
    UnsupportedLineageTraversal,
    UnsupportedComparisonFamily,
    #[cfg(test)]
    BroadLineageScanDenied,
    ComplexityContractViolationDenied,
    #[cfg(test)]
    LineageToCorrespondenceFallbackDenied,
    AdvisoryAsAuthoritativeDenied,
}
