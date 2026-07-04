use crate::obligations::catalog::UiObligationFamily;
use worth_ui_inspection::{
    UiInspectionEvidenceSource, UiInspectionObligationDecision,
    UiInspectionObligationDenialPosture, UiInspectionObligationDispatchPosture,
    UiInspectionObligationFamily, UiInspectionObligationLegalityReason,
    UiInspectionObligationNonSelectionReason, UiInspectionObligationVerdictClass,
    UiInspectionObligationVerdictPosture,
};

use super::selection_reason_mapping::{
    inspection_budget, inspection_host_capability, inspection_query_basis,
    inspection_stale_evidence,
};
use super::{
    UiObligationEvidenceDecision, UiObligationEvidenceDenialPosture,
    UiObligationEvidenceDispatchPosture,
    UiObligationEvidencePrerequisiteSource, UiObligationLegalityReasonEvidence,
    UiObligationNonSelectionReason,
};
use crate::obligations::verdict::UiObligationDispatchStopPosture;

pub(super) fn inspection_decision(
    decision: UiObligationEvidenceDecision,
) -> UiInspectionObligationDecision {
    match decision {
        UiObligationEvidenceDecision::Selected => UiInspectionObligationDecision::Selected,
        UiObligationEvidenceDecision::NotSelected => UiInspectionObligationDecision::NotSelected,
        UiObligationEvidenceDecision::Dispatch => UiInspectionObligationDecision::Dispatch,
        UiObligationEvidenceDecision::Verdict => UiInspectionObligationDecision::Verdict,
        UiObligationEvidenceDecision::Admission => UiInspectionObligationDecision::Admission,
    }
}

pub(super) fn inspection_dispatch_posture(
    posture: UiObligationEvidenceDispatchPosture,
) -> UiInspectionObligationDispatchPosture {
    match posture {
        UiObligationEvidenceDispatchPosture::ImmediateCheck => {
            UiInspectionObligationDispatchPosture::ImmediateCheck
        }
        UiObligationEvidenceDispatchPosture::TypedStop(stop_posture) => {
            inspection_dispatch_stop_posture(stop_posture)
        }
    }
}

pub(super) fn inspection_verdict_class(
    class: crate::obligations::verdict::UiObligationVerdictClass,
) -> UiInspectionObligationVerdictClass {
    match class {
        crate::obligations::verdict::UiObligationVerdictClass::Success => {
            UiInspectionObligationVerdictClass::Success
        }
        crate::obligations::verdict::UiObligationVerdictClass::Advisory => {
            UiInspectionObligationVerdictClass::Advisory
        }
        crate::obligations::verdict::UiObligationVerdictClass::Violation => {
            UiInspectionObligationVerdictClass::Violation
        }
    }
}

pub(super) fn inspection_verdict_posture(
    stop_posture: UiObligationDispatchStopPosture,
) -> UiInspectionObligationVerdictPosture {
    match stop_posture {
        UiObligationDispatchStopPosture::None => UiInspectionObligationVerdictPosture::None,
        UiObligationDispatchStopPosture::Unsupported => {
            UiInspectionObligationVerdictPosture::Unsupported
        }
        UiObligationDispatchStopPosture::Deferred => {
            UiInspectionObligationVerdictPosture::Deferred
        }
        UiObligationDispatchStopPosture::DiagnosticOnly => {
            UiInspectionObligationVerdictPosture::DiagnosticOnly
        }
        UiObligationDispatchStopPosture::WrongWorld => {
            UiInspectionObligationVerdictPosture::WrongWorld
        }
        UiObligationDispatchStopPosture::WrongQueryBasis { required, observed } => {
            UiInspectionObligationVerdictPosture::WrongQueryBasis {
                required: inspection_query_basis(required),
                observed: inspection_query_basis(observed),
            }
        }
        UiObligationDispatchStopPosture::WrongHostCapability { required, observed } => {
            UiInspectionObligationVerdictPosture::WrongHostCapability {
                required: inspection_host_capability(required),
                observed: inspection_host_capability(observed),
            }
        }
        UiObligationDispatchStopPosture::Stale {
            required,
            observed,
            evidence,
        } => UiInspectionObligationVerdictPosture::Stale {
            required: inspection_query_basis(required),
            observed: inspection_query_basis(observed),
            evidence: inspection_stale_evidence(evidence),
        },
        UiObligationDispatchStopPosture::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        } => UiInspectionObligationVerdictPosture::Ambiguous {
            required_query_basis: required_query_basis.map(inspection_query_basis),
            observed_query_basis: observed_query_basis.map(inspection_query_basis),
            required_host_capability: required_host_capability.map(inspection_host_capability),
            observed_host_capability: observed_host_capability.map(inspection_host_capability),
        },
        UiObligationDispatchStopPosture::BudgetExceeded {
            budget,
            attempted_lane_cost,
        } => UiInspectionObligationVerdictPosture::BudgetExceeded {
            budget: inspection_budget(budget),
            attempted_lane_cost,
        },
    }
}

