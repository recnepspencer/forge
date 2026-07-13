//! Compaction interlock verdicts must consume executed lower evidence.
//!
//! A compaction read verdict cannot be minted from a plan alone:
//!
//! ```compile_fail
//! let _shortcut =
//!     forge_store_physical_isolation::execute_admitted_compaction_rewrite_for_plan;
//! ```

mod candidate_ranges;
mod counters;
mod cutover_delta;
mod denial;
mod foundational_evidence;
mod mutation_lane_receipt;
mod owner_case;
mod owner_inventory;
mod plan;
mod plan_identity;
mod protected_set;
mod publication;
mod reclaim_queue;
mod scheduler_demand;
mod stability_proof;
#[cfg(test)]
mod state_machine_contract_tests;
#[cfg(any(test, feature = "certification-authority"))]
mod test_authority;
mod verdict;

pub use candidate_ranges::CompactionCandidateRangeSet;
pub use counters::CompactionReadInterlockCounters;
pub use cutover_delta::CompactionCutoverDelta;
pub use denial::CompactionReadInterlockDenial;
pub use foundational_evidence::CompactionInterlockFoundationalEvidence;
pub use mutation_lane_receipt::{
    CompactionMutationLaneOrigin, CompactionMutationLaneReceipt, CompactionMutationLaneReceiptKind,
};
pub use owner_case::{CompactionCutoverState, CompactionOwnerCase, CompactionOwnerCaseId};
pub use owner_inventory::compaction_owner_case_inventory;
#[cfg(any(test, feature = "certification-authority"))]
pub use plan::compaction_read_interlock_plan_for_certification_root_seed;
#[cfg(any(test, feature = "certification-authority"))]
pub use plan::compaction_read_interlock_plan_for_certification_test;
pub use plan::{CompactionReadInterlockPlan, CompactionSourceIntegrityEvidence};
pub use protected_set::CompactionProtectedReferenceSet;
#[cfg(any(test, feature = "certification-authority"))]
pub use publication::publish_compaction_rewrite_for_certification;
pub use publication::CompactionRewritePublication;
pub use reclaim_queue::{CompactionDeferredReclaimQueue, DrainedCompactionReclaim};
pub use scheduler_demand::compaction_rewrite_scheduler_demand;
pub use stability_proof::CompactionCutoverStabilityProof;
#[cfg(any(test, feature = "certification-authority"))]
pub use test_authority::{
    compaction_cutover_evidence_for_certification_plan,
    compaction_cutover_evidence_for_certification_rewrite_manifest,
    CompactionCutoverEvidenceForCertification,
};
pub use verdict::{execute_read_during_compaction_cutover, ReadDuringCompactionVerdict};
