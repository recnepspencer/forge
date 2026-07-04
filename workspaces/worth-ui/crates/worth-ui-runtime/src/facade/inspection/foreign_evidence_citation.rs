use std::collections::BTreeSet;

use worth_ui_query_binding::WorthUiQueryPrerequisiteEvidence;

use crate::facade::retained_obligation_registry::WorthUiRetainedObligationRegistry;
use crate::facade::{
    UiInspectionForeignEvidenceCitation, UiInspectionForeignEvidenceRef,
    UiInspectionQueryForeignEvidenceArtifactKind, UiInspectionQueryForeignEvidenceCitation,
    UiInspectionQueryForeignEvidenceKind, UiInspectionQueryForeignEvidenceRef,
};
use crate::obligations::inspection::UiObligationEvidenceRecord;

pub(crate) fn cite_foreign_evidence(
    registry: &WorthUiRetainedObligationRegistry,
    foreign_ref: UiInspectionForeignEvidenceRef,
) -> UiInspectionForeignEvidenceCitation {
    match foreign_ref {
        UiInspectionForeignEvidenceRef::Query(query_ref) => {
            UiInspectionForeignEvidenceCitation::Query(UiInspectionQueryForeignEvidenceCitation::new(
                query_ref,
                resolve_query_prerequisite_evidence(registry, query_ref),
            ))
        }
    }
}

pub(crate) fn foreign_evidence_refs_for_obligation_record(
    record: &UiObligationEvidenceRecord,
) -> Box<[UiInspectionForeignEvidenceRef]> {
    record
        .query_prerequisite_evidence()
        .iter()
        .flat_map(|evidence| {
            query_foreign_routes(evidence).map(move |kind| {
                UiInspectionForeignEvidenceRef::Query(UiInspectionQueryForeignEvidenceRef::new(
                    kind,
                    UiInspectionQueryForeignEvidenceArtifactKind::PrerequisiteEvidence,
                    query_artifact_identity_digest(evidence),
                    record.handle().digest(),
                    record.graph_node_digest(),
                    record.touch_identity_digest(),
                ))
            })
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn resolve_query_prerequisite_evidence(
    registry: &WorthUiRetainedObligationRegistry,
    query_ref: UiInspectionQueryForeignEvidenceRef,
) -> Option<WorthUiQueryPrerequisiteEvidence> {
    let selected = registry.retained_selection(query_ref.obligation_handle_digest())?;
    selected
        .evidence_index()
        .records()
        .iter()
        .find(|record| record.handle().digest() == query_ref.obligation_handle_digest())
        .and_then(|record| {
            record
                .query_prerequisite_evidence()
                .iter()
                .find(|evidence| {
                    query_artifact_identity_digest(evidence) == query_ref.artifact_identity_digest()
                        && query_foreign_routes(evidence).any(|kind| kind == query_ref.kind())
                })
                .cloned()
        })
}

fn query_foreign_routes(
    evidence: &WorthUiQueryPrerequisiteEvidence,
) -> impl Iterator<Item = UiInspectionQueryForeignEvidenceKind> + '_ {
    let mut routes = Vec::from([UiInspectionQueryForeignEvidenceKind::ProjectionConsumption]);
    if evidence.inspection_lane() == worth_ui_query_binding::WorthUiQueryInspectionLane::WorkspaceInspect
    {
        routes.push(UiInspectionQueryForeignEvidenceKind::Inspection);
    }
    if evidence.causal_explanation_lane()
        == worth_ui_query_binding::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
    {
        routes.push(UiInspectionQueryForeignEvidenceKind::CausalExplanation);
    }
    routes.into_iter()
}

fn query_artifact_identity_digest(evidence: &WorthUiQueryPrerequisiteEvidence) -> u64 {
    stable_text_digest(evidence.basis().proof().digest().as_str())
        ^ (evidence.basis_posture() as u64).rotate_left(13)
        ^ (evidence.projection_consumption_lane() as u64).rotate_left(29)
        ^ (evidence.inspection_lane() as u64).rotate_left(37)
        ^ (evidence.causal_explanation_lane() as u64).rotate_left(43)
}

fn stable_text_digest(text: &str) -> u64 {
    text.as_bytes()
        .iter()
        .fold(0xCBF2_9CE4_8422_2325, |digest, byte| {
            digest.wrapping_mul(0x0000_0100_0000_01B3) ^ u64::from(*byte)
        })
}
