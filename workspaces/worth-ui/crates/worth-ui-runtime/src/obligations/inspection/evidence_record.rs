use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
    UiAdmissionStaleEvidence,
};
use crate::obligations::catalog::UiObligationFamily;
use crate::obligations::selection::UiObligationSelectionReason;
use worth_ui_inspection::UiInspectionObligationReasonProjection;

use super::projection_mapping::{
    inspection_decision, inspection_denial_posture, inspection_family, inspection_legality_reason,
    inspection_non_selection_reason, inspection_source,
};
use super::selection_reason_mapping::inspection_selection_reason;
use super::UiObligationEvidenceHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidenceDecision {
    Selected,
    NotSelected,
    Verdict,
    Admission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationNonSelectionReason {
    RuleDidNotMatch,
    FamilyUnavailable,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidenceDenialPosture {
    Unsupported,
    Deferred,
    DiagnosticOnly,
    WrongWorld,
    WrongQueryBasis {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
    },
    WrongHostCapability {
        required: UiAdmissionHostCapability,
        observed: UiAdmissionHostCapability,
    },
    Stale {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
        evidence: UiAdmissionStaleEvidence,
    },
    Ambiguous {
        required_query_basis: Option<UiAdmissionQueryBasis>,
        observed_query_basis: Option<UiAdmissionQueryBasis>,
        required_host_capability: Option<UiAdmissionHostCapability>,
        observed_host_capability: Option<UiAdmissionHostCapability>,
    },
    BudgetExceeded {
        budget: UiAdmissionSelectionBudget,
        attempted_lane_cost: u8,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidencePrerequisiteSource {
    QueryBasis,
    QueryProjectionConsumption,
    QueryInspection,
    QueryCausalExplanation,
    HostCapability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationLegalityReasonEvidence {
    MissingDeclarationArtifact,
    MissingQueryPrerequisiteEvidence,
    MissingHostCapabilityReport,
    QueryBindingRequiresLaterRuntimeLane,
    ServiceUsageRequiresLaterRuntimeLane,
    WrongQueryBasis {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
    },
    WrongHostCapability {
        required: UiAdmissionHostCapability,
        observed: UiAdmissionHostCapability,
    },
    Stale {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
        evidence: UiAdmissionStaleEvidence,
    },
    Ambiguous {
        required_query_basis: Option<UiAdmissionQueryBasis>,
        observed_query_basis: Option<UiAdmissionQueryBasis>,
        required_host_capability: Option<UiAdmissionHostCapability>,
        observed_host_capability: Option<UiAdmissionHostCapability>,
    },
    RebindRequired {
        required: UiAdmissionQueryBasis,
        observed: UiAdmissionQueryBasis,
    },
    BudgetExceeded {
        budget: UiAdmissionSelectionBudget,
        attempted_lane_cost: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiObligationEvidenceRecord {
    handle: UiObligationEvidenceHandle,
    graph_node_digest: u64,
    touch_identity_digest: Option<u64>,
    family: Option<UiObligationFamily>,
    decision: UiObligationEvidenceDecision,
    denial_posture: Option<UiObligationEvidenceDenialPosture>,
    selection_reasons: Box<[UiObligationSelectionReason]>,
    prerequisite_sources: Box<[UiObligationEvidencePrerequisiteSource]>,
    non_selection_reason: Option<UiObligationNonSelectionReason>,
    legality_reason: Option<UiObligationLegalityReasonEvidence>,
}

impl UiObligationEvidenceRecord {
    pub(crate) fn new(
        handle: UiObligationEvidenceHandle,
        graph_node_digest: u64,
        touch_identity_digest: Option<u64>,
        family: Option<UiObligationFamily>,
        decision: UiObligationEvidenceDecision,
        denial_posture: Option<UiObligationEvidenceDenialPosture>,
        selection_reasons: Box<[UiObligationSelectionReason]>,
        prerequisite_sources: Box<[UiObligationEvidencePrerequisiteSource]>,
        non_selection_reason: Option<UiObligationNonSelectionReason>,
        legality_reason: Option<UiObligationLegalityReasonEvidence>,
    ) -> Self {
        Self {
            handle,
            graph_node_digest,
            touch_identity_digest,
            family,
            decision,
            denial_posture,
            selection_reasons,
            prerequisite_sources,
            non_selection_reason,
            legality_reason,
        }
    }

    pub fn handle(&self) -> UiObligationEvidenceHandle {
        self.handle
    }

    pub fn graph_node_digest(&self) -> u64 {
        self.graph_node_digest
    }

    pub fn touch_identity_digest(&self) -> Option<u64> {
        self.touch_identity_digest
    }

    pub fn family(&self) -> Option<UiObligationFamily> {
        self.family
    }

    pub fn decision(&self) -> UiObligationEvidenceDecision {
        self.decision
    }

    pub fn denial_posture(&self) -> Option<UiObligationEvidenceDenialPosture> {
        self.denial_posture
    }

    pub fn selection_reasons(&self) -> &[UiObligationSelectionReason] {
        &self.selection_reasons
    }

    pub fn prerequisite_sources(&self) -> &[UiObligationEvidencePrerequisiteSource] {
        &self.prerequisite_sources
    }

    pub fn non_selection_reason(&self) -> Option<UiObligationNonSelectionReason> {
        self.non_selection_reason
    }

    pub fn legality_reason(&self) -> Option<UiObligationLegalityReasonEvidence> {
        self.legality_reason
    }

    pub(crate) fn to_projection(&self) -> UiInspectionObligationReasonProjection {
        UiInspectionObligationReasonProjection::new(
            self.handle.digest(),
            self.graph_node_digest,
            self.touch_identity_digest,
            self.family.map(inspection_family),
            inspection_decision(self.decision),
            self.denial_posture.map(inspection_denial_posture),
            self.selection_reasons
                .iter()
                .copied()
                .map(inspection_selection_reason)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            self.prerequisite_sources
                .iter()
                .copied()
                .map(inspection_source)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            self.non_selection_reason
                .map(inspection_non_selection_reason),
            self.legality_reason.map(inspection_legality_reason),
        )
    }
}
