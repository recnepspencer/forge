use crate::obligations::dispatch::{UiObligationDispatchExecution, UiObligationDispatchPlan};
use crate::obligations::inspection::{
    UiObligationEvidenceAuthoritySource, UiObligationEvidenceDecision,
    UiObligationEvidenceDispatchPosture, UiObligationEvidenceHandle,
    UiObligationEvidenceHandleKind, UiObligationEvidenceRecord, UiObligationEvidenceRecordInput,
};

pub(crate) fn dispatch_evidence_records(
    dispatch_plan: &UiObligationDispatchPlan,
) -> Vec<UiObligationEvidenceRecord> {
    if dispatch_plan.entries().is_empty() {
        return vec![UiObligationEvidenceRecord::new(
            UiObligationEvidenceRecordInput {
                handle: UiObligationEvidenceHandle::new(
                    UiObligationEvidenceHandleKind::Dispatch,
                    dispatch_plan.shape_digest(),
                ),
                authority_source: UiObligationEvidenceAuthoritySource::DispatchPlan,
                authority_digest: dispatch_plan.shape_digest(),
                graph_node_digest: dispatch_plan
                    .selected()
                    .touch()
                    .target()
                    .graph_node_identity()
                    .digest(),
                touch_identity_digest: Some(dispatch_plan.selected().touch().identity_digest()),
                family: None,
                decision: UiObligationEvidenceDecision::Dispatch,
                dispatch_posture: Some(UiObligationEvidenceDispatchPosture::TypedStop(
                    dispatch_plan.plan_stop_posture(),
                )),
                verdict_posture: None,
                denial_posture: None,
                selection_reasons: Box::new([]),
                prerequisite_sources: Box::new([]),
                non_selection_reason: None,
                legality_reason: None,
            },
        )];
    }

    dispatch_plan
        .entries()
        .iter()
        .map(|entry| {
            let dispatch_posture = if dispatch_plan.plan_stop_posture()
                != crate::obligations::verdict::UiObligationDispatchStopPosture::None
            {
                UiObligationEvidenceDispatchPosture::TypedStop(dispatch_plan.plan_stop_posture())
            } else {
                match entry.execution() {
                    UiObligationDispatchExecution::ImmediateCheck => {
                        UiObligationEvidenceDispatchPosture::ImmediateCheck
                    }
                    UiObligationDispatchExecution::TypedStop(stop_posture) => {
                        UiObligationEvidenceDispatchPosture::TypedStop(stop_posture)
                    }
                }
            };

            UiObligationEvidenceRecord::new(UiObligationEvidenceRecordInput {
                handle: UiObligationEvidenceHandle::new(
                    UiObligationEvidenceHandleKind::Dispatch,
                    dispatch_plan.shape_digest()
                        ^ entry
                            .selected()
                            .identity()
                            .identity_digest()
                            .rotate_left(13),
                ),
                authority_source: UiObligationEvidenceAuthoritySource::DispatchPlan,
                authority_digest: dispatch_plan.shape_digest(),
                graph_node_digest: dispatch_plan
                    .selected()
                    .touch()
                    .target()
                    .graph_node_identity()
                    .digest(),
                touch_identity_digest: Some(dispatch_plan.selected().touch().identity_digest()),
                family: Some(entry.selected().family()),
                decision: UiObligationEvidenceDecision::Dispatch,
                dispatch_posture: Some(dispatch_posture),
                verdict_posture: None,
                denial_posture: None,
                selection_reasons: entry
                    .selected()
                    .selection_reasons()
                    .to_vec()
                    .into_boxed_slice(),
                prerequisite_sources:
                    crate::obligations::inspection::prerequisite_sources_from_refs(
                        entry.selected().prerequisite_evidence_refs(),
                    )
                    .into_boxed_slice(),
                non_selection_reason: None,
                legality_reason: None,
            })
        })
        .collect()
}
