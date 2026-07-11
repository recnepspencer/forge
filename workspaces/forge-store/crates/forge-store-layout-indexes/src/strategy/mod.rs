mod admission;
pub(crate) mod btree;
mod capability;
mod counter_evidence;
mod counter_path;
mod declaration;
mod denial;
mod family;
mod invariant_suite;
mod lsm;
mod posture;
#[cfg(test)]
pub(crate) mod tests;
#[cfg(test)]
pub(crate) mod tests_support;

pub use admission::S8AdmittedLayoutStrategy;
pub(crate) use admission::{admit_strategy, planned_counter_envelope_for};
pub use btree::{
    S8BTreeCorruptionRegion, S8BTreeInvariantSuite, S8BTreeLookupBranch, S8BTreeNodeFormatLaw,
    S8BTreeRebuildMigrationLaw, S8BTreeRootPublicationLaw, S8BTreeSearchOutcome,
    S8BTreeSearchOutcomeView, S8BTreeSearchPathLaw, S8BTreeSeparatorLaw, S8BTreeSiblingLinkLaw,
    S8BTreeSplitMergeLaw, S8BTreeStableReadLaw, S8BTreeTombstoneLaw,
};
pub use counter_evidence::S8StrategyCounterEvidence;
pub(crate) use declaration::S8StrategyDeclaration;
pub use denial::S8StrategyDenial;
pub use family::S8LayoutStrategyFamily;
pub(crate) use invariant_suite::S8StrategyInvariantAdmissionOutcome;
pub use invariant_suite::{
    S8StrategyCounterProfile, S8StrategyIntegrityInvariant, S8StrategyInvariantSuite,
    S8StrategyLookupInvariant, S8StrategyMutationInvariant, S8StrategyPublicationInvariant,
    S8StrategyRecoveryInvariant,
};
pub use lsm::{
    S8LsmAdvisoryFilterLaw, S8LsmInvariantSuite, S8LsmLookupDisposition, S8LsmMemtableWalLaw,
    S8LsmRunPublicationLaw, S8LsmStaleRunCleanupLaw, S8LsmTombstoneLaw, S8LsmWriteAmplificationLaw,
};
pub use posture::{
    S8StrategyAmplificationProfile, S8StrategyCorruptionIsolationBehavior,
    S8StrategyLocalityProfile, S8StrategyMaterializationPosture,
    S8StrategyRebuildSourceRequirement,
};
