#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationSurfaceFamily {
    DeclarationEntry,
    RouteFromProgressed,
    ReceiptFromProgressed,
    EnvelopeFromProgressed,
    ContinuationPrepareTarget,
    ContinuationPrepareContext,
    ContinuationExecute,
    SignalCompatibilityOrchestration,
    ContributionComposedOrchestration,
    GroupedNeighborhoodOrchestration,
    RecoveryBoundary,
}

impl WorthQueryOrchestrationSurfaceFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationEntry => "declaration_entry",
            Self::RouteFromProgressed => "route_from_progressed",
            Self::ReceiptFromProgressed => "receipt_from_progressed",
            Self::EnvelopeFromProgressed => "envelope_from_progressed",
            Self::ContinuationPrepareTarget => "continuation_prepare_target",
            Self::ContinuationPrepareContext => "continuation_prepare_context",
            Self::ContinuationExecute => "continuation_execute",
            Self::SignalCompatibilityOrchestration => "signal_compatibility_orchestration",
            Self::ContributionComposedOrchestration => "contribution_composed_orchestration",
            Self::GroupedNeighborhoodOrchestration => "grouped_neighborhood_orchestration",
            Self::RecoveryBoundary => "recovery_boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationSurfaceVisibility {
    Ordinary,
    OrdinaryOutcome,
    Checked,
    ProofVisible,
}

impl WorthQueryOrchestrationSurfaceVisibility {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::OrdinaryOutcome => "ordinary_outcome",
            Self::Checked => "checked",
            Self::ProofVisible => "proof_visible",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationTranscriptFamily {
    DeclarationEntry,
    DeclarationRoute,
    DeclarationReceipt,
    DeclarationEnvelope,
    PreparedContinuation,
    ContinuationExecution,
    SignalCompatibilityOrchestration,
    ContributionComposedOrchestration,
    GroupedNeighborhoodOrchestration,
    RecoveryBoundary,
}

impl WorthQueryOrchestrationTranscriptFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationEntry => "declaration_entry",
            Self::DeclarationRoute => "declaration_route",
            Self::DeclarationReceipt => "declaration_receipt",
            Self::DeclarationEnvelope => "declaration_envelope",
            Self::PreparedContinuation => "prepared_continuation",
            Self::ContinuationExecution => "continuation_execution",
            Self::SignalCompatibilityOrchestration => "signal_compatibility_orchestration",
            Self::ContributionComposedOrchestration => "contribution_composed_orchestration",
            Self::GroupedNeighborhoodOrchestration => "grouped_neighborhood_orchestration",
            Self::RecoveryBoundary => "recovery_boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationSupportSurface {
    DeclarationEntryCrossingInventory,
    DeclarationEntryReadiness,
    ContinuationPreparedContract,
    SignalCompatibilityOrchestration,
    ContributionComposedOrchestration,
    RecoveryBoundary,
}

impl WorthQueryOrchestrationSupportSurface {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationEntryCrossingInventory => "declaration_entry_crossing_inventory",
            Self::DeclarationEntryReadiness => "declaration_entry_readiness",
            Self::ContinuationPreparedContract => "continuation_prepared_contract",
            Self::SignalCompatibilityOrchestration => "signal_compatibility_orchestration",
            Self::ContributionComposedOrchestration => "contribution_composed_orchestration",
            Self::RecoveryBoundary => "recovery_boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationCheckedTopologyKind {
    DeclarationEntryStage,
    Continuation,
    SignalCompatibilityOrchestration,
    ContributionComposed,
    RecoveryBoundary,
}

impl WorthQueryOrchestrationCheckedTopologyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclarationEntryStage => "declaration_entry_stage",
            Self::Continuation => "continuation",
            Self::SignalCompatibilityOrchestration => "signal_compatibility_orchestration",
            Self::ContributionComposed => "contribution_composed",
            Self::RecoveryBoundary => "recovery_boundary",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum WorthQueryOrchestrationBindingProjection {
    None,
    SharedContinuationBinding,
    SharedSignalCompatibilityBinding,
    SharedContributionBinding,
    SharedGroupedBinding,
}

impl WorthQueryOrchestrationBindingProjection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SharedContinuationBinding => "shared_continuation_binding",
            Self::SharedSignalCompatibilityBinding => "shared_signal_compatibility_binding",
            Self::SharedContributionBinding => "shared_contribution_binding",
            Self::SharedGroupedBinding => "shared_grouped_binding",
        }
    }
}
