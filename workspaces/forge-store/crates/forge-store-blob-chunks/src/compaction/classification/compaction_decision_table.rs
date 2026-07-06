use super::compaction_eligibility_case::CompactionEligibilityCase;
use crate::compaction::types::BlobCompactionIntent;
use crate::compaction::verification::{
    cold_readiness, dedupe_edges, lifecycle_placement, lifecycle_reachability, physical_interlock,
    quarantine_absent, reachability_present, read_hold_matches_physical, s6_pacing,
    uncompacted_publication,
};
use crate::{BlobCompactionCounterSnapshot, BlobCompactionDenial};

pub(crate) fn classify_compaction_eligibility(
    intent: &BlobCompactionIntent,
) -> CompactionEligibilityCase {
    if let Some(case) = physical_interlock::require_physical_interlock(intent) {
        return case;
    }
    if let Some(case) = reachability_present::require_reachability_present(intent) {
        return case;
    }
    if intent.read_hold().is_active() {
        return CompactionEligibilityCase::ActiveReadHold;
    }
    let physical = intent
        .physical()
        .admitted()
        .expect("admitted physical interlock should be present after classification");
    if let Some(case) =
        read_hold_matches_physical::require_read_hold_matches_physical(intent.read_hold(), physical)
    {
        return case;
    }
    if let Some(case) = s6_pacing::require_s6_pacing(intent) {
        return case;
    }
    if let Some(case) = cold_readiness::require_cold_readiness(intent) {
        return case;
    }
    if let Some(case) = quarantine_absent::require_quarantine_absent(intent) {
        return case;
    }
    if let Some(case) = uncompacted_publication::require_uncompacted_publication(
        intent.lifecycle(),
        intent.uncompacted_publication(),
    ) {
        return case;
    }
    let reachability = intent
        .reachability()
        .expect("reachability should be present after classification");
    if let Some(case) =
        lifecycle_reachability::require_lifecycle_reachability(intent.lifecycle(), reachability)
    {
        return case;
    }
    if let Some(case) =
        lifecycle_placement::require_lifecycle_placement(intent.lifecycle(), intent.placement())
    {
        return case;
    }
    if let Some(case) =
        dedupe_edges::require_dedupe_edges(intent.dedupe_references(), reachability)
    {
        return case;
    }
    CompactionEligibilityCase::Admit
}

pub(crate) fn assemble_compaction_denial(
    case: CompactionEligibilityCase,
    intent: &BlobCompactionIntent,
    counters: BlobCompactionCounterSnapshot,
) -> BlobCompactionDenial {
    let denial_counters = counters.record_denial();
    match case {
        CompactionEligibilityCase::Admit => {
            unreachable!("eligible compaction intent does not assemble denials")
        }
        CompactionEligibilityCase::PhysicalInterlockDenied => {
            let source = intent
                .physical()
                .denial()
                .expect("physical interlock denial should carry denial source");
            BlobCompactionDenial::PhysicalInterlockDenied {
                source,
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::MissingReachabilityProof => {
            BlobCompactionDenial::MissingReachabilityProof {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::ActiveReadHold => {
            BlobCompactionDenial::ActiveReadHold {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::ReadHoldPlanMismatch => {
            BlobCompactionDenial::ReadHoldPlanMismatch {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::UnsupportedS6Pacing => {
            BlobCompactionDenial::UnsupportedS6Pacing {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::UnavailableColdChunk => {
            BlobCompactionDenial::UnavailableColdChunk {
                state: intent.cold().state(),
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::QuarantineHold => {
            BlobCompactionDenial::QuarantineHold {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::EquivalenceBasisMismatch => {
            BlobCompactionDenial::EquivalenceBasisMismatch {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::LifecycleReachabilityMismatch => {
            BlobCompactionDenial::LifecycleReachabilityMismatch {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::LifecyclePlacementMismatch => {
            BlobCompactionDenial::LifecyclePlacementMismatch {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::DedupeScopeMismatch => {
            BlobCompactionDenial::DedupeScopeMismatch {
                counters: denial_counters,
            }
        }
        CompactionEligibilityCase::StaleDedupeReference => {
            BlobCompactionDenial::StaleDedupeReference {
                counters: denial_counters,
            }
        }
    }
}