pub(super) fn inspection_family(family: UiObligationFamily) -> UiInspectionObligationFamily {
    match family {
        UiObligationFamily::StructuralLegality => UiInspectionObligationFamily::StructuralLegality,
        UiObligationFamily::ParticipationLegality => {
            UiInspectionObligationFamily::ParticipationLegality
        }
        UiObligationFamily::SlotContract => UiInspectionObligationFamily::SlotContract,
        UiObligationFamily::MeasurementRequirement => {
            UiInspectionObligationFamily::MeasurementRequirement
        }
        UiObligationFamily::QueryBindingRequirement => {
            UiInspectionObligationFamily::QueryBindingRequirement
        }
        UiObligationFamily::IntentOperabilityRequirement => {
            UiInspectionObligationFamily::IntentOperabilityRequirement
        }
        UiObligationFamily::PortalHostRequirement => {
            UiInspectionObligationFamily::PortalHostRequirement
        }
        UiObligationFamily::FocusRouteRequirement => {
            UiInspectionObligationFamily::FocusRouteRequirement
        }
        UiObligationFamily::MotionSupportRequirement => {
            UiInspectionObligationFamily::MotionSupportRequirement
        }
        UiObligationFamily::AccessibilityRequirement => {
            UiInspectionObligationFamily::AccessibilityRequirement
        }
        UiObligationFamily::HostCapabilityRequirement => {
            UiInspectionObligationFamily::HostCapabilityRequirement
        }
        UiObligationFamily::DiagnosticSurfaceRequirement => {
            UiInspectionObligationFamily::DiagnosticSurfaceRequirement
        }
    }
}

pub(super) fn inspection_denial_posture(
    posture: UiObligationEvidenceDenialPosture,
) -> UiInspectionObligationDenialPosture {
    match posture {
        UiObligationEvidenceDenialPosture::Unsupported => {
            UiInspectionObligationDenialPosture::Unsupported
        }
        UiObligationEvidenceDenialPosture::Deferred => {
            UiInspectionObligationDenialPosture::Deferred
        }
        UiObligationEvidenceDenialPosture::DiagnosticOnly => {
            UiInspectionObligationDenialPosture::DiagnosticOnly
        }
        UiObligationEvidenceDenialPosture::WrongWorld => {
            UiInspectionObligationDenialPosture::WrongWorld
        }
        UiObligationEvidenceDenialPosture::WrongQueryBasis { required, observed } => {
            UiInspectionObligationDenialPosture::WrongQueryBasis {
                required: inspection_query_basis(required),
                observed: inspection_query_basis(observed),
            }
        }
        UiObligationEvidenceDenialPosture::WrongHostCapability { required, observed } => {
            UiInspectionObligationDenialPosture::WrongHostCapability {
                required: inspection_host_capability(required),
                observed: inspection_host_capability(observed),
            }
        }
        UiObligationEvidenceDenialPosture::Stale {
            required,
            observed,
            evidence,
        } => UiInspectionObligationDenialPosture::Stale {
            required: inspection_query_basis(required),
            observed: inspection_query_basis(observed),
            evidence: inspection_stale_evidence(evidence),
        },
        UiObligationEvidenceDenialPosture::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        } => UiInspectionObligationDenialPosture::Ambiguous {
            required_query_basis: required_query_basis.map(inspection_query_basis),
            observed_query_basis: observed_query_basis.map(inspection_query_basis),
            required_host_capability: required_host_capability.map(inspection_host_capability),
            observed_host_capability: observed_host_capability.map(inspection_host_capability),
        },
        UiObligationEvidenceDenialPosture::BudgetExceeded {
            budget,
            attempted_lane_cost,
        } => UiInspectionObligationDenialPosture::BudgetExceeded {
            budget: inspection_budget(budget),
            attempted_lane_cost,
        },
    }
}

