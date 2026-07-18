use worth_ui_inspection::{UiEvidenceExpansionOutcome, UiEvidenceRichness};

use crate::evidence::{
    preflight_evidence_expansion, UiAllocationPlanningInspectionReceipt, UiEvidenceExpansion,
    UiEvidenceRef,
};
use crate::facade::lifecycle::WorthUiFacadeLifecycleBootstrap;
use crate::facade::WorthUiApp;

pub(crate) fn expand_retained_obligation_ref(
    app: &WorthUiApp,
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
) -> UiEvidenceExpansion {
    if let Some(expansion) =
        lookup_obligation_retained_expansion(app, evidence_ref, requested_richness)
    {
        record_materialization_if_available(app.lifecycle(), &expansion);
        return expansion;
    }
    assemble_unsupported_obligation_expansion(evidence_ref, requested_richness)
}

pub(crate) fn expand_retained_allocation_planning_ref(
    app: &WorthUiApp,
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
) -> UiEvidenceExpansion {
    let current_generation = resolve_planning_generation(app, &evidence_ref);
    let evidence_ref = normalize_planning_ref(app, evidence_ref);
    if let Some(preflight) =
        preflight_evidence_expansion(current_generation, evidence_ref, requested_richness)
    {
        return preflight;
    }
    match lookup_planning_retained_receipt(app, evidence_ref) {
        Some(receipt) => {
            let expansion = receipt.expand_evidence_ref(evidence_ref, requested_richness);
            record_materialization_if_available(app.lifecycle(), &expansion);
            expansion
        }
        None => assemble_unsupported_planning_expansion(evidence_ref, requested_richness),
    }
}

fn lookup_obligation_retained_expansion(
    app: &WorthUiApp,
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
) -> Option<UiEvidenceExpansion> {
    app.retained_obligation_registry()
        .retained_selection(evidence_ref.handle().handle_digest())
        .map(|selected| selected.expand_evidence_ref(evidence_ref, requested_richness))
}

fn assemble_unsupported_obligation_expansion(
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
) -> UiEvidenceExpansion {
    UiEvidenceExpansion::new(
        evidence_ref,
        requested_richness,
        UiEvidenceExpansionOutcome::Unsupported,
        None,
        Box::new([]),
        None,
    )
}

fn resolve_planning_generation(
    app: &WorthUiApp,
    evidence_ref: &UiEvidenceRef,
) -> worth_ui_inspection::UiEvidenceAuthorityGeneration {
    app.retained_allocation_planning_registry()
        .current_generation_for(evidence_ref.handle().handle_digest())
        .unwrap_or_else(|| evidence_ref.authority_generation())
}

fn normalize_planning_ref(app: &WorthUiApp, evidence_ref: UiEvidenceRef) -> UiEvidenceRef {
    app.retained_allocation_planning_registry()
        .discarded_ref(evidence_ref)
        .unwrap_or(evidence_ref)
}

fn lookup_planning_retained_receipt(
    app: &WorthUiApp,
    evidence_ref: UiEvidenceRef,
) -> Option<UiAllocationPlanningInspectionReceipt> {
    app.retained_allocation_planning_registry()
        .retained_receipt(evidence_ref.handle().handle_digest())
}

fn assemble_unsupported_planning_expansion(
    evidence_ref: UiEvidenceRef,
    requested_richness: UiEvidenceRichness,
) -> UiEvidenceExpansion {
    UiEvidenceExpansion::new(
        evidence_ref,
        requested_richness,
        UiEvidenceExpansionOutcome::Unsupported,
        None,
        Box::new([]),
        None,
    )
}

fn record_materialization_if_available(
    lifecycle: &WorthUiFacadeLifecycleBootstrap,
    expansion: &UiEvidenceExpansion,
) {
    if expansion.outcome().is_available() {
        lifecycle.record_rich_artifact_materialization();
    }
}
