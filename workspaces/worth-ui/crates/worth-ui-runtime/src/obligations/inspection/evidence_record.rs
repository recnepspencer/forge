use crate::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
    UiAdmissionStaleEvidence,
};
use crate::evidence::{
    evidence_ref, UiEvidenceFamily, UiEvidenceRef, UiInspectionObligationReasonProjection,
};
use crate::obligations::catalog::UiObligationFamily;
use crate::obligations::selection::UiObligationSelectionReason;
use crate::obligations::verdict::{UiObligationDispatchStopPosture, UiObligationVerdictClass};
use worth_ui_inspection::{
    UiEvidenceAuthorityGeneration, UiEvidenceMaterializationPosture, UiEvidenceRetentionPosture,
};
use worth_ui_query_binding::compatibility::managed_live::WorthUiQueryPrerequisiteEvidence;

use super::projection_mapping::{
    inspection_decision, inspection_denial_posture, inspection_dispatch_posture, inspection_family,
    inspection_legality_reason, inspection_non_selection_reason, inspection_source,
    inspection_verdict_class, inspection_verdict_posture,
};
use super::selection_reason_mapping::inspection_selection_reason;
use super::{UiObligationEvidenceAuthoritySource, UiObligationEvidenceHandle};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidenceDecision {
    Selected,
    NotSelected,
    Dispatch,
    Verdict,
    Admission,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiObligationEvidenceDispatchPosture {
    ImmediateCheck,
    TypedStop(UiObligationDispatchStopPosture),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObligationEvidenceVerdictPosture {
    class: UiObligationVerdictClass,
    stop_posture: UiObligationDispatchStopPosture,
}

impl UiObligationEvidenceVerdictPosture {
    pub const fn new(
        class: UiObligationVerdictClass,
        stop_posture: UiObligationDispatchStopPosture,
    ) -> Self {
        Self {
            class,
            stop_posture,
        }
    }

    pub const fn class(self) -> UiObligationVerdictClass {
        self.class
    }

    pub const fn stop_posture(self) -> UiObligationDispatchStopPosture {
        self.stop_posture
    }
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
    authority_source: UiObligationEvidenceAuthoritySource,
    authority_digest: u64,
    graph_node_digest: u64,
    touch_identity_digest: Option<u64>,
    family: Option<UiObligationFamily>,
    decision: UiObligationEvidenceDecision,
    dispatch_posture: Option<UiObligationEvidenceDispatchPosture>,
    verdict_posture: Option<UiObligationEvidenceVerdictPosture>,
    denial_posture: Option<UiObligationEvidenceDenialPosture>,
    selection_reasons: Box<[UiObligationSelectionReason]>,
    prerequisite_sources: Box<[UiObligationEvidencePrerequisiteSource]>,
    query_prerequisite_evidence: Box<[WorthUiQueryPrerequisiteEvidence]>,
    non_selection_reason: Option<UiObligationNonSelectionReason>,
    legality_reason: Option<UiObligationLegalityReasonEvidence>,
}

pub(crate) struct UiObligationEvidenceRecordInput {
    pub handle: UiObligationEvidenceHandle,
    pub authority_source: UiObligationEvidenceAuthoritySource,
    pub authority_digest: u64,
    pub graph_node_digest: u64,
    pub touch_identity_digest: Option<u64>,
    pub family: Option<UiObligationFamily>,
    pub decision: UiObligationEvidenceDecision,
    pub dispatch_posture: Option<UiObligationEvidenceDispatchPosture>,
    pub verdict_posture: Option<UiObligationEvidenceVerdictPosture>,
    pub denial_posture: Option<UiObligationEvidenceDenialPosture>,
    pub selection_reasons: Box<[UiObligationSelectionReason]>,
    pub prerequisite_sources: Box<[UiObligationEvidencePrerequisiteSource]>,
    pub query_prerequisite_evidence: Box<[WorthUiQueryPrerequisiteEvidence]>,
    pub non_selection_reason: Option<UiObligationNonSelectionReason>,
    pub legality_reason: Option<UiObligationLegalityReasonEvidence>,
}

impl UiObligationEvidenceRecord {
    pub(crate) fn new(input: UiObligationEvidenceRecordInput) -> Self {
        let UiObligationEvidenceRecordInput {
            handle,
            authority_source,
            authority_digest,
            graph_node_digest,
            touch_identity_digest,
            family,
            decision,
            dispatch_posture,
            verdict_posture,
            denial_posture,
            selection_reasons,
            prerequisite_sources,
            query_prerequisite_evidence,
            non_selection_reason,
            legality_reason,
        } = input;
        Self {
            handle,
            authority_source,
            authority_digest,
            graph_node_digest,
            touch_identity_digest,
            family,
            decision,
            dispatch_posture,
            verdict_posture,
            denial_posture,
            selection_reasons,
            prerequisite_sources,
            query_prerequisite_evidence,
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

    pub fn evidence_ref(
        &self,
        authority_generation: UiEvidenceAuthorityGeneration,
    ) -> UiEvidenceRef {
        evidence_ref(
            UiEvidenceFamily::Obligation,
            self.handle.public_identity(),
            self.authority_source
                .into_public_binding(self.authority_digest, authority_generation),
            UiEvidenceMaterializationPosture::DetailAvailable,
            UiEvidenceRetentionPosture::CurrentGenerationOnly,
            self.handle.public_handle(),
        )
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

    pub fn dispatch_posture(&self) -> Option<UiObligationEvidenceDispatchPosture> {
        self.dispatch_posture
    }

    pub fn verdict_posture(&self) -> Option<UiObligationEvidenceVerdictPosture> {
        self.verdict_posture
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

    pub fn query_prerequisite_evidence(&self) -> &[WorthUiQueryPrerequisiteEvidence] {
        &self.query_prerequisite_evidence
    }

    pub fn non_selection_reason(&self) -> Option<UiObligationNonSelectionReason> {
        self.non_selection_reason
    }

    pub fn legality_reason(&self) -> Option<UiObligationLegalityReasonEvidence> {
        self.legality_reason
    }

    pub(crate) fn to_projection(&self) -> UiInspectionObligationReasonProjection {
        UiInspectionObligationReasonProjection::new(
            crate::evidence::UiInspectionObligationReasonProjectionInput {
                handle_digest: self.handle.digest(),
                graph_node_digest: self.graph_node_digest,
                touch_identity_digest: self.touch_identity_digest,
                family: self.family.map(inspection_family),
                decision: inspection_decision(self.decision),
                dispatch_posture: self.dispatch_posture.map(inspection_dispatch_posture),
                verdict_class: self
                    .verdict_posture
                    .map(|posture| inspection_verdict_class(posture.class())),
                verdict_posture: self
                    .verdict_posture
                    .map(|posture| inspection_verdict_posture(posture.stop_posture())),
                denial_posture: self.denial_posture.map(inspection_denial_posture),
                selection_reasons: self
                    .selection_reasons
                    .iter()
                    .copied()
                    .map(inspection_selection_reason)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                prerequisite_sources: self
                    .prerequisite_sources
                    .iter()
                    .copied()
                    .map(inspection_source)
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                non_selection_reason: self
                    .non_selection_reason
                    .map(inspection_non_selection_reason),
                legality_reason: self.legality_reason.map(inspection_legality_reason),
            },
        )
    }
}
