use crate::runtime::replacement::compatibility::managed_live::denial::WorthUiQueryBindingDriftDenialReason;
use crate::runtime::replacement::compatibility::managed_live::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingPreservation, WorthUiQueryBindingRebind,
    WorthUiQueryBindingRebindReason, WorthUiQueryBindingRetirement,
    WorthUiQueryBindingRetirementReason, WorthUiQueryLiveRebindEntry,
    WorthUiQueryLiveRebindOutcome,
};
use crate::runtime::replacement::query_binding::{
    WorthUiQueryBindingComparisonEntry, WorthUiQueryBindingComparisonOutcome,
};

pub(super) fn decide_entry(
    comparison: &WorthUiQueryBindingComparisonEntry,
) -> WorthUiQueryLiveRebindEntry {
    let identity = comparison.identity().clone();
    let outcome = match comparison.outcome() {
        WorthUiQueryBindingComparisonOutcome::PreserveMeaning => preserve_or_deny(comparison),
        WorthUiQueryBindingComparisonOutcome::RebindRequired => {
            let reason = if comparison.has_query_authority_drift() {
                WorthUiQueryBindingRebindReason::QueryAuthorityChanged
            } else {
                WorthUiQueryBindingRebindReason::QueryIdentityChanged
            };
            rebind_or_deny(comparison, reason)
        }
        WorthUiQueryBindingComparisonOutcome::MissingActiveBinding => rebind_or_deny(
            comparison,
            WorthUiQueryBindingRebindReason::FreshCandidateBinding,
        ),
        WorthUiQueryBindingComparisonOutcome::MissingCandidateBinding => retire_or_deny(comparison),
    };
    WorthUiQueryLiveRebindEntry::new(identity, outcome)
}

fn preserve_or_deny(
    comparison: &WorthUiQueryBindingComparisonEntry,
) -> WorthUiQueryLiveRebindOutcome {
    let Some(candidate_ui_requirements) = comparison.candidate_ui_requirements() else {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::MissingCandidateUiRequirementsForRebind,
        );
    };
    WorthUiQueryLiveRebindOutcome::Preserve(WorthUiQueryBindingPreservation::new(
        comparison.identity().clone(),
        candidate_ui_requirements.clone(),
    ))
}

fn rebind_or_deny(
    comparison: &WorthUiQueryBindingComparisonEntry,
    reason: WorthUiQueryBindingRebindReason,
) -> WorthUiQueryLiveRebindOutcome {
    let Some(candidate_ui_requirements) = comparison.candidate_ui_requirements() else {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::MissingCandidateUiRequirementsForRebind,
        );
    };
    WorthUiQueryLiveRebindOutcome::Rebind(WorthUiQueryBindingRebind::new(
        comparison.identity().clone(),
        candidate_ui_requirements.clone(),
        reason,
        comparison.ui_requirement_drifts().to_vec(),
    ))
}

fn retire_or_deny(
    comparison: &WorthUiQueryBindingComparisonEntry,
) -> WorthUiQueryLiveRebindOutcome {
    let Some(active_ui_requirements) = comparison.active_ui_requirements() else {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::MissingActiveUiRequirementsForRetirement,
        );
    };
    WorthUiQueryLiveRebindOutcome::Retire(WorthUiQueryBindingRetirement::new(
        comparison.identity().clone(),
        active_ui_requirements.clone(),
        WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding,
    ))
}

fn deny(
    comparison: &WorthUiQueryBindingComparisonEntry,
    reason: WorthUiQueryBindingDriftDenialReason,
) -> WorthUiQueryLiveRebindOutcome {
    WorthUiQueryLiveRebindOutcome::Deny(WorthUiQueryBindingDriftDenial::new(
        comparison.identity().clone(),
        comparison.active_ui_requirements().cloned(),
        comparison.candidate_ui_requirements().cloned(),
        comparison.ui_requirement_drifts().to_vec(),
        reason,
    ))
}
