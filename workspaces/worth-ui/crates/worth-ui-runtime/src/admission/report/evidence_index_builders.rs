use crate::admission::{
    UiAdmissionAggregation, UiAdmissionDecision, UiAdmissionTarget, UiLegalityPosture,
};
use crate::obligations::inspection::{
    UiObligationEvidenceDecision, UiObligationEvidenceDenialPosture, UiObligationEvidenceHandle,
    UiObligationEvidenceHandleKind, UiObligationEvidenceIndex, UiObligationEvidenceRecord,
    UiObligationLegalityReasonEvidence,
};
use crate::obligations::selection::UiSelectedObligationSet;
use crate::obligations::verdict::{UiObligationVerdict, UiObligationVerdictClass};

pub(crate) fn verdict_evidence_records(
    selected: &UiSelectedObligationSet,
    verdicts: &[UiObligationVerdict],
) -> Vec<UiObligationEvidenceRecord> {
    verdicts
        .iter()
        .map(|verdict| {
            UiObligationEvidenceRecord::new(
                verdict.evidence_handle(),
                selected.touch().target().graph_node_identity().digest(),
                Some(selected.touch().identity_digest()),
                verdict.family(),
                UiObligationEvidenceDecision::Verdict,
                denial_posture_from_verdict(verdict),
                verdict.selection_reasons().to_vec().into_boxed_slice(),
                selected
                    .obligations()
                    .iter()
                    .find(|entry| verdict.selected_identity() == Some(entry.identity()))
                    .map(|entry| prerequisite_sources_from_refs(entry.prerequisite_evidence_refs()))
                    .unwrap_or_default()
                    .into_boxed_slice(),
                None,
                None,
            )
        })
        .collect()
}

pub(crate) fn denial_evidence_index(decision: &UiAdmissionDecision) -> UiObligationEvidenceIndex {
    let legality_reason = decision
        .legality_decision()
        .and_then(|legality| legality_reason_from_posture(legality.posture()));
    let denial_posture = legality_reason.and_then(denial_posture_from_legality_reason);
    UiObligationEvidenceIndex::new(
        vec![UiObligationEvidenceRecord::new(
            UiObligationEvidenceHandle::new(
                UiObligationEvidenceHandleKind::Admission,
                decision
                    .support_snapshot()
                    .target()
                    .graph_node_identity()
                    .digest(),
            ),
            decision
                .support_snapshot()
                .target()
                .graph_node_identity()
                .digest(),
            None,
            None,
            UiObligationEvidenceDecision::Admission,
            denial_posture,
            Box::new([]),
            prerequisite_sources_from_target(decision.support_snapshot().target())
                .into_boxed_slice(),
            None,
            legality_reason,
        )]
        .into_boxed_slice(),
    )
}

pub(crate) fn aggregation_from_selected(
    selected: &UiSelectedObligationSet,
    verdicts: &[UiObligationVerdict],
) -> UiAdmissionAggregation {
    match selected.support_snapshot().posture() {
        crate::admission::UiSupportPosture::Unsupported { .. } => {
            UiAdmissionAggregation::Unsupported
        }
        crate::admission::UiSupportPosture::WrongWorld { .. } => UiAdmissionAggregation::WrongWorld,
        crate::admission::UiSupportPosture::Deferred { .. } => UiAdmissionAggregation::Deferred,
        crate::admission::UiSupportPosture::DiagnosticOnly { .. } => {
            UiAdmissionAggregation::DiagnosticOnly
        }
        crate::admission::UiSupportPosture::Supported { .. } => {
            if verdicts
                .iter()
                .any(|verdict| verdict.class() == UiObligationVerdictClass::Violation)
            {
                UiAdmissionAggregation::Denied
            } else if verdicts
                .iter()
                .any(|verdict| verdict.class() == UiObligationVerdictClass::Advisory)
            {
                UiAdmissionAggregation::AdmittedWithAdvisory
            } else {
                UiAdmissionAggregation::Admitted
            }
        }
    }
}

