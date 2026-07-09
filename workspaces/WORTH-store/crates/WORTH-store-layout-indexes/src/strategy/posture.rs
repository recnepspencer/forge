use crate::materialization::S8MaterializationStateClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyLocalityProfile {
    OrderedPageLocality,
    BufferedRunLocality,
    LinearLayoutTraversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyAmplificationProfile {
    SplitMergeBounded,
    CompactionWriteAmplified,
    ReadMostlyBounded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyCorruptionIsolationBehavior {
    PageScoped,
    RunScoped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyRebuildSourceRequirement {
    PhysicalSnapshotReplay,
    WalReplay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8StrategyMaterializationPosture {
    PublishedTreeLifecycle,
    WalBufferedRunLifecycle,
    RangeScannableSnapshots,
}

impl S8StrategyMaterializationPosture {
    pub const fn supports_state(self, state: S8MaterializationStateClass) -> bool {
        match self {
            Self::PublishedTreeLifecycle => matches!(
                state,
                S8MaterializationStateClass::DeclaredOnly
                    | S8MaterializationStateClass::EmptyInitialized
                    | S8MaterializationStateClass::Building
                    | S8MaterializationStateClass::PartiallyCovered
                    | S8MaterializationStateClass::Exact
                    | S8MaterializationStateClass::ExactThroughPhysicalBasis
                    | S8MaterializationStateClass::Lagged
                    | S8MaterializationStateClass::Stale
                    | S8MaterializationStateClass::RebuildRequired
                    | S8MaterializationStateClass::Migrating
                    | S8MaterializationStateClass::Quarantined
                    | S8MaterializationStateClass::Retired
            ),
            Self::WalBufferedRunLifecycle => matches!(
                state,
                S8MaterializationStateClass::DeclaredOnly
                    | S8MaterializationStateClass::EmptyInitialized
                    | S8MaterializationStateClass::Building
                    | S8MaterializationStateClass::PartiallyCovered
                    | S8MaterializationStateClass::ExactThroughPhysicalBasis
                    | S8MaterializationStateClass::Lagged
                    | S8MaterializationStateClass::Stale
                    | S8MaterializationStateClass::RebuildRequired
                    | S8MaterializationStateClass::Migrating
                    | S8MaterializationStateClass::Quarantined
                    | S8MaterializationStateClass::Retired
            ),
            Self::RangeScannableSnapshots => matches!(
                state,
                S8MaterializationStateClass::DeclaredOnly
                    | S8MaterializationStateClass::EmptyInitialized
                    | S8MaterializationStateClass::Building
                    | S8MaterializationStateClass::PartiallyCovered
                    | S8MaterializationStateClass::Lagged
                    | S8MaterializationStateClass::Stale
                    | S8MaterializationStateClass::RebuildRequired
                    | S8MaterializationStateClass::Migrating
                    | S8MaterializationStateClass::Quarantined
                    | S8MaterializationStateClass::Retired
            ),
        }
    }
}
