use crate::materialization::MaterializationStateClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyLocalityProfile {
    OrderedPageLocality,
    BufferedRunLocality,
    LinearLayoutTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyAmplificationProfile {
    SplitMergeBounded,
    CompactionWriteAmplified,
    ReadMostlyBounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyCorruptionIsolationBehavior {
    PageScoped,
    RunScoped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyRebuildSourceRequirement {
    PhysicalSnapshotReplay,
    WalReplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrategyMaterializationPosture {
    PublishedTreeLifecycle,
    WalBufferedRunLifecycle,
    RangeScannableSnapshots,
}

impl StrategyMaterializationPosture {
    pub const fn supports_state(self, state: MaterializationStateClass) -> bool {
        match self {
            Self::PublishedTreeLifecycle => matches!(
                state,
                MaterializationStateClass::DeclaredOnly
                    | MaterializationStateClass::EmptyInitialized
                    | MaterializationStateClass::Building
                    | MaterializationStateClass::PartiallyCovered
                    | MaterializationStateClass::Exact
                    | MaterializationStateClass::ExactThroughPhysicalBasis
                    | MaterializationStateClass::Lagged
                    | MaterializationStateClass::Stale
                    | MaterializationStateClass::RebuildRequired
                    | MaterializationStateClass::Migrating
                    | MaterializationStateClass::Quarantined
                    | MaterializationStateClass::Retired
            ),
            Self::WalBufferedRunLifecycle => matches!(
                state,
                MaterializationStateClass::DeclaredOnly
                    | MaterializationStateClass::EmptyInitialized
                    | MaterializationStateClass::Building
                    | MaterializationStateClass::PartiallyCovered
                    | MaterializationStateClass::ExactThroughPhysicalBasis
                    | MaterializationStateClass::Lagged
                    | MaterializationStateClass::Stale
                    | MaterializationStateClass::RebuildRequired
                    | MaterializationStateClass::Migrating
                    | MaterializationStateClass::Quarantined
                    | MaterializationStateClass::Retired
            ),
            Self::RangeScannableSnapshots => matches!(
                state,
                MaterializationStateClass::DeclaredOnly
                    | MaterializationStateClass::EmptyInitialized
                    | MaterializationStateClass::Building
                    | MaterializationStateClass::PartiallyCovered
                    | MaterializationStateClass::Lagged
                    | MaterializationStateClass::Stale
                    | MaterializationStateClass::RebuildRequired
                    | MaterializationStateClass::Migrating
                    | MaterializationStateClass::Quarantined
                    | MaterializationStateClass::Retired
            ),
        }
    }
}
