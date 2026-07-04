use crate::obligations::inspection::{
    prerequisite_sources_from_refs, query_prerequisite_evidence_from_refs,
    UiObligationEvidenceAuthoritySource, UiObligationEvidenceDecision,
    UiObligationEvidenceRecord, UiObligationEvidenceVerdictPosture,
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
            UiObligationEvidenceRecord::new(
                verdict.evidence_handle(),
                UiObligationEvidenceAuthoritySource::ObligationVerdict,
                verdict.identity_digest(),
                selected.touch().target().graph_node_identity().digest(),
                Some(selected.touch().identity_digest()),
                verdict.family(),
                UiObligationEvidenceDecision::Verdict,
                None,
                Some(UiObligationEvidenceVerdictPosture::new(
                    verdict.class(),
                    verdict.stop_posture(),
                )),
                None,
                verdict.selection_reasons().to_vec().into_boxed_slice(),
                selected
                    .obligations()
                    .iter()
                    .find(|entry| verdict.selected_identity() == Some(entry.identity()))
                    .map(|entry| {
                        prerequisite_sources_from_refs(entry.prerequisite_evidence_refs())
                    })
                    .unwrap_or_default()
                    .into_boxed_slice(),
                selected
                    .obligations()
                    .iter()
                    .find(|entry| verdict.selected_identity() == Some(entry.identity()))
                    .map(|entry| {
                        query_prerequisite_evidence_from_refs(entry.prerequisite_evidence_refs())
                    })
                    .unwrap_or_default()
                    .into_boxed_slice(),
                None,
                None,
            )
        })
        .collect()
}
