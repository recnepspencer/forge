#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub enum ForgeQueryOrchestrationSurfaceFamily {
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

impl ForgeQueryOrchestrationSurfaceFamily {
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
pub enum ForgeQueryOrchestrationSurfaceVisibility {
    Ordinary,
    OrdinaryOutcome,
    Checked,
    ProofVisible,
}

impl ForgeQueryOrchestrationSurfaceVisibility {
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
pub enum ForgeQueryOrchestrationTranscriptFamily {
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

impl ForgeQueryOrchestrationTranscriptFamily {
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
pub enum ForgeQueryOrchestrationSupportSurface {
    DeclarationEntryCrossingInventory,
    DeclarationEntryReadiness,
    ContinuationPreparedContract,
    SignalCompatibilityOrchestration,
    ContributionComposedOrchestration,
    RecoveryBoundary,
}

impl ForgeQueryOrchestrationSupportSurface {
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
pub enum ForgeQueryOrchestrationCheckedTopologyKind {
    DeclarationEntryStage,
    Continuation,
    SignalCompatibilityOrchestration,
    ContributionComposed,
    RecoveryBoundary,
}

impl ForgeQueryOrchestrationCheckedTopologyKind {
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
pub enum ForgeQueryOrchestrationBindingProjection {
    None,
    SharedContinuationBinding,
    SharedSignalCompatibilityBinding,
    SharedContributionBinding,
    SharedGroupedBinding,
}

impl ForgeQueryOrchestrationBindingProjection {
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
