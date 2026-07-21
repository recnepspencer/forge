use crate::admission::UiAdmissionTarget;
use crate::obligations::inspection::{
    UiObligationEvidenceAuthoritySource, UiObligationEvidenceDecision, UiObligationEvidenceHandle,
    UiObligationEvidenceHandleKind, UiObligationEvidencePrerequisiteSource,
    UiObligationEvidenceRecord, UiObligationEvidenceRecordInput, UiObligationNonSelectionReason,
};
use crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef;
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::touch::UiGraphTouchDescriptor;
use worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence;

pub(crate) fn selected_obligation_evidence_records(
    selected: &UiSelectedObligationSet,
) -> Vec<UiObligationEvidenceRecord> {
    selected
        .obligations()
        .iter()
        .map(|obligation| {
            UiObligationEvidenceRecord::new(UiObligationEvidenceRecordInput {
                handle: obligation.evidence_handle(),
                authority_source: UiObligationEvidenceAuthoritySource::SelectedObligationSet,
                authority_digest: selected.identity_digest(),
                graph_node_digest: selected.touch().target().graph_node_identity().digest(),
                touch_identity_digest: Some(selected.touch().identity_digest()),
                family: Some(obligation.family()),
                decision: UiObligationEvidenceDecision::Selected,
                dispatch_posture: None,
                verdict_posture: None,
                denial_posture: None,
                selection_reasons: obligation.selection_reasons().to_vec().into_boxed_slice(),
                prerequisite_sources: prerequisite_sources_from_refs(
                    obligation.prerequisite_evidence_refs(),
                )
                .into_boxed_slice(),
                query_prerequisite_evidence: query_prerequisite_evidence_from_refs(
                    obligation.prerequisite_evidence_refs(),
                )
                .into_boxed_slice(),
                non_selection_reason: None,
                legality_reason: None,
            })
        })
        .collect()
}

pub(crate) fn not_selected_obligation_evidence_record(
    touch: &UiGraphTouchDescriptor,
    authority_digest: u64,
    ordinal: usize,
    family: crate::obligations::catalog::UiObligationFamily,
    selection_reasons: Box<[crate::obligations::selection::UiObligationSelectionReason]>,
    prerequisite_evidence_refs: &[UiObligationPrerequisiteEvidenceRef],
    non_selection_reason: UiObligationNonSelectionReason,
) -> UiObligationEvidenceRecord {
    let handle_seed = touch.identity_digest()
        ^ (family as u64).rotate_left(11)
        ^ (ordinal as u64).rotate_left(29);

    UiObligationEvidenceRecord::new(UiObligationEvidenceRecordInput {
        handle: UiObligationEvidenceHandle::new(
            UiObligationEvidenceHandleKind::NotSelected,
            handle_seed,
        ),
        authority_source: UiObligationEvidenceAuthoritySource::SelectedObligationSet,
        authority_digest,
        graph_node_digest: touch.target().graph_node_identity().digest(),
        touch_identity_digest: Some(touch.identity_digest()),
        family: Some(family),
        decision: UiObligationEvidenceDecision::NotSelected,
        dispatch_posture: None,
        verdict_posture: None,
        denial_posture: None,
        selection_reasons,
        prerequisite_sources: prerequisite_sources_from_refs(prerequisite_evidence_refs)
            .into_boxed_slice(),
        query_prerequisite_evidence: query_prerequisite_evidence_from_refs(
            prerequisite_evidence_refs,
        )
        .into_boxed_slice(),
        non_selection_reason: Some(non_selection_reason),
        legality_reason: None,
    })
}

pub(crate) fn prerequisite_sources_from_target(
    target: &UiAdmissionTarget,
) -> Vec<UiObligationEvidencePrerequisiteSource> {
    let mut sources = Vec::new();
    if let Some(query) = target.query_prerequisites() {
        sources.push(UiObligationEvidencePrerequisiteSource::QueryBasis);
        sources.push(UiObligationEvidencePrerequisiteSource::QueryProjectionConsumption);
        if query.inspection_lane()
            == worth_ui_query_binding::compatibility::managed_live::WorthUiQueryInspectionLane::WorkspaceInspect
        {
            sources.push(UiObligationEvidencePrerequisiteSource::QueryInspection);
        }
        if query.causal_explanation_lane()
            == worth_ui_query_binding::compatibility::managed_live::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
        {
            sources.push(UiObligationEvidencePrerequisiteSource::QueryCausalExplanation);
        }
    }
    if target.host_capability_report().is_some() {
        sources.push(UiObligationEvidencePrerequisiteSource::HostCapability);
    }
    sources
}

pub(crate) fn query_prerequisite_evidence_from_target(
    target: &UiAdmissionTarget,
) -> Vec<WorthUiQueryPrerequisiteEvidence> {
    target.query_prerequisites().into_iter().cloned().collect()
}

pub(crate) fn prerequisite_sources_from_refs(
    refs: &[UiObligationPrerequisiteEvidenceRef],
) -> Vec<UiObligationEvidencePrerequisiteSource> {
    let mut sources = Vec::new();
    for reference in refs {
        match reference {
            UiObligationPrerequisiteEvidenceRef::Query(evidence) => {
                sources.push(UiObligationEvidencePrerequisiteSource::QueryBasis);
                sources.push(UiObligationEvidencePrerequisiteSource::QueryProjectionConsumption);
                if evidence.inspection_lane()
                    == worth_ui_query_binding::compatibility::managed_live::WorthUiQueryInspectionLane::WorkspaceInspect
                {
                    sources.push(UiObligationEvidencePrerequisiteSource::QueryInspection);
                }
                if evidence.causal_explanation_lane()
                    == worth_ui_query_binding::compatibility::managed_live::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
                {
                    sources.push(UiObligationEvidencePrerequisiteSource::QueryCausalExplanation);
                }
            }
            UiObligationPrerequisiteEvidenceRef::Host(_) => {
                sources.push(UiObligationEvidencePrerequisiteSource::HostCapability);
            }
        }
    }
    sources
}

pub(crate) fn query_prerequisite_evidence_from_refs(
    refs: &[UiObligationPrerequisiteEvidenceRef],
) -> Vec<WorthUiQueryPrerequisiteEvidence> {
    refs.iter()
        .filter_map(UiObligationPrerequisiteEvidenceRef::query)
        .cloned()
        .collect()
}