fn inspection_dispatch_stop_posture(
    stop_posture: UiObligationDispatchStopPosture,
) -> UiInspectionObligationDispatchPosture {
    match stop_posture {
        UiObligationDispatchStopPosture::None => UiInspectionObligationDispatchPosture::ImmediateCheck,
        UiObligationDispatchStopPosture::Unsupported => {
            UiInspectionObligationDispatchPosture::Unsupported
        }
        UiObligationDispatchStopPosture::Deferred => {
            UiInspectionObligationDispatchPosture::Deferred
        }
        UiObligationDispatchStopPosture::DiagnosticOnly => {
            UiInspectionObligationDispatchPosture::DiagnosticOnly
        }
        UiObligationDispatchStopPosture::WrongWorld => {
            UiInspectionObligationDispatchPosture::WrongWorld
        }
        UiObligationDispatchStopPosture::WrongQueryBasis { required, observed } => {
            UiInspectionObligationDispatchPosture::WrongQueryBasis {
                required: inspection_query_basis(required),
                observed: inspection_query_basis(observed),
            }
        }
        UiObligationDispatchStopPosture::WrongHostCapability { required, observed } => {
            UiInspectionObligationDispatchPosture::WrongHostCapability {
                required: inspection_host_capability(required),
                observed: inspection_host_capability(observed),
            }
        }
        UiObligationDispatchStopPosture::Stale {
            required,
            observed,
            evidence,
        } => UiInspectionObligationDispatchPosture::Stale {
            required: inspection_query_basis(required),
            observed: inspection_query_basis(observed),
            evidence: inspection_stale_evidence(evidence),
        },
        UiObligationDispatchStopPosture::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        } => UiInspectionObligationDispatchPosture::Ambiguous {
            required_query_basis: required_query_basis.map(inspection_query_basis),
            observed_query_basis: observed_query_basis.map(inspection_query_basis),
            required_host_capability: required_host_capability.map(inspection_host_capability),
            observed_host_capability: observed_host_capability.map(inspection_host_capability),
        },
        UiObligationDispatchStopPosture::BudgetExceeded {
            budget,
            attempted_lane_cost,
        } => UiInspectionObligationDispatchPosture::BudgetExceeded {
            budget: inspection_budget(budget),
            attempted_lane_cost,
        },
    }
}

pub(super) fn inspection_source(
    source: UiObligationEvidencePrerequisiteSource,
) -> UiInspectionEvidenceSource {
    match source {
        UiObligationEvidencePrerequisiteSource::QueryBasis => {
            UiInspectionEvidenceSource::WorthLocal
        }
        UiObligationEvidencePrerequisiteSource::QueryProjectionConsumption => {
            UiInspectionEvidenceSource::QueryProjectionConsumption
        }
        UiObligationEvidencePrerequisiteSource::QueryInspection => {
            UiInspectionEvidenceSource::QueryInspection
        }
        UiObligationEvidencePrerequisiteSource::QueryCausalExplanation => {
            UiInspectionEvidenceSource::QueryCausalExplanation
        }
        UiObligationEvidencePrerequisiteSource::HostCapability => {
            UiInspectionEvidenceSource::HostCapability
        }
    }
}

