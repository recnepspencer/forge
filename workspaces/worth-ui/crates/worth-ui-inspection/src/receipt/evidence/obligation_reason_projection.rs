use crate::UiInspectionEvidenceSource;

use super::UiInspectionObligationDecision;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationFamily {
    StructuralLegality,
    ParticipationLegality,
    SlotContract,
    MeasurementRequirement,
    QueryBindingRequirement,
    IntentOperabilityRequirement,
    PortalHostRequirement,
    FocusRouteRequirement,
    MotionSupportRequirement,
    AccessibilityRequirement,
    HostCapabilityRequirement,
    DiagnosticSurfaceRequirement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionTouchTargetClass {
    Node,
    SlotOccupancy,
    PageMembership,
    RegionMembership,
    MosaicMembership,
    AttachmentLane,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionTouchOriginClass {
    DeclarationChange,
    QueryFactChange,
    HostObservation,
    ServiceEvent,
    IntentSubmission,
    DiagnosticOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionTouchRuntimeLane {
    Structural,
    Participation,
    Measurement,
    QueryBinding,
    IntentOperability,
    Service,
    HostCapability,
    Diagnostic,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionTouchAspectPosture {
    Read,
    Written,
    Invalidated,
    Preserved,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationWorldProfileClass {
    Authoritative,
    Preview,
    Branch,
    HotReloadCandidate,
    Diagnostic,
    HostObservation,
    TestCertification,
    QuerySnapshotBasis,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationSupportSelectionPosture {
    Supported,
    Unsupported,
    Deferred,
    DiagnosticOnly,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionSupportRowSchemaKind {
    QueryBinding,
    ServiceUsage,
    TouchMeaning,
    MeasurementPolicy,
    HostCapability,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionAdmissionQueryBasis {
    GraphAligned,
    WrongWorldProjection,
    RebindRequired,
    StaleReceipt,
    AmbiguousSources,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionAdmissionHostCapability {
    Available,
    Missing,
    Ambiguous,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionAdmissionStaleEvidence {
    DeclarationArtifactMissing,
    QueryReceiptExpired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationSelectionReason {
    TouchTargetClass(UiInspectionTouchTargetClass),
    TouchOriginClass(UiInspectionTouchOriginClass),
    TouchRuntimeLane(UiInspectionTouchRuntimeLane),
    TouchAspectPosture(UiInspectionTouchAspectPosture),
    WorldProfile(UiInspectionObligationWorldProfileClass),
    SupportPosture(UiInspectionObligationSupportSelectionPosture),
    SupportRow(UiInspectionSupportRowSchemaKind),
    QueryBasis(UiInspectionAdmissionQueryBasis),
    HostCapability(UiInspectionAdmissionHostCapability),
    GraphQueryBindingAttachment,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationNonSelectionReason {
    RuleDidNotMatch,
    FamilyUnavailable,
    WrongWorld,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionSelectionBudget {
    Unbounded,
    OrdinaryLaneBudget { lane_limit: u8 },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationDenialPosture {
    Unsupported,
    Deferred,
    DiagnosticOnly,
    WrongWorld,
    WrongQueryBasis {
        required: UiInspectionAdmissionQueryBasis,
        observed: UiInspectionAdmissionQueryBasis,
    },
    WrongHostCapability {
        required: UiInspectionAdmissionHostCapability,
        observed: UiInspectionAdmissionHostCapability,
    },
    Stale {
        required: UiInspectionAdmissionQueryBasis,
        observed: UiInspectionAdmissionQueryBasis,
        evidence: UiInspectionAdmissionStaleEvidence,
    },
    Ambiguous {
        required_query_basis: Option<UiInspectionAdmissionQueryBasis>,
        observed_query_basis: Option<UiInspectionAdmissionQueryBasis>,
        required_host_capability: Option<UiInspectionAdmissionHostCapability>,
        observed_host_capability: Option<UiInspectionAdmissionHostCapability>,
    },
    BudgetExceeded {
        budget: UiInspectionSelectionBudget,
        attempted_lane_cost: u8,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationLegalityReason {
    MissingDeclarationArtifact,
    MissingQueryPrerequisiteEvidence,
    MissingHostCapabilityReport,
    QueryBindingRequiresLaterRuntimeLane,
    ServiceUsageRequiresLaterRuntimeLane,
    WrongQueryBasis {
        required: UiInspectionAdmissionQueryBasis,
        observed: UiInspectionAdmissionQueryBasis,
    },
    WrongHostCapability {
        required: UiInspectionAdmissionHostCapability,
        observed: UiInspectionAdmissionHostCapability,
    },
    Stale {
        required: UiInspectionAdmissionQueryBasis,
        observed: UiInspectionAdmissionQueryBasis,
        evidence: UiInspectionAdmissionStaleEvidence,
    },
    Ambiguous {
        required_query_basis: Option<UiInspectionAdmissionQueryBasis>,
        observed_query_basis: Option<UiInspectionAdmissionQueryBasis>,
        required_host_capability: Option<UiInspectionAdmissionHostCapability>,
        observed_host_capability: Option<UiInspectionAdmissionHostCapability>,
    },
    RebindRequired {
        required: UiInspectionAdmissionQueryBasis,
        observed: UiInspectionAdmissionQueryBasis,
    },
    BudgetExceeded {
        budget: UiInspectionSelectionBudget,
        attempted_lane_cost: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionObligationReasonProjection {
    handle_digest: u64,
    graph_node_digest: u64,
    touch_identity_digest: Option<u64>,
    family: Option<UiInspectionObligationFamily>,
    decision: UiInspectionObligationDecision,
    denial_posture: Option<UiInspectionObligationDenialPosture>,
    selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
    prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
    non_selection_reason: Option<UiInspectionObligationNonSelectionReason>,
    legality_reason: Option<UiInspectionObligationLegalityReason>,
}

impl UiInspectionObligationReasonProjection {
    pub fn new(
        handle_digest: u64,
        graph_node_digest: u64,
        touch_identity_digest: Option<u64>,
        family: Option<UiInspectionObligationFamily>,
        decision: UiInspectionObligationDecision,
        denial_posture: Option<UiInspectionObligationDenialPosture>,
        selection_reasons: Box<[UiInspectionObligationSelectionReason]>,
        prerequisite_sources: Box<[UiInspectionEvidenceSource]>,
        non_selection_reason: Option<UiInspectionObligationNonSelectionReason>,
        legality_reason: Option<UiInspectionObligationLegalityReason>,
    ) -> Self {
        Self {
            handle_digest,
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

    pub fn handle_digest(&self) -> u64 {
        self.handle_digest
    }

    pub fn graph_node_digest(&self) -> u64 {
        self.graph_node_digest
    }

    pub fn touch_identity_digest(&self) -> Option<u64> {
        self.touch_identity_digest
    }

    pub fn family(&self) -> Option<UiInspectionObligationFamily> {
        self.family
    }

    pub fn decision(&self) -> UiInspectionObligationDecision {
        self.decision
    }

    pub fn denial_posture(&self) -> Option<UiInspectionObligationDenialPosture> {
        self.denial_posture
    }

    pub fn selection_reasons(&self) -> &[UiInspectionObligationSelectionReason] {
        &self.selection_reasons
    }

    pub fn prerequisite_sources(&self) -> &[UiInspectionEvidenceSource] {
        &self.prerequisite_sources
    }

    pub fn non_selection_reason(&self) -> Option<UiInspectionObligationNonSelectionReason> {
        self.non_selection_reason
    }

    pub fn legality_reason(&self) -> Option<UiInspectionObligationLegalityReason> {
        self.legality_reason
    }
}
