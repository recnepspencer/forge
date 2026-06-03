pub(in crate::harness::tests::pricing_shock) use super::super::pricing_domain::{
    MaterialPriceAttribution, MaterialTickWave, PricingCommitAttribution, PricingDomainWorld,
    PricingMaterial, ProductPriceBreakdown,
};
pub(in crate::harness::tests::pricing_shock) use super::super::pricing_support::*;
pub(in crate::harness::tests::pricing_shock) use crate::adapter::{
    TruthWritebackAuthority, TruthWritebackAuthorityError, TruthWritebackReceipt,
    TruthWritebackRequest,
};
pub(in crate::harness::tests::pricing_shock) use crate::error::BridgeDeliveryErrorKind;
pub(in crate::harness::tests::pricing_shock) use crate::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeCommittedPatchEnvelope,
    BridgeCommittedPatchItem, BridgeDiagnosticsTier, BridgeExecutionPolicyClass,
    BridgeFailureClass, BridgeMappingId, BridgeMappingRegistration, BridgeMergeAuthorityBasis,
    BridgeMergeAuthorityBasisKind, BridgeMergeConsumptionClass, BridgeMergeOntologyMappingSurface,
    BridgeMergeParentOrderProof, BridgeMergeStructuralAdvisoryDisposition, BridgePolicyDeclaration,
    BridgePolicyDeclarationIdentity, BridgePreviewResidueClass, BridgePreviewSessionDeclaration,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, BridgeReplayErrorKind,
    BridgeRequestKind, BridgeRuntimePolicy, BridgeSignalBranchIdentity, BridgeSourceCapability,
    BridgeSourceCapabilitySet, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, BridgeSpeculativeSessionRequest,
    BridgeStandardRouteError, BridgeTruthViewEvaluationRequest, BridgeTruthViewSelector,
    BridgeWritebackCausalityBasis, BridgeWritebackCausalityIdentity, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIdentity,
    BridgeWritebackEffectIntent, BridgeWritebackErrorKind, BridgeWritebackFailureClass,
    BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass, BridgeWritebackIdempotenceIdentity,
    BridgeWritebackRequestMode, BridgeWritebackStrategyClass, CoarseRoutingMode,
    FineGrainedMatchStatus, MappingSelector, MergeHistoryDeclaration,
    MergeHistoryDeclarationIdentity, RuntimeBridge, RuntimeBridgeBuilder, SignalInvalidationScope,
    SliceWideningPolicy, SnapshotReadRecord, SubscriptionSliceKind, TruthBranchIdentity,
    TruthCommitIdentity, TruthDeltaSurfaceKind, TruthPatchIdentity, TruthPatchScope,
    TruthSnapshotIdentity,
};
pub(in crate::harness::tests::pricing_shock) use crate::harness::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink,
    RecordingTruthWritebackAuthority, SnapshotFixture,
};
pub(in crate::harness::tests::pricing_shock) use crate::snapshot::{
    SnapshotReadPacket, SnapshotReadRequest,
};
pub(in crate::harness::tests::pricing_shock) use crate::source::{
    SourceDeclaration, SourceDeclarationIdentity,
};
pub(in crate::harness::tests::pricing_shock) use crate::speculation::BridgePreviewLifecycleStateKind;
pub(in crate::harness::tests::pricing_shock) use forge_foundational::facade::{
    AspectKey, AspectValue,
};
pub(in crate::harness::tests::pricing_shock) use forge_harness::facade::{
    ExecutionProfile, FeedStreamEventKind, FeedVolatilityRegime, ScenarioPlan,
};
pub(in crate::harness::tests::pricing_shock) use std::collections::BTreeMap;

mod capture_failures;
mod capture_lifecycle;
mod capture_reference;
mod historical_portfolio;
mod pricing_snapshot_records;
mod provenance_records;
mod route_and_runtime;
mod scenario;
mod simulation_capture;
mod simulation_inputs;
mod source_fixtures;
mod writeback_authority;
mod writeback_merge_capture;

pub(in crate::harness::tests::pricing_shock) use capture_failures::*;
pub(in crate::harness::tests::pricing_shock) use capture_lifecycle::*;
pub(in crate::harness::tests::pricing_shock) use capture_reference::*;
pub(in crate::harness::tests::pricing_shock) use historical_portfolio::*;
pub(in crate::harness::tests::pricing_shock) use pricing_snapshot_records::*;
pub(in crate::harness::tests::pricing_shock) use provenance_records::*;
pub(in crate::harness::tests::pricing_shock) use route_and_runtime::*;
pub(in crate::harness::tests::pricing_shock) use scenario::*;
pub(in crate::harness::tests::pricing_shock) use simulation_capture::*;
pub(in crate::harness::tests::pricing_shock) use simulation_inputs::*;
pub(in crate::harness::tests::pricing_shock) use source_fixtures::*;
pub(in crate::harness::tests::pricing_shock) use writeback_authority::*;
pub(in crate::harness::tests::pricing_shock) use writeback_merge_capture::*;
