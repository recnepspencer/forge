use crate::runtime::{
    WorthUiQueryBindingDriftDenialKind, WorthUiQueryBindingPosture,
    WorthUiQueryBindingPostureDriftFamily, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirementReason, WorthUiQueryLiveRebindCounters,
    WorthUiQueryLiveRebindEntry, WorthUiQueryLiveRebindOutcome, WorthUiQueryRebindRequiredSurface,
};

pub(super) fn query_rebind_basis_digest(
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    entries: &[WorthUiQueryLiveRebindEntry],
    counters: WorthUiQueryLiveRebindCounters,
) -> u64 {
    let mut fold = WorthUiQueryRebindDigestFold::new(0x7175_6572_795f_0025);
    fold.fold_u64(active_artifact_digest);
    fold.fold_u64(candidate_artifact_digest);
    fold.fold_usize(entries.len());
    fold.fold_usize(counters.bindings_planned());
    fold.fold_usize(counters.preserved_binding_count());
    fold.fold_usize(counters.rebound_binding_count());
    fold.fold_usize(counters.retired_binding_count());
    fold.fold_usize(counters.denied_binding_count());
    for entry in entries {
        fold.fold_u64(entry.identity().canonical_identity());
        match entry.outcome() {
            WorthUiQueryLiveRebindOutcome::Preserve(preservation) => {
                fold.fold_tag(1);
                fold_posture(&mut fold, preservation.preserved_posture());
                fold.fold_u64(preservation.preservation_receipt().canonical_identity());
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

fn fold_posture(fold: &mut WorthUiQueryRebindDigestFold, posture: &WorthUiQueryBindingPosture) {
    fold.fold_u64(posture.canonical_identity());
}

fn fold_optional_posture(
    fold: &mut WorthUiQueryRebindDigestFold,
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

fn fold_rebind_reason(
    fold: &mut WorthUiQueryRebindDigestFold,
    reason: WorthUiQueryBindingRebindReason,
) {
    fold.fold_tag(match reason {
        WorthUiQueryBindingRebindReason::FreshCandidateBinding => 1,
        WorthUiQueryBindingRebindReason::QueryIdentityChanged => 2,
        WorthUiQueryBindingRebindReason::QueryOwnedPostureDrift => 3,
    });
}

fn fold_retirement_reason(
    fold: &mut WorthUiQueryRebindDigestFold,
    reason: WorthUiQueryBindingRetirementReason,
) {
    fold.fold_tag(match reason {
        WorthUiQueryBindingRetirementReason::CandidateRemovedQueryBinding => 1,
    });
}

fn fold_denial_reason(
    fold: &mut WorthUiQueryRebindDigestFold,
    reason: WorthUiQueryBindingDriftDenialKind,
) {
    fold.fold_tag(match reason {
        WorthUiQueryBindingDriftDenialKind::UiLocalDenialPresentationWouldReplaceQueryRecovery => 1,
        WorthUiQueryBindingDriftDenialKind::QuerySupportPostureNotAdmitted => 2,
        WorthUiQueryBindingDriftDenialKind::MissingCandidatePostureForRebind => 3,
        WorthUiQueryBindingDriftDenialKind::MissingActivePostureForRetirement => 4,
    });
}

fn fold_drift_families(
    fold: &mut WorthUiQueryRebindDigestFold,
    families: &[WorthUiQueryBindingPostureDriftFamily],
) {
    fold.fold_usize(families.len());
    for family in families {
        fold.fold_tag(match family {
            WorthUiQueryBindingPostureDriftFamily::SupportAdmission => 1,
            WorthUiQueryBindingPostureDriftFamily::BasisCapability => 2,
            WorthUiQueryBindingPostureDriftFamily::LiveCompatibility => 3,
            WorthUiQueryBindingPostureDriftFamily::AsyncResultState => 4,
            WorthUiQueryBindingPostureDriftFamily::Recovery => 5,
            WorthUiQueryBindingPostureDriftFamily::Inspection => 6,
            WorthUiQueryBindingPostureDriftFamily::ProjectionConsumption => 7,
            WorthUiQueryBindingPostureDriftFamily::DenialPresentation => 8,
        });
    }
}

fn fold_required_surfaces(
    fold: &mut WorthUiQueryRebindDigestFold,
    surfaces: &[WorthUiQueryRebindRequiredSurface],
) {
    fold.fold_usize(surfaces.len());
    for surface in surfaces {
        fold.fold_tag(match surface {
            WorthUiQueryRebindRequiredSurface::LiveViewsAndLivePromotion => 1,
            WorthUiQueryRebindRequiredSurface::SubscriptionSelectionAndDiagnostics => 2,
            WorthUiQueryRebindRequiredSurface::BasisCapabilityLifecycle => 3,
            WorthUiQueryRebindRequiredSurface::AsyncResourcesAndResultState => 4,
            WorthUiQueryRebindRequiredSurface::Recovery => 5,
            WorthUiQueryRebindRequiredSurface::Inspection => 6,
            WorthUiQueryRebindRequiredSurface::ProjectionConsumption => 7,
            WorthUiQueryRebindRequiredSurface::ContinuationPipeline => 8,
        });
    }
}

struct WorthUiQueryRebindDigestFold {
    value: u64,
}

impl WorthUiQueryRebindDigestFold {
    fn new(seed: u64) -> Self {
        Self { value: seed }
    }

    fn fold_u64(&mut self, value: u64) {
        self.value ^= value.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        self.value = self.value.rotate_left(13);
    }

    fn fold_usize(&mut self, value: usize) {
        self.fold_u64(value as u64);
    }

    fn fold_tag(&mut self, tag: u64) {
        self.fold_u64(tag);
    }

    fn finish(self) -> u64 {
        self.value ^ 0xa47f_2b19_63d5_81ceu64
    }
}
