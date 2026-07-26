use worth_ui::facade::admission::WorthUiAdmissionExt;
use worth_ui_certification::scenario::obligation_dispatch_prerequisite as obligation_dispatch_prerequisite_support;

use worth_ui::facade::admission::{
    UiAdmissionAggregation, UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionWorld,
};
use worth_ui::facade::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};
use worth_ui::facade::obligations::{
    UiAdmissionAuthorityHandoff, UiObligationCheckKind, UiObligationCloseoutReport,
    UiObligationDispatchStopPosture, UiObligationEvidenceHandle, UiObligationFamily,
    UiObligationSelectionHandoff, UiObligationSelectionReason, UiObligationSupportSelectionPosture,
    UiObligationVerdictClass,
};
use worth_ui_inspection::UiInspectionSupportPosture;
use worth_ui_runtime::facade::admission::{UiAdmissionReport, UiLegalityPosture};
use worth_ui_runtime::facade::obligations::{
    UiObligationDispatchPlan, UiObligationPrerequisiteEvidenceRef, UiSelectedObligationSet,
};

use self::obligation_dispatch_prerequisite_support::{
    admission_targets::available_host_capability_target, application_authority::service_touch_app,
    dispatch_execution::execute_for_target, graph_touches::service_touch,
};

#[derive(Debug, Eq, PartialEq)]
struct LaterSliceSelectionSummary {
    touch_digest: u64,
    target_node: UiGraphNodeIdentity,
    touch_world: UiGraphWorldProfile,
    support_posture: UiInspectionSupportPosture,
    query_basis: UiAdmissionQueryBasis,
    host_capability: UiAdmissionHostCapability,
    obligations: Vec<SelectionObligationSummary>,
    evidence_handles: Vec<UiObligationEvidenceHandle>,
    evidence_row_count: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct SelectionObligationSummary {
    family: UiObligationFamily,
    check_kind: UiObligationCheckKind,
    support_posture: UiObligationSupportSelectionPosture,
    world_profile: UiGraphWorldProfile,
    selection_reasons: Vec<UiObligationSelectionReason>,
    prerequisite_evidence_refs: Vec<UiObligationPrerequisiteEvidenceRef>,
    evidence_handle: UiObligationEvidenceHandle,
}

#[derive(Debug, Eq, PartialEq)]
struct LaterSliceAdmissionSummary {
    target_node: UiGraphNodeIdentity,
    target_world: UiAdmissionWorld,
    support_posture: UiInspectionSupportPosture,
    aggregation: UiAdmissionAggregation,
    query_basis: UiAdmissionQueryBasis,
    host_capability: UiAdmissionHostCapability,
    legality_posture: Option<UiLegalityPosture>,
    dispatch_plan: Option<DispatchPlanSummary>,
    verdicts: Vec<VerdictSummary>,
    verdict_handles: Vec<UiObligationEvidenceHandle>,
    evidence_row_count: usize,
    closeout_report: UiObligationCloseoutReport,
}

#[derive(Debug, Eq, PartialEq)]
struct DispatchPlanSummary {
    entry_count: usize,
    plan_stop_posture: UiObligationDispatchStopPosture,
    shape_digest: u64,
}

#[derive(Debug, Eq, PartialEq)]
struct VerdictSummary {
    family: Option<UiObligationFamily>,
    check_kind: Option<UiObligationCheckKind>,
    selected_identity_digest: Option<u64>,
    class: UiObligationVerdictClass,
    stop_posture: UiObligationDispatchStopPosture,
    selection_reasons: Vec<UiObligationSelectionReason>,
    prerequisite_evidence_refs: Vec<UiObligationPrerequisiteEvidenceRef>,
    evidence_handle: UiObligationEvidenceHandle,
}

#[test]
fn later_runtime_slices_consume_sealed_selection_and_admission_authority() {
    let app = service_touch_app();
    let touch = service_touch(&app);
    let target = available_host_capability_target(&touch);
    let selected = app
        .admission()
        .select_obligations_for_target(&touch, target.clone());
    let report = app.admission().admit_selected_obligations(&selected);
    let dispatch_bundle = execute_for_target(&app, &touch, target);

    let selection_handoff = selected.handoff();
    let admission_handoff = report.handoff();

    assert_eq!(
        summarize_selection_handoff(selection_handoff),
        summarize_selected_truth(&selected)
    );
    assert_eq!(
        summarize_admission_handoff(admission_handoff),
        summarize_admission_truth(&report)
    );
    assert_eq!(
        admission_handoff.verdicts(),
        dispatch_bundle.verdicts.as_ref()
    );
}

fn summarize_selection_handoff(
    handoff: UiObligationSelectionHandoff<'_>,
) -> LaterSliceSelectionSummary {
    LaterSliceSelectionSummary {
        touch_digest: handoff.touch().identity_digest(),
        target_node: handoff.support_snapshot().target().graph_node_identity(),
        touch_world: handoff.touch().world().world_profile().clone(),
        support_posture: handoff.support_snapshot().inspection_posture(),
        query_basis: handoff.support_snapshot().target().query_basis(),
        host_capability: handoff.support_snapshot().target().host_capability(),
        obligations: handoff
            .obligations()
            .iter()
            .map(|obligation| SelectionObligationSummary {
                family: obligation.family(),
                check_kind: obligation.check_kind(),
                support_posture: obligation.support_posture(),
                world_profile: obligation.identity().world_profile().clone(),
                selection_reasons: obligation.selection_reasons().to_vec(),
                prerequisite_evidence_refs: obligation.prerequisite_evidence_refs().to_vec(),
                evidence_handle: obligation.evidence_handle(),
            })
            .collect(),
        evidence_handles: handoff.selected_obligation_handles().into_vec(),
        evidence_row_count: handoff.evidence_index().records().len(),
    }
}

fn summarize_selected_truth(selected: &UiSelectedObligationSet) -> LaterSliceSelectionSummary {
    LaterSliceSelectionSummary {
        touch_digest: selected.touch().identity_digest(),
        target_node: selected.support_snapshot().target().graph_node_identity(),
        touch_world: selected.touch().world().world_profile().clone(),
        support_posture: selected.support_snapshot().inspection_posture(),
        query_basis: selected.support_snapshot().target().query_basis(),
        host_capability: selected.support_snapshot().target().host_capability(),
        obligations: selected
            .obligations()
            .iter()
            .map(|obligation| SelectionObligationSummary {
                family: obligation.family(),
                check_kind: obligation.check_kind(),
                support_posture: obligation.support_posture(),
                world_profile: obligation.identity().world_profile().clone(),
                selection_reasons: obligation.selection_reasons().to_vec(),
                prerequisite_evidence_refs: obligation.prerequisite_evidence_refs().to_vec(),
                evidence_handle: obligation.evidence_handle(),
            })
            .collect(),
        evidence_handles: selected.selected_obligation_handles().into_vec(),
        evidence_row_count: selected.evidence_index().records().len(),
    }
}

fn summarize_admission_handoff(
    handoff: UiAdmissionAuthorityHandoff<'_>,
) -> LaterSliceAdmissionSummary {
    LaterSliceAdmissionSummary {
        target_node: handoff.target().graph_node_identity(),
        target_world: handoff.target().world().clone(),
        support_posture: handoff.support_snapshot().inspection_posture(),
        aggregation: handoff.aggregation(),
        query_basis: handoff.target().query_basis(),
        host_capability: handoff.target().host_capability(),
        legality_posture: handoff
            .legality_decision()
            .map(|decision| decision.posture()),
        dispatch_plan: handoff.dispatch_plan().map(summarize_dispatch_plan),
        verdicts: handoff.verdicts().iter().map(summarize_verdict).collect(),
        verdict_handles: handoff.verdict_evidence_handles().into_vec(),
        evidence_row_count: handoff.evidence_index().records().len(),
        closeout_report: handoff.closeout_report(),
    }
}

fn summarize_admission_truth(report: &UiAdmissionReport) -> LaterSliceAdmissionSummary {
    LaterSliceAdmissionSummary {
        target_node: report.target().graph_node_identity(),
        target_world: report.target().world().clone(),
        support_posture: report.support_snapshot().inspection_posture(),
        aggregation: report.aggregation(),
        query_basis: report.target().query_basis(),
        host_capability: report.target().host_capability(),
        legality_posture: report
            .legality_decision()
            .map(|decision| decision.posture()),
        dispatch_plan: report.dispatch_plan().map(summarize_dispatch_plan),
        verdicts: report.verdicts().iter().map(summarize_verdict).collect(),
        verdict_handles: report.verdict_evidence_handles().into_vec(),
        evidence_row_count: report.evidence_index().records().len(),
        closeout_report: report.closeout_report(),
    }
}

fn summarize_dispatch_plan(plan: &UiObligationDispatchPlan) -> DispatchPlanSummary {
    DispatchPlanSummary {
        entry_count: plan.entries().len(),
        plan_stop_posture: plan.plan_stop_posture(),
        shape_digest: plan.shape_digest(),
    }
}

fn summarize_verdict(
    verdict: &worth_ui_runtime::facade::obligations::UiObligationVerdict,
) -> VerdictSummary {
    VerdictSummary {
        family: verdict.family(),
        check_kind: verdict.check_kind(),
        selected_identity_digest: verdict
            .selected_identity()
            .map(|identity| identity.identity_digest()),
        class: verdict.class(),
        stop_posture: verdict.stop_posture(),
        selection_reasons: verdict.selection_reasons().to_vec(),
        prerequisite_evidence_refs: verdict.prerequisite_evidence_refs().to_vec(),
        evidence_handle: verdict.evidence_handle(),
    }
}
