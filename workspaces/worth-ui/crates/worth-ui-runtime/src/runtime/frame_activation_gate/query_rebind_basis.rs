use crate::runtime::frame_activation_gate::digest_fold::WorthUiActivationGateDigestFold;
use crate::runtime::{
    WorthUiQueryBindingDriftDenialKind, WorthUiQueryBindingIdentity, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirementReason, WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlan,
    WorthUiQueryRebindRequiredSurface, WorthUiQuerySupportStatus,
};

pub(in crate::runtime) fn query_rebind_basis_digest(plan: &WorthUiQueryLiveRebindPlan) -> u64 {
    let mut fold = WorthUiActivationGateDigestFold::new(0x7175_6572_795f_0025);
    fold.fold_u64(plan.active_artifact_digest());
    fold.fold_u64(plan.candidate_artifact_digest());
    fold.fold_usize(plan.entries().len());
    let counters = plan.counters();
    fold.fold_usize(counters.bindings_planned());
    fold.fold_usize(counters.preserved_binding_count());
    fold.fold_usize(counters.rebound_binding_count());
    fold.fold_usize(counters.retired_binding_count());
    fold.fold_usize(counters.denied_binding_count());
    for entry in plan.entries() {
        fold_identity(&mut fold, entry.identity());
        match entry.outcome() {
            WorthUiQueryLiveRebindOutcome::Preserve(preservation) => {
                fold.fold_tag(1);
                fold_posture(&mut fold, preservation.preserved_posture());
                fold.fold_text(preservation.preservation_receipt());
            }
            WorthUiQueryLiveRebindOutcome::Rebind(rebind) => {
                fold.fold_tag(2);
                fold_rebind_reason(&mut fold, rebind.reason());
                fold_drift_families(&mut fold, rebind.drift_families());
                fold_required_surfaces(&mut fold, rebind.required_query_surfaces());
                fold_posture(&mut fold, rebind.candidate_posture());
            }
            WorthUiQueryLiveRebindOutcome::Retire(retirement) => {
                fold.fold_tag(3);
                fold_retirement_reason(&mut fold, retirement.reason());
                fold_posture(&mut fold, retirement.active_posture());
            }
            WorthUiQueryLiveRebindOutcome::Deny(denial) => {
                fold.fold_tag(4);
                fold_denial_reason(&mut fold, denial.reason());
                fold_optional_posture(&mut fold, denial.active_posture());
                fold_optional_posture(&mut fold, denial.candidate_posture());
                fold_drift_families(&mut fold, denial.drift_families());
            }
        }
    }
    fold.finish()
}

fn fold_identity(
    fold: &mut WorthUiActivationGateDigestFold,
    identity: &WorthUiQueryBindingIdentity,
) {
    fold.fold_text(identity.view_binding_id());
    fold.fold_text(identity.query_capability_digest());
    fold.fold_text(identity.query_composition_profile_digest());
    fold.fold_text(identity.result_shape_digest());
}

fn fold_posture(fold: &mut WorthUiActivationGateDigestFold, posture: &WorthUiQueryBindingPosture) {
    fold_query_support_status(fold, posture.query_support_status());
    fold.fold_text(posture.support_admission_digest());
    fold.fold_text(posture.basis_capability_digest());
    fold.fold_text(posture.live_compatibility_digest());
    fold.fold_text(posture.async_result_state_digest());
    fold.fold_text(posture.recovery_digest());
    fold.fold_text(posture.inspection_digest());
    fold.fold_text(posture.projection_consumption_digest());
    fold.fold_text(posture.denial_presentation_digest());
}

fn fold_optional_posture(
    fold: &mut WorthUiActivationGateDigestFold,
    posture: Option<&WorthUiQueryBindingPosture>,
) {
    match posture {
        Some(posture) => {
            fold.fold_tag(1);
            fold_posture(fold, posture);
        }
        None => fold.fold_tag(0),
    }
}

fn fold_query_support_status(
    fold: &mut WorthUiActivationGateDigestFold,
    status: WorthUiQuerySupportStatus,
) {
    let tag = match status {
        WorthUiQuerySupportStatus::Supported => 1,
        WorthUiQuerySupportStatus::Deferred => 2,
        WorthUiQuerySupportStatus::Unsupported => 3,
    };
    fold.fold_tag(tag);
}

fn fold_rebind_reason(
    fold: &mut WorthUiActivationGateDigestFold,
    reason: WorthUiQueryBindingRebindReason,
) {
    let tag = match reason {
        WorthUiQueryBindingRebindReason::FreshCandidateBinding => 1,
        WorthUiQueryBindingRebindReason::QueryIdentityChanged => 2,
        WorthUiQueryBindingRebindReason::QueryOwnedPostureDrift => 3,
    };
    fold.fold_tag(tag);
}

fn fold_retirement_reason(
    fold: &mut WorthUiActivationGateDigestFold,
    reason: WorthUiQueryBindingRetirementReason,
) {
    let tag = match reason {
        WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding => 1,
    };
    fold.fold_tag(tag);
}

fn fold_denial_reason(
    fold: &mut WorthUiActivationGateDigestFold,
    reason: WorthUiQueryBindingDriftDenialKind,
) {
    let tag = match reason {
        WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery => 1,
        WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted => 2,
        WorthUiQueryBindingDriftDenialKind::MissingCandidatePostureForRebind => 3,
        WorthUiQueryBindingDriftDenialKind::MissingActivePostureForRetirement => 4,
    };
    fold.fold_tag(tag);
}

fn fold_drift_families(
    fold: &mut WorthUiActivationGateDigestFold,
    families: &[WorthUiQueryBindingPostureDriftFamily],
) {
    fold.fold_usize(families.len());
    for family in families {
        let tag = match family {
            WorthUiQueryBindingPostureDriftFamily::SupportAdmission => 1,
            WorthUiQueryBindingPostureDriftFamily::BasisCapability => 2,
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility => 3,
            WorthUiQueryBindingPostureDriftFamily::AsyncResultState => 4,
            WorthUiQueryBindingPostureDriftFamily::Recovery => 5,
            WorthUiQueryBindingPostureDriftFamily::Inspection => 6,
            WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption => 7,
            WorthUiQueryBindingPostureDriftFamily::DenialPresentation => 8,
        };
        fold.fold_tag(tag);
    }
}

fn fold_required_surfaces(
    fold: &mut WorthUiActivationGateDigestFold,
    surfaces: &[WorthUiQueryRebindRequiredSurface],
) {
    fold.fold_usize(surfaces.len());
    for surface in surfaces {
        let tag = match surface {
            WorthUiQueryRebindRequiredSurface::LiveViewsAndLivePromotion => 1,
            WorthUiQueryRebindRequiredSurface::SubscriptionSelectionAndDiagnostics => 2,
            WorthUiQueryRebindRequiredSurface::BasisCapabilityLifecycle => 3,
            WorthUiQueryRebindRequiredSurface::AsyncResourcesAndResultState => 4,
            WorthUiQueryRebindRequiredSurface::Recovery => 5,
            WorthUiQueryRebindRequiredSurface::Inspection => 6,
            WorthUiQueryRebindRequiredSurface::ProjectionConsumption => 7,
            WorthUiQueryRebindRequiredSurface::ContinuationPipeline => 8,
        };
        fold.fold_tag(tag);
    }
}
