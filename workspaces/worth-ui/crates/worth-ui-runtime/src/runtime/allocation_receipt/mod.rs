//! Allocation truth progression: candidate -> preview or committed receipt.

mod candidate;
mod commit_outcome;
mod committed_allocation;
mod committed_catalog_binding;
mod committed_evidence;
mod committed_lowering_input;
mod committed_receipt;
mod denial;
mod denial_report;
mod durable_semantic_state;
mod equivalence_basis;
mod geometry_evidence;
mod ledger_denial;
mod ledger_state;
mod prepared_portal_commit;
mod prepared_replan;
mod preview_candidate;
mod preview_isolation;
mod receipt_budget;
mod receipt_commit;
mod receipt_generation;
mod receipt_identity;
mod receipt_ledger;
mod receipt_ledger_entry;
#[cfg(test)]
mod receipt_ledger_test_support;
#[cfg(test)]
pub(crate) use receipt_ledger_test_support::{
    detached_non_portal_receipt, UiNonPortalReceiptLawCandidate,
};
mod receipt_report;
mod replan_commit_mode;
mod replan_transaction;
mod reuse_verdict;
mod transaction_outcome;
mod truth_revision;
mod viewport_inspection;

pub use candidate::UiAllocationCandidate;
pub use commit_outcome::UiAllocationReceiptCommitOutcome;
pub(crate) use committed_allocation::UiCommittedAllocation;
pub(crate) use committed_catalog_binding::UiCommittedAllocationCatalogActivation;
pub use committed_catalog_binding::UiCommittedAllocationCatalogActivationDenial;
pub(crate) use committed_catalog_binding::UiCommittedAllocationCatalogBindings;
pub(crate) use committed_catalog_binding::UiCommittedPortalActivationSource;
pub(crate) use committed_catalog_binding::UiCommittedScrollActivationSource;
pub use committed_evidence::{UiCommittedAllocationEvidenceSet, UiCommittedPortalAnchorEvidence};
pub use committed_lowering_input::UiCommittedAllocationLoweringInput;
pub use committed_receipt::UiAllocationReceipt;
pub use denial::UiAllocationReceiptCommitDenial;
pub use denial_report::{UiAllocationReceiptDenialCause, UiAllocationReceiptDenialReport};
pub use durable_semantic_state::UiAllocationDurableSemanticState;
pub use equivalence_basis::{
    UiAllocationConstraintPayloadShape, UiAllocationConstraintPropagationShape,
    UiAllocationReceiptConstraintShape, UiAllocationReceiptEquivalenceBasis,
};
pub use geometry_evidence::{
    UiAllocationAnchorPosture, UiAllocationAxis, UiAllocationAxisAlignedBounds,
    UiAllocationEdgeReference, UiAllocationGeometryKnowledge,
    UiCommittedAllocationGeometryEvidence, UiPortalAnchorObservationGeometryEvidence,
};
pub use prepared_portal_commit::UiPortalAllocationCommitBindDenial;
pub use preview_candidate::UiAllocationPreviewCandidate;
pub use preview_isolation::{
    UiPreviewPaintIsolationOutcome, UiPreviewPaintIsolationReceipt,
    UiPreviewPaintIsolationViolation,
};
pub use receipt_generation::UiAllocationReceiptGeneration;
pub use receipt_identity::UiAllocationReceiptIdentity;
pub use receipt_report::{UiAllocationReceiptFreshnessPosture, UiAllocationReceiptReport};
pub use replan_transaction::{UiAllocationReplanTransaction, UiAllocationReplanTransactionDenial};
pub use reuse_verdict::{
    UiAllocationLeafRemeasureWitness, UiAllocationReuseDenial, UiAllocationReuseVerdict,
};
pub use transaction_outcome::{
    UiAllocationReplanTransactionCommitDenial, UiAllocationReplanTransactionCounters,
    UiAllocationReplanTransactionOutcome, UiCommittedAllocationReplan,
};
pub use truth_revision::{
    UiAllocationAuthorityCounter, UiAllocationAuthorityCounterExhaustion, UiAllocationTruthDelta,
    UiAllocationTruthRevision,
};

pub(crate) use ledger_state::UiAllocationCatalogLedgerLineage;
pub(crate) use ledger_state::UiAllocationCatalogLedgerTransition;
pub(in crate::runtime) use prepared_replan::{
    UiAllocationLedgerPreparation, UiPreparedAllocationLedgerTransition,
};
pub(crate) use preview_isolation::UiPreviewPaintIsolationPort;
pub(crate) use receipt_commit::project_allocation_preview;
pub(crate) use receipt_ledger::UiAllocationReceiptLedger;
pub(in crate::runtime) use receipt_ledger_entry::UiPreparedAllocationCatalogLedgerCommit;
mod activation_catalog_commit;
