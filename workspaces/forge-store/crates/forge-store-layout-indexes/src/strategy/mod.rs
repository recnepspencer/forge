mod admission;
mod admitted_strategy;
pub(crate) mod btree;
mod capability;
mod capability_queries;
mod counter_evidence;
mod counter_path;
mod counter_planning;
mod declaration;
mod denial;
mod family;
mod invariant_suite;
mod key_law_validation;
mod lsm;
mod materialization_policy;
mod posture;
#[cfg(test)]
pub(crate) mod tests;
#[cfg(test)]
pub(crate) mod tests_support;

pub(crate) use admission::admit_strategy;
pub use admitted_strategy::S8AdmittedLayoutStrategy;
pub use btree::{
    S8BTreeCorruptionRegion, S8BTreeInvariantSuite, S8BTreeLookupBranch, S8BTreeNodeFormatLaw,
    S8BTreeRebuildMigrationLaw, S8BTreeRootPublicationLaw, S8BTreeSearchOutcome,
    S8BTreeSearchOutcomeView, S8BTreeSearchPathLaw, S8BTreeSeparatorLaw, S8BTreeSiblingLinkLaw,
    S8BTreeSplitMergeLaw, S8BTreeStableReadLaw, S8BTreeTombstoneLaw,
};
pub use counter_evidence::S8StrategyCounterEvidence;
pub(crate) use counter_planning::planned_counter_envelope_for;
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
    baseline_lsm_manifest_artifact_bytes, baseline_lsm_output_artifact_bytes,
    baseline_lsm_record_artifact_bytes, lsm_strategy, BaselineLsmAdmittedKey,
    BaselineLsmAdmittedRecord, BaselineLsmCompactionKeyIdentity, BaselineLsmCompactionPlan,
    BaselineLsmCompactionPublicationReceipt, BaselineLsmCompactionRecordIdentity,
    BaselineLsmCompactionRecordKind, BaselineLsmCompactionTransition,
    BaselineLsmCounterObservation, BaselineLsmDurableInputs, BaselineLsmExecutionAdmissionDenial,
    BaselineLsmExecutionIntent, BaselineLsmExecutionWitness, BaselineLsmLookupExecution,
    BaselineLsmManifestPublicationExecution, BaselineLsmMembershipObservation,
    BaselineLsmPhysicalPublicationBinding, BaselineLsmReplayExecution, BaselineLsmRunIdentity,
    BaselineLsmWalIndexSession, LsmStrategy, S8LsmAdvisoryFilterLaw, S8LsmInvariantSuite,
    S8LsmLookupDisposition, S8LsmMemtableWalLaw, S8LsmRunPublicationLaw, S8LsmStaleRunCleanupLaw,
    S8LsmTombstoneLaw, S8LsmWriteAmplificationLaw,
};
pub use posture::{
    S8StrategyAmplificationProfile, S8StrategyCorruptionIsolationBehavior,
    S8StrategyLocalityProfile, S8StrategyMaterializationPosture,
    S8StrategyRebuildSourceRequirement,
};