pub(super) fn inspection_non_selection_reason(
    reason: UiObligationNonSelectionReason,
) -> UiInspectionObligationNonSelectionReason {
    match reason {
        UiObligationNonSelectionReason::RuleDidNotMatch => {
            UiInspectionObligationNonSelectionReason::RuleDidNotMatch
        }
        UiObligationNonSelectionReason::FamilyUnavailable => {
            UiInspectionObligationNonSelectionReason::FamilyUnavailable
        }
        UiObligationNonSelectionReason::WrongWorld => {
            UiInspectionObligationNonSelectionReason::WrongWorld
        }
    }
}

pub(super) fn inspection_legality_reason(
    reason: UiObligationLegalityReasonEvidence,
) -> UiInspectionObligationLegalityReason {
    match reason {
        UiObligationLegalityReasonEvidence::MissingDeclarationArtifact => {
            UiInspectionObligationLegalityReason::MissingDeclarationArtifact
        }
        UiObligationLegalityReasonEvidence::MissingQueryPrerequisiteEvidence => {
            UiInspectionObligationLegalityReason::MissingQueryPrerequisiteEvidence
        }
        UiObligationLegalityReasonEvidence::MissingHostCapabilityReport => {
            UiInspectionObligationLegalityReason::MissingHostCapabilityReport
        }
        UiObligationLegalityReasonEvidence::QueryBindingRequiresLaterRuntimeLane => {
            UiInspectionObligationLegalityReason::QueryBindingRequiresLaterRuntimeLane
        }
        UiObligationLegalityReasonEvidence::ServiceUsageRequiresLaterRuntimeLane => {
            UiInspectionObligationLegalityReason::ServiceUsageRequiresLaterRuntimeLane
        }
        UiObligationLegalityReasonEvidence::WrongQueryBasis { required, observed } => {
            UiInspectionObligationLegalityReason::WrongQueryBasis {
                required: inspection_query_basis(required),
                observed: inspection_query_basis(observed),
            }
        }
        UiObligationLegalityReasonEvidence::WrongHostCapability { required, observed } => {
            UiInspectionObligationLegalityReason::WrongHostCapability {
                required: inspection_host_capability(required),
                observed: inspection_host_capability(observed),
            }
        }
        UiObligationLegalityReasonEvidence::Stale {
            required,
            observed,
            evidence,
        } => UiInspectionObligationLegalityReason::Stale {
            required: inspection_query_basis(required),
            observed: inspection_query_basis(observed),
            evidence: inspection_stale_evidence(evidence),
        },
        UiObligationLegalityReasonEvidence::Ambiguous {
            required_query_basis,
            observed_query_basis,
            required_host_capability,
            observed_host_capability,
        } => UiInspectionObligationLegalityReason::Ambiguous {
            required_query_basis: required_query_basis.map(inspection_query_basis),
            observed_query_basis: observed_query_basis.map(inspection_query_basis),
            required_host_capability: required_host_capability.map(inspection_host_capability),
            observed_host_capability: observed_host_capability.map(inspection_host_capability),
        },
        UiObligationLegalityReasonEvidence::RebindRequired { required, observed } => {
            UiInspectionObligationLegalityReason::RebindRequired {
                required: inspection_query_basis(required),
                observed: inspection_query_basis(observed),
            }
        }
        UiObligationLegalityReasonEvidence::BudgetExceeded {
            budget,
            attempted_lane_cost,
        } => UiInspectionObligationLegalityReason::BudgetExceeded {
            budget: inspection_budget(budget),
            attempted_lane_cost,
        },
    }
}
