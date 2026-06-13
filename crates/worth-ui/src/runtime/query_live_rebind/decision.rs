use crate::runtime::query_binding::{
    WorthUiQueryBindingComparisonEntry, WorthUiQueryBindingComparisonOutcome,
    WorthUiQueryBindingPostureDriftFamily,
};
use crate::runtime::query_live_rebind::denial::WorthUiQueryBindingDriftDenialReason;
use crate::runtime::query_live_rebind::{
    WorthUiQueryBindingDriftDenial, WorthUiQueryBindingPreservation, WorthUiQueryBindingRebind,
    WorthUiQueryBindingRebindReason, WorthUiQueryBindingRetirement,
    WorthUiQueryBindingRetirementReason, WorthUiQueryLiveRebindEntry,
    WorthUiQueryLiveRebindOutcome,
};
use crate::runtime::WorthUiQuerySupportStatus;

pub(super) fn decide_entry(
    comparison: &WorthUiQueryBindingComparisonEntry,
) -> WorthUiQueryLiveRebindEntry {
    let identity = comparison.identity().clone();
    let outcome = match comparison.outcome() {
        WorthUiQueryBindingComparisonOutcome::PreserveMeaning => preserve_or_deny(comparison),
        WorthUiQueryBindingComparisonOutcome::RebindRequired => {
            let reason = if comparison.posture_drifts().is_empty() {
                WorthUiQueryBindingRebindReason::QueryIdentityChanged
            } else {
                WorthUiQueryBindingRebindReason::QueryOwnedPostureDrift
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
    let Some(candidate_posture) = comparison.candidate_posture() else {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::MissingCandidatePostureForRebind,
        );
    };
    if candidate_posture.query_support_status() != WorthUiQuerySupportStatus::Supported {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::QuerySupportPostureNotAdmitted,
        );
    }
    WorthUiQueryLiveRebindOutcome::Preserve(WorthUiQueryBindingPreservation::new(
        comparison.identity().clone(),
        candidate_posture.clone(),
    ))
}

fn rebind_or_deny(
    comparison: &WorthUiQueryBindingComparisonEntry,
    reason: WorthUiQueryBindingRebindReason,
) -> WorthUiQueryLiveRebindOutcome {
    if comparison
        .posture_drifts()
        .contains(&WorthUiQueryBindingPostureDriftFamily::DenialPresentation)
    {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::UiLocalDenialPresentationWouldReplaceQueryRecovery,
        );
    }
    let Some(candidate_posture) = comparison.candidate_posture() else {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::MissingCandidatePostureForRebind,
        );
    };
    if candidate_posture.query_support_status() != WorthUiQuerySupportStatus::Supported {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::QuerySupportPostureNotAdmitted,
        );
    }
    WorthUiQueryLiveRebindOutcome::Rebind(WorthUiQueryBindingRebind::new(
        comparison.identity().clone(),
        candidate_posture.clone(),
        reason,
        comparison.posture_drifts().to_vec(),
    ))
}

fn retire_or_deny(
    comparison: &WorthUiQueryBindingComparisonEntry,
) -> WorthUiQueryLiveRebindOutcome {
    let Some(active_posture) = comparison.active_posture() else {
        return deny(
            comparison,
            WorthUiQueryBindingDriftDenialReason::MissingActivePostureForRetirement,
        );
    };
    WorthUiQueryLiveRebindOutcome::Retire(WorthUiQueryBindingRetirement::new(
        comparison.identity().clone(),
        active_posture.clone(),
        WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding,
    ))
}

fn deny(
    comparison: &WorthUiQueryBindingComparisonEntry,
    reason: WorthUiQueryBindingDriftDenialReason,
) -> WorthUiQueryLiveRebindOutcome {
    WorthUiQueryLiveRebindOutcome::Deny(WorthUiQueryBindingDriftDenial::new(
        comparison.identity().clone(),
        comparison.active_posture().cloned(),
        comparison.candidate_posture().cloned(),
        comparison.posture_drifts().to_vec(),
        reason,
    ))
}