fn prerequisite_sources_from_refs(
    refs: &[crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef],
) -> Vec<crate::obligations::inspection::UiObligationEvidencePrerequisiteSource> {
    let mut sources = Vec::new();
    for reference in refs {
        match reference {
            crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef::Query(
                evidence,
            ) => {
                sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryBasis);
                sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryProjectionConsumption);
                if evidence.inspection_lane()
                    == worth_ui_query_binding::WorthUiQueryInspectionLane::WorkspaceInspect
                {
                    sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryInspection);
                }
                if evidence.causal_explanation_lane()
                    == worth_ui_query_binding::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
                {
                    sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryCausalExplanation);
                }
            }
            crate::obligations::prerequisites::UiObligationPrerequisiteEvidenceRef::Host(_) => {
                sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::HostCapability);
            }
        }
    }
    sources
}

fn legality_reason_from_posture(
    posture: UiLegalityPosture,
) -> Option<UiObligationLegalityReasonEvidence> {
    match posture {
        UiLegalityPosture::Denied(reason) | UiLegalityPosture::AdmittedWithAdvisory(reason) => {
            Some(legality_reason_evidence(reason))
        }
        UiLegalityPosture::Admitted => None,
    }
}

fn legality_reason_evidence(
    reason: crate::admission::UiLegalityReason,
) -> UiObligationLegalityReasonEvidence {
    match reason {
        crate::admission::UiLegalityReason::MissingDeclarationArtifact => {
            UiObligationLegalityReasonEvidence::MissingDeclarationArtifact
        }
        crate::admission::UiLegalityReason::MissingQueryPrerequisiteEvidence => {
            UiObligationLegalityReasonEvidence::MissingQueryPrerequisiteEvidence
        }
        crate::admission::UiLegalityReason::MissingHostCapabilityReport => {
            UiObligationLegalityReasonEvidence::MissingHostCapabilityReport
        }
        crate::admission::UiLegalityReason::QueryBindingRequiresLaterRuntimeLane => {
            UiObligationLegalityReasonEvidence::QueryBindingRequiresLaterRuntimeLane
        }
        crate::admission::UiLegalityReason::ServiceUsageRequiresLaterRuntimeLane => {
            UiObligationLegalityReasonEvidence::ServiceUsageRequiresLaterRuntimeLane
        }
        crate::admission::UiLegalityReason::WrongQueryBasis { required, observed } => {
            UiObligationLegalityReasonEvidence::WrongQueryBasis { required, observed }
        }
        crate::admission::UiLegalityReason::WrongHostCapability { required, observed } => {
            UiObligationLegalityReasonEvidence::WrongHostCapability { required, observed }
        }
        crate::admission::UiLegalityReason::Stale {
            required,
            observed,
            evidence,
        } => UiObligationLegalityReasonEvidence::Stale {
            required,
            observed,
            evidence,
        },
        crate::admission::UiLegalityReason::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        } => UiObligationLegalityReasonEvidence::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        },
        crate::admission::UiLegalityReason::RebindRequired { required, observed } => {
            UiObligationLegalityReasonEvidence::RebindRequired { required, observed }
        }
        crate::admission::UiLegalityReason::BudgetExceeded {
            budget,
            attempted_lane_cost,
        } => UiObligationLegalityReasonEvidence::BudgetExceeded {
            budget,
            attempted_lane_cost,
        },
    }
}

