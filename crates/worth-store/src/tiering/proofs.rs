#![allow(dead_code)]

mod candidates;
mod classification;
mod coalescing;
mod observation;
mod placement_evidence;
mod recall_completion;
mod recall_eligibility;
mod recall_path;
mod residency;
mod residency_manifest;
mod transfer_lifecycle;

#[cfg(test)]
mod tests;

use super::PlacementObservationScopeClass;

pub use candidates::{TierCoolingCandidate, TierPromotionCandidate};
pub use classification::{
    HotnessClassificationVerdict, PlacementBudgetClass, PlacementExecutionOrigin,
    RecallAmplificationBudget, RecallCostClass, TierResidenceClass,
};
pub use coalescing::RecallCoalescingKey;
pub use observation::{PlacementDemandSummary, WorkingSetObservationWindow};
pub use placement_evidence::{PlacementNonAuthorityWitness, TierPlacementEvidence};
pub use recall_completion::RecallCompletionWitness;
pub use recall_eligibility::RecallEligibilityWitness;
pub use recall_path::{ColdRecallTierPath, RetainedReadPlacementPath, TierMissOutcome};
pub use residency::{AuthoritativeTierResidency, DerivedTierResidency};
pub use residency_manifest::CanonicalResidencyManifest;
pub use transfer_lifecycle::{
    RetiredTierReplica, TierCutoverWitness, TierTransferIntent, TransferredTierReplica,
    VerifiedTierReplica,
};
pub use worth_store_contracts::PlacementArtifactFamily;
