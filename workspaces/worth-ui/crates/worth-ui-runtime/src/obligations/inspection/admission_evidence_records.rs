use crate::admission::{UiAdmissionReport, UiLegalityPosture};
use crate::obligations::inspection::{
    prerequisite_sources_from_target, UiObligationEvidenceAuthoritySource,
    UiObligationEvidenceDecision, UiObligationEvidenceDenialPosture, UiObligationEvidenceHandle,
    UiObligationEvidenceHandleKind, UiObligationEvidenceRecord, UiObligationEvidenceRecordInput,
    UiObligationLegalityReasonEvidence,
};

pub(crate) fn admitted_report_evidence_records(
    report: &UiAdmissionReport,
) -> Vec<UiObligationEvidenceRecord> {
    vec![admission_record(report)]
}

fn admission_record(report: &UiAdmissionReport) -> UiObligationEvidenceRecord {
    let authority_digest = report.identity_digest();
    let target = report.target();
    let legality_posture = report
        .legality_decision()
        .map(|legality| legality.posture());
    let legality_reason = legality_posture.and_then(legality_reason_from_posture);
    let touch_identity_digest = report
        .dispatch_plan()
        .map(|dispatch| dispatch.selected().touch().identity_digest());

    UiObligationEvidenceRecord::new(UiObligationEvidenceRecordInput {
        handle: UiObligationEvidenceHandle::new(
            UiObligationEvidenceHandleKind::Admission,
            authority_digest ^ target.graph_node_identity().digest().rotate_left(17),
        ),
        authority_source: UiObligationEvidenceAuthoritySource::AdmissionReport,
        authority_digest,
        graph_node_digest: target.graph_node_identity().digest(),
        touch_identity_digest,
        family: None,
        decision: UiObligationEvidenceDecision::Admission,
        dispatch_posture: None,
        verdict_posture: None,
        denial_posture: legality_reason.and_then(denial_posture_from_legality_reason),
        selection_reasons: Box::new([]),
        prerequisite_sources: prerequisite_sources_from_target(target).into_boxed_slice(),
        non_selection_reason: None,
        legality_reason,
    })
}

fn legality_reason_from_posture(
    posture: UiLegalityPosture,
) -> Option<UiObligationLegalityReasonEvidence> {
    match posture {
        UiLegalityPosture::Denied(reason) | UiLegalityPosture::AdmittedWithAdvisory(reason) => {
            Some(match reason {
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
            })
        }
        UiLegalityPosture::Admitted => None,
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
