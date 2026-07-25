use crate::runtime::{
    WorthUiQueryBindingDriftDenialKind, WorthUiQueryBindingRebindReason,
    WorthUiQueryBindingRetirementReason, WorthUiQueryBindingUiRequirements,
    WorthUiQueryBindingUiRequirementsDriftFamily, WorthUiQueryLiveRebindCounters,
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
                fold_ui_requirements(&mut fold, preservation.preserved_ui_requirements());
                fold.fold_u64(preservation.preservation_receipt().canonical_identity());
            }
            WorthUiQueryLiveRebindOutcome::Rebind(rebind) => {
                fold.fold_tag(2);
                fold_rebind_reason(&mut fold, rebind.reason());
                fold_drift_families(&mut fold, rebind.drift_families());
                fold_required_surfaces(&mut fold, rebind.required_query_surfaces());
                fold_ui_requirements(&mut fold, rebind.candidate_ui_requirements());
            }
            WorthUiQueryLiveRebindOutcome::Retire(retirement) => {
                fold.fold_tag(3);
                fold_retirement_reason(&mut fold, retirement.reason());
                fold_ui_requirements(&mut fold, retirement.active_ui_requirements());
            }
            WorthUiQueryLiveRebindOutcome::Deny(denial) => {
                fold.fold_tag(4);
                fold_denial_reason(&mut fold, denial.reason());
                fold_optional_ui_requirements(&mut fold, denial.active_ui_requirements());
                fold_optional_ui_requirements(&mut fold, denial.candidate_ui_requirements());
                fold_drift_families(&mut fold, denial.drift_families());
            }
        }
    }
    fold.finish()
}

fn fold_ui_requirements(
    fold: &mut WorthUiQueryRebindDigestFold,
    ui_requirements: &WorthUiQueryBindingUiRequirements,
) {
    fold.fold_u64(ui_requirements.canonical_identity());
}

fn fold_optional_ui_requirements(
    fold: &mut WorthUiQueryRebindDigestFold,
    ui_requirements: Option<&WorthUiQueryBindingUiRequirements>,
) {
    match ui_requirements {
        Some(ui_requirements) => {
            fold.fold_tag(1);
            fold_ui_requirements(fold, ui_requirements);
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
        WorthUiQueryBindingRebindReason::QueryAuthorityChanged => 3,
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
        WorthUiQueryBindingDriftDenialKind::MissingCandidateUiRequirementsForRebind => 1,
        WorthUiQueryBindingDriftDenialKind::MissingActiveUiRequirementsForRetirement => 2,
    });
}

fn fold_drift_families(
    fold: &mut WorthUiQueryRebindDigestFold,
    families: &[WorthUiQueryBindingUiRequirementsDriftFamily],
) {
    fold.fold_usize(families.len());
    for family in families {
        fold.fold_tag(match family {
            WorthUiQueryBindingUiRequirementsDriftFamily::LifecycleDeclaration => 1,
            WorthUiQueryBindingUiRequirementsDriftFamily::AsyncResultPresentation => 2,
            WorthUiQueryBindingUiRequirementsDriftFamily::RecoveryPresentation => 3,
            WorthUiQueryBindingUiRequirementsDriftFamily::InspectionRelevance => 4,
            WorthUiQueryBindingUiRequirementsDriftFamily::ProjectionConsumption => 5,
            WorthUiQueryBindingUiRequirementsDriftFamily::DenialPresentation => 6,
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
