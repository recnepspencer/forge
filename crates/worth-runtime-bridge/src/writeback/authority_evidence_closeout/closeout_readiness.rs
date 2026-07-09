#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeAuthorityEvidenceReadyCapability {
    QueryFacingContractCarriesTargetCausalityProvenanceNamingContinuity,
    BatchSessionBundlesPreserveAggregateEvidenceDigests,
    ReplaySafeRequestReceiptDigestsCarriedForward,
}

impl BridgeAuthorityEvidenceReadyCapability {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::QueryFacingContractCarriesTargetCausalityProvenanceNamingContinuity => {
                "query-facing-contract-carries-target-causality-provenance-naming-continuity"
            }
            Self::BatchSessionBundlesPreserveAggregateEvidenceDigests => {
                "batch-session-bundles-preserve-aggregate-evidence-digests"
            }
            Self::ReplaySafeRequestReceiptDigestsCarriedForward => {
                "replay-safe-request-receipt-digests-carried-forward"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeAuthorityEvidenceDeferredBoundary {
    DurableRestartTemporalAsyncAuthorityMutationSemantics,
    UnsupportedMutationFamiliesRemainFailClosed,
    DownstreamDomainsCannotReconstructDroppedCausalityProvenance,
}

impl BridgeAuthorityEvidenceDeferredBoundary {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::DurableRestartTemporalAsyncAuthorityMutationSemantics => {
                "durable-restart-temporal-async-authority-mutation-semantics"
            }
            Self::UnsupportedMutationFamiliesRemainFailClosed => {
                "unsupported-mutation-families-remain-fail-closed"
            }
            Self::DownstreamDomainsCannotReconstructDroppedCausalityProvenance => {
                "downstream-domains-cannot-reconstruct-dropped-causality-provenance"
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BridgeAuthorityEvidenceVerificationGate {
    FormatRuntimeBridge,
    CheckRuntimeBridgeTests,
    FocusedRuntimeBridgeWritebackTests,
    PhaseBoundaryCompileFail,
    DiffWhitespace,
}

impl BridgeAuthorityEvidenceVerificationGate {
    pub(super) const fn digest_entry(self) -> &'static str {
        match self {
            Self::FormatRuntimeBridge => "format-runtime-bridge",
            Self::CheckRuntimeBridgeTests => "check-runtime-bridge-tests",
            Self::FocusedRuntimeBridgeWritebackTests => "focused-runtime-bridge-writeback-tests",
            Self::PhaseBoundaryCompileFail => "phase-boundary-compile-fail",
            Self::DiffWhitespace => "diff-whitespace",
        }
    }
}

pub(super) fn standard_ready_capabilities() -> Vec<BridgeAuthorityEvidenceReadyCapability> {
    vec![
        BridgeAuthorityEvidenceReadyCapability::QueryFacingContractCarriesTargetCausalityProvenanceNamingContinuity,
        BridgeAuthorityEvidenceReadyCapability::BatchSessionBundlesPreserveAggregateEvidenceDigests,
        BridgeAuthorityEvidenceReadyCapability::ReplaySafeRequestReceiptDigestsCarriedForward,
    ]
}

pub(super) fn standard_deferred_boundaries() -> Vec<BridgeAuthorityEvidenceDeferredBoundary> {
    vec![
        BridgeAuthorityEvidenceDeferredBoundary::DurableRestartTemporalAsyncAuthorityMutationSemantics,
        BridgeAuthorityEvidenceDeferredBoundary::UnsupportedMutationFamiliesRemainFailClosed,
        BridgeAuthorityEvidenceDeferredBoundary::DownstreamDomainsCannotReconstructDroppedCausalityProvenance,
    ]
}

pub(super) fn standard_verification_gates() -> Vec<BridgeAuthorityEvidenceVerificationGate> {
    vec![
        BridgeAuthorityEvidenceVerificationGate::FormatRuntimeBridge,
        BridgeAuthorityEvidenceVerificationGate::CheckRuntimeBridgeTests,
        BridgeAuthorityEvidenceVerificationGate::FocusedRuntimeBridgeWritebackTests,
        BridgeAuthorityEvidenceVerificationGate::PhaseBoundaryCompileFail,
        BridgeAuthorityEvidenceVerificationGate::DiffWhitespace,
    ]
}
