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
    QueryBindingChange,
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
    SettledQueryBinding,
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
pub enum UiInspectionObligationDispatchPosture {
    ImmediateCheck,
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
pub enum UiInspectionObligationVerdictClass {
    Success,
    Advisory,
    Violation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionObligationVerdictPosture {
    None,
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
