use crate::admission::UiAdmissionTarget;
use crate::obligations::inspection::{
    UiObligationEvidenceAuthoritySource, UiObligationEvidenceDecision, UiObligationEvidenceHandle,
    UiObligationEvidenceHandleKind, UiObligationEvidencePrerequisiteSource,
    UiObligationEvidenceRecord, UiObligationNonSelectionReason,
};
use crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef;
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::touch::UiGraphTouchDescriptor;
use worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence;

pub(crate) fn selected_obligation_evidence_records(
    selected: &UiSelectedObligationSet,
) -> Vec<UiObligationEvidenceRecord> {
    selected
        .obligations()
        .iter()
        .map(|obligation| {
            UiObligationEvidenceRecord::new(
                obligation.evidence_handle(),
                UiObligationEvidenceAuthoritySource::SelectedObligationSet,
                selected.identity_digest(),
                selected.touch().target().graph_node_identity().digest(),
                Some(selected.touch().identity_digest()),
                Some(obligation.family()),
                UiObligationEvidenceDecision::Selected,
                None,
                None,
                None,
                obligation.selection_reasons().to_vec().into_boxed_slice(),
                prerequisite_sources_from_refs(obligation.prerequisite_evidence_refs())
                    .into_boxed_slice(),
                query_prerequisite_evidence_from_refs(obligation.prerequisite_evidence_refs())
                    .into_boxed_slice(),
                None,
                None,
            )
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

    UiObligationEvidenceRecord::new(
        UiObligationEvidenceHandle::new(UiObligationEvidenceHandleKind::NotSelected, handle_seed),
        UiObligationEvidenceAuthoritySource::SelectedObligationSet,
        authority_digest,
        touch.target().graph_node_identity().digest(),
        Some(touch.identity_digest()),
        Some(family),
        UiObligationEvidenceDecision::NotSelected,
        None,
        None,
        None,
        selection_reasons,
        prerequisite_sources_from_refs(prerequisite_evidence_refs).into_boxed_slice(),
        query_prerequisite_evidence_from_refs(prerequisite_evidence_refs).into_boxed_slice(),
        Some(non_selection_reason),
        None,
    )
}

pub(crate) fn prerequisite_sources_from_target(
    target: &UiAdmissionTarget,
) -> Vec<UiObligationEvidencePrerequisiteSource> {
    let mut sources = Vec::new();
    if let Some(query) = target.query_prerequisites() {
        sources.push(UiObligationEvidencePrerequisiteSource::QueryBasis);
        sources.push(UiObligationEvidencePrerequisiteSource::QueryProjectionConsumption);
        if query.inspection_lane()
            == worth_ui_query_binding::WorthUiQueryInspectionLane::WorkspaceInspect
        {
            sources.push(UiObligationEvidencePrerequisiteSource::QueryInspection);
        }
        if query.causal_explanation_lane()
            == worth_ui_query_binding::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
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
    target
        .query_prerequisites()
        .into_iter()
        .cloned()
        .collect()
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
                    == worth_ui_query_binding::WorthUiQueryInspectionLane::WorkspaceInspect
                {
                    sources.push(UiObligationEvidencePrerequisiteSource::QueryInspection);
                }
                if evidence.causal_explanation_lane()
                    == worth_ui_query_binding::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
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
