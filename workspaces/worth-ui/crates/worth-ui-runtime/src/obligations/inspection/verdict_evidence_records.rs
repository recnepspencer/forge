use crate::obligations::inspection::{
    prerequisite_sources_from_refs, UiObligationEvidenceAuthoritySource,
    UiObligationEvidenceDecision, UiObligationEvidenceRecord, UiObligationEvidenceRecordInput,
    UiObligationEvidenceVerdictPosture,
};
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::verdict::UiObligationVerdict;

pub(crate) fn verdict_evidence_records(
    selected: &UiSelectedObligationSet,
    verdicts: &[UiObligationVerdict],
) -> Vec<UiObligationEvidenceRecord> {
    verdicts
        .iter()
        .map(|verdict| {
            UiObligationEvidenceRecord::new(UiObligationEvidenceRecordInput {
                handle: verdict.evidence_handle(),
                authority_source: UiObligationEvidenceAuthoritySource::ObligationVerdict,
                authority_digest: verdict.identity_digest(),
                graph_node_digest: selected.touch().target().graph_node_identity().digest(),
                touch_identity_digest: Some(selected.touch().identity_digest()),
                family: verdict.family(),
                decision: UiObligationEvidenceDecision::Verdict,
                dispatch_posture: None,
                verdict_posture: Some(UiObligationEvidenceVerdictPosture::new(
                    verdict.class(),
                    verdict.stop_posture(),
                )),
                denial_posture: None,
                selection_reasons: verdict.selection_reasons().to_vec().into_boxed_slice(),
                prerequisite_sources: selected
                    .obligations()
                    .iter()
                    .find(|entry| verdict.selected_identity() == Some(entry.identity()))
                    .map(|entry| prerequisite_sources_from_refs(entry.prerequisite_evidence_refs()))
                    .unwrap_or_default()
                    .into_boxed_slice(),
                non_selection_reason: None,
                legality_reason: None,
            })
        })
        .collect()
}