fn denial_posture_from_legality_reason(
    reason: UiObligationLegalityReasonEvidence,
) -> Option<UiObligationEvidenceDenialPosture> {
    match reason {
        UiObligationLegalityReasonEvidence::MissingDeclarationArtifact
        | UiObligationLegalityReasonEvidence::MissingQueryPrerequisiteEvidence
        | UiObligationLegalityReasonEvidence::MissingHostCapabilityReport => {
            Some(UiObligationEvidenceDenialPosture::Unsupported)
        }
        UiObligationLegalityReasonEvidence::QueryBindingRequiresLaterRuntimeLane
        | UiObligationLegalityReasonEvidence::ServiceUsageRequiresLaterRuntimeLane => None,
        UiObligationLegalityReasonEvidence::WrongQueryBasis { required, observed }
        | UiObligationLegalityReasonEvidence::RebindRequired { required, observed } => {
            Some(UiObligationEvidenceDenialPosture::WrongQueryBasis { required, observed })
        }
        UiObligationLegalityReasonEvidence::WrongHostCapability { required, observed } => {
            Some(UiObligationEvidenceDenialPosture::WrongHostCapability { required, observed })
        }
        UiObligationLegalityReasonEvidence::Stale {
            required,
            observed,
            evidence,
        } => Some(UiObligationEvidenceDenialPosture::Stale {
            required,
            observed,
            evidence,
        }),
        UiObligationLegalityReasonEvidence::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        } => Some(UiObligationEvidenceDenialPosture::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        }),
        UiObligationLegalityReasonEvidence::BudgetExceeded {
            budget,
            attempted_lane_cost,
        } => Some(UiObligationEvidenceDenialPosture::BudgetExceeded {
            budget,
            attempted_lane_cost,
        }),
    }
}

fn prerequisite_sources_from_target(
    target: &UiAdmissionTarget,
) -> Vec<crate::obligations::inspection::UiObligationEvidencePrerequisiteSource> {
    let mut sources = Vec::new();
    if let Some(query) = target.query_prerequisites() {
        sources.push(
            crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryBasis,
        );
        sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryProjectionConsumption);
        if query.inspection_lane()
            == worth_ui_query_binding::WorthUiQueryInspectionLane::WorkspaceInspect
        {
            sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryInspection);
        }
        if query.causal_explanation_lane()
            == worth_ui_query_binding::WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection
        {
            sources.push(crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::QueryCausalExplanation);
        }
    }
    if target.host_capability_report().is_some() {
        sources.push(
            crate::obligations::inspection::UiObligationEvidencePrerequisiteSource::HostCapability,
        );
    }
    sources
}

fn denial_posture_from_verdict(
    verdict: &UiObligationVerdict,
) -> Option<UiObligationEvidenceDenialPosture> {
    match verdict.stop_posture() {
        crate::obligations::verdict::UiObligationDispatchStopPosture::None => None,
        crate::obligations::verdict::UiObligationDispatchStopPosture::Unsupported => {
            Some(UiObligationEvidenceDenialPosture::Unsupported)
        }
        crate::obligations::verdict::UiObligationDispatchStopPosture::Deferred => {
            Some(UiObligationEvidenceDenialPosture::Deferred)
        }
        crate::obligations::verdict::UiObligationDispatchStopPosture::DiagnosticOnly => {
            Some(UiObligationEvidenceDenialPosture::DiagnosticOnly)
        }
        crate::obligations::verdict::UiObligationDispatchStopPosture::WrongWorld => {
            Some(UiObligationEvidenceDenialPosture::WrongWorld)
        }
        crate::obligations::verdict::UiObligationDispatchStopPosture::WrongQueryBasis {
            required,
            observed,
        } => Some(UiObligationEvidenceDenialPosture::WrongQueryBasis { required, observed }),
        crate::obligations::verdict::UiObligationDispatchStopPosture::WrongHostCapability {
            required,
            observed,
        } => Some(UiObligationEvidenceDenialPosture::WrongHostCapability { required, observed }),
        crate::obligations::verdict::UiObligationDispatchStopPosture::Stale {
            required,
            observed,
            evidence,
        } => Some(UiObligationEvidenceDenialPosture::Stale {
            required,
            observed,
            evidence,
        }),
        crate::obligations::verdict::UiObligationDispatchStopPosture::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        } => Some(UiObligationEvidenceDenialPosture::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        }),
        crate::obligations::verdict::UiObligationDispatchStopPosture::BudgetExceeded {
            budget,
            attempted_lane_cost,
        } => Some(UiObligationEvidenceDenialPosture::BudgetExceeded {
            budget,
            attempted_lane_cost,
        }),
    }
}
