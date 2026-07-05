use crate::obligations::dispatch::{UiObligationDispatchExecution, UiObligationDispatchPlan};
use crate::obligations::inspection::{
    query_prerequisite_evidence_from_refs, UiObligationEvidenceAuthoritySource,
    UiObligationEvidenceDecision, UiObligationEvidenceDispatchPosture, UiObligationEvidenceHandle,
    UiObligationEvidenceHandleKind, UiObligationEvidenceRecord,
};

pub(crate) fn dispatch_evidence_records(
    dispatch_plan: &UiObligationDispatchPlan,
) -> Vec<UiObligationEvidenceRecord> {
    if dispatch_plan.entries().is_empty() {
        return vec![UiObligationEvidenceRecord::new(
            UiObligationEvidenceHandle::new(
                UiObligationEvidenceHandleKind::Dispatch,
                dispatch_plan.shape_digest(),
            ),
            UiObligationEvidenceAuthoritySource::DispatchPlan,
            dispatch_plan.shape_digest(),
            dispatch_plan
                .selected()
                .touch()
                .target()
                .graph_node_identity()
                .digest(),
            Some(dispatch_plan.selected().touch().identity_digest()),
            None,
            UiObligationEvidenceDecision::Dispatch,
            Some(UiObligationEvidenceDispatchPosture::TypedStop(
                dispatch_plan.plan_stop_posture(),
            )),
            None,
            None,
            Box::new([]),
            Box::new([]),
            Box::new([]),
            None,
            None,
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

            UiObligationEvidenceRecord::new(
                UiObligationEvidenceHandle::new(
                    UiObligationEvidenceHandleKind::Dispatch,
                    dispatch_plan.shape_digest()
                        ^ entry
                            .selected()
                            .identity()
                            .identity_digest()
                            .rotate_left(13),
                ),
                UiObligationEvidenceAuthoritySource::DispatchPlan,
                dispatch_plan.shape_digest(),
                dispatch_plan
                    .selected()
                    .touch()
                    .target()
                    .graph_node_identity()
                    .digest(),
                Some(dispatch_plan.selected().touch().identity_digest()),
                Some(entry.selected().family()),
                UiObligationEvidenceDecision::Dispatch,
                Some(dispatch_posture),
                None,
                None,
                entry
                    .selected()
                    .selection_reasons()
                    .to_vec()
                    .into_boxed_slice(),
                crate::obligations::inspection::prerequisite_sources_from_refs(
                    entry.selected().prerequisite_evidence_refs(),
                )
                .into_boxed_slice(),
                query_prerequisite_evidence_from_refs(
                    entry.selected().prerequisite_evidence_refs(),
                )
                .into_boxed_slice(),
                None,
                None,
            )
        })
        .collect()
}
