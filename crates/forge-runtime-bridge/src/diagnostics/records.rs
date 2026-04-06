use std::sync::Arc;

use crate::input::envelope::{TruthCommitIdentity, TruthPatchIdentity};
use crate::error::BridgeErrorContext;
use crate::mapping::{
    BridgeAspectRegistrationId, BridgeMappingFallbackClass, BridgeMappingId, CoarseRoutingMode,
    SliceFallbackPolicy, SubscriptionSliceKind, TruthDeltaSurfaceKind,
};
use crate::error::{BridgeDeliveryErrorKind, BridgeReplayErrorKind};
use crate::routing::{
    BridgeInvalidationIdentity, BridgeInvalidationTarget, BridgeRouteIdentity, BridgeRoutingCounters,
    BridgeRouteContractProof, BridgeSubscriptionSlice, BridgeSubscriptionSliceIdentity,
    FineGrainedMatchOutcome, FineGrainedMatchStatus,
};
use crate::snapshot::TruthSnapshotIdentity;
use crate::routing::context::BridgeMappingContext;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRecordEntry {
    entity_identity: String,
    aspect_label: String,
    surface_label: String,
    raw_patch_surface_label: String,
    truth_surface_identity: String,
    mapping_id: BridgeMappingId,
    signal_scope: String,
    routing_mode: CoarseRoutingMode,
    fallback_class: Option<BridgeMappingFallbackClass>,
    match_detail: FineGrainedMatchOutcome,
}
pub type BridgeRouteRecordMatch = FineGrainedMatchOutcome;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteSourceRecord {
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
}

impl BridgeRouteSourceRecord {
    pub(crate) fn new(
        source_commit: TruthCommitIdentity,
        source_patch: TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
    ) -> Self {
        Self {
            source_commit,
            source_patch,
            source_snapshot,
        }
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        &self.source_commit
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        &self.source_patch
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRoutingDiagnosticsRecord {
    route_identity: BridgeRouteIdentity,
    entries: Arc<[BridgeRouteRecordEntry]>,
    counters: BridgeRoutingCounters,
}

impl BridgeRoutingDiagnosticsRecord {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        entries: Arc<[BridgeRouteRecordEntry]>,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            route_identity,
            entries,
            counters,
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        &self.route_identity
    }

    pub fn entries(&self) -> &[BridgeRouteRecordEntry] {
        &self.entries
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeLoweringDiagnosticsRecord {
    invalidation_identity: BridgeInvalidationIdentity,
    subscription_slice_identity: BridgeSubscriptionSliceIdentity,
    subscription_slices: Arc<[BridgeSubscriptionSlice]>,
    invalidation_targets: Arc<[BridgeInvalidationTarget]>,
}

impl BridgeLoweringDiagnosticsRecord {
    pub(crate) fn new(
        invalidation_identity: BridgeInvalidationIdentity,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
        subscription_slices: Arc<[BridgeSubscriptionSlice]>,
        invalidation_targets: Arc<[BridgeInvalidationTarget]>,
    ) -> Self {
        Self {
            invalidation_identity,
            subscription_slice_identity,
            subscription_slices,
            invalidation_targets,
        }
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        &self.invalidation_identity
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        &self.subscription_slice_identity
    }

    pub fn subscription_slices(&self) -> &[BridgeSubscriptionSlice] {
        &self.subscription_slices
    }

    pub fn invalidation_targets(&self) -> &[BridgeInvalidationTarget] {
        &self.invalidation_targets
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeContractDiagnosticsRecord {
    contract_proof: BridgeRouteContractProof,
}

impl BridgeContractDiagnosticsRecord {
    pub(crate) fn new(contract_proof: BridgeRouteContractProof) -> Self {
        Self { contract_proof }
    }

    pub fn contract_proof(&self) -> &BridgeRouteContractProof {
        &self.contract_proof
    }

    pub fn mapping_context(&self) -> &BridgeMappingContext {
        self.contract_proof.mapping_context()
    }
}

impl BridgeRouteRecordEntry {
    pub(crate) fn new(
        entity_identity: impl Into<String>,
        aspect_label: impl Into<String>,
        surface_label: impl Into<String>,
        raw_patch_surface_label: impl Into<String>,
        truth_surface_identity: impl Into<String>,
        mapping_id: BridgeMappingId,
        signal_scope: impl Into<String>,
        routing_mode: CoarseRoutingMode,
        fallback_class: Option<BridgeMappingFallbackClass>,
        match_detail: BridgeRouteRecordMatch,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: surface_label.into(),
            raw_patch_surface_label: raw_patch_surface_label.into(),
            truth_surface_identity: truth_surface_identity.into(),
            mapping_id,
            signal_scope: signal_scope.into(),
            routing_mode,
            fallback_class,
            match_detail,
        }
    }

    pub fn entity_identity(&self) -> &str {
        &self.entity_identity
    }

    pub fn aspect_label(&self) -> &str {
        &self.aspect_label
    }

    pub fn surface_label(&self) -> &str {
        &self.surface_label
    }

    pub fn raw_patch_surface_label(&self) -> &str {
        &self.raw_patch_surface_label
    }

    pub fn truth_surface_identity(&self) -> &str {
        &self.truth_surface_identity
    }

    pub fn mapping_id(&self) -> &BridgeMappingId {
        &self.mapping_id
    }

    pub fn signal_scope(&self) -> &str {
        &self.signal_scope
    }

    pub fn routing_mode(&self) -> CoarseRoutingMode {
        self.routing_mode
    }

    pub fn fallback_class(&self) -> Option<BridgeMappingFallbackClass> {
        self.fallback_class
    }

    pub fn truth_surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.match_detail.truth_surface_kind()
    }

    pub fn fine_grained_match_status(&self) -> FineGrainedMatchStatus {
        self.match_detail.status()
    }

    pub fn aspect_registration_id(&self) -> Option<&BridgeAspectRegistrationId> {
        self.match_detail.aspect_registration_id()
    }

    pub fn subscription_slice_kind(&self) -> Option<&SubscriptionSliceKind> {
        self.match_detail.subscription_slice_kind()
    }

    pub fn slice_fallback_policy(&self) -> Option<SliceFallbackPolicy> {
        self.match_detail.slice_fallback_policy()
    }

    pub fn match_detail(&self) -> &BridgeRouteRecordMatch {
        &self.match_detail
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRouteRecord {
    source: BridgeRouteSourceRecord,
    routing: BridgeRoutingDiagnosticsRecord,
    lowering: BridgeLoweringDiagnosticsRecord,
    contract: BridgeContractDiagnosticsRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeFailureClass {
    Delivery(BridgeDeliveryErrorKind),
    Replay(BridgeReplayErrorKind),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeFailureRecord {
    failure_class: BridgeFailureClass,
    source_commit: TruthCommitIdentity,
    source_patch: TruthPatchIdentity,
    source_snapshot: TruthSnapshotIdentity,
    route_identity: Option<BridgeRouteIdentity>,
    invalidation_identity: Option<BridgeInvalidationIdentity>,
    subscription_slice_identity: Option<BridgeSubscriptionSliceIdentity>,
    contract_proof: Option<BridgeRouteContractProof>,
    counters: BridgeRoutingCounters,
    detail: String,
    context: BridgeErrorContext,
}

impl BridgeFailureRecord {
    pub(crate) fn from_failure(
        source: crate::diagnostics::BridgeFailureSource,
        failure_class: BridgeFailureClass,
        detail: impl Into<String>,
        context: BridgeErrorContext,
    ) -> Self {
        Self {
            failure_class,
            source_commit: source.source_commit,
            source_patch: source.source_patch,
            source_snapshot: source.source_snapshot,
            route_identity: source.route_identity,
            invalidation_identity: source.invalidation_identity,
            subscription_slice_identity: source.subscription_slice_identity,
            contract_proof: source.contract_proof,
            counters: source.counters,
            detail: detail.into(),
            context,
        }
    }

    pub fn failure_class(&self) -> &BridgeFailureClass {
        &self.failure_class
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        &self.source_commit
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        &self.source_patch
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        &self.source_snapshot
    }

    pub fn route_identity(&self) -> Option<&BridgeRouteIdentity> {
        self.route_identity.as_ref()
    }

    pub fn invalidation_identity(&self) -> Option<&BridgeInvalidationIdentity> {
        self.invalidation_identity.as_ref()
    }

    pub fn subscription_slice_identity(&self) -> Option<&BridgeSubscriptionSliceIdentity> {
        self.subscription_slice_identity.as_ref()
    }

    pub fn contract_proof(&self) -> Option<&BridgeRouteContractProof> {
        self.contract_proof.as_ref()
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        &self.counters
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn context(&self) -> &BridgeErrorContext {
        &self.context
    }
}

impl BridgeRouteRecord {
    pub(crate) fn new(
        route_identity: BridgeRouteIdentity,
        invalidation_identity: BridgeInvalidationIdentity,
        source_commit: TruthCommitIdentity,
        source_patch: TruthPatchIdentity,
        source_snapshot: TruthSnapshotIdentity,
        contract_proof: BridgeRouteContractProof,
        subscription_slice_identity: BridgeSubscriptionSliceIdentity,
        entries: Arc<[BridgeRouteRecordEntry]>,
        subscription_slices: Arc<[BridgeSubscriptionSlice]>,
        invalidation_targets: Arc<[BridgeInvalidationTarget]>,
        counters: BridgeRoutingCounters,
    ) -> Self {
        Self {
            source: BridgeRouteSourceRecord::new(source_commit, source_patch, source_snapshot),
            routing: BridgeRoutingDiagnosticsRecord::new(route_identity, entries, counters),
            lowering: BridgeLoweringDiagnosticsRecord::new(
                invalidation_identity,
                subscription_slice_identity,
                subscription_slices,
                invalidation_targets,
            ),
            contract: BridgeContractDiagnosticsRecord::new(contract_proof),
        }
    }

    pub fn route_identity(&self) -> &BridgeRouteIdentity {
        self.routing.route_identity()
    }

    pub fn invalidation_identity(&self) -> &BridgeInvalidationIdentity {
        self.lowering.invalidation_identity()
    }

    pub fn source_commit(&self) -> &TruthCommitIdentity {
        self.source.source_commit()
    }

    pub fn source_patch(&self) -> &TruthPatchIdentity {
        self.source.source_patch()
    }

    pub fn source_snapshot(&self) -> &TruthSnapshotIdentity {
        self.source.source_snapshot()
    }

    pub fn contract_proof(&self) -> &BridgeRouteContractProof {
        self.contract.contract_proof()
    }

    pub fn mapping_context(&self) -> &BridgeMappingContext {
        self.contract.mapping_context()
    }

    pub fn source_digest(&self) -> &crate::input::envelope::BridgeCommittedPatchDigest {
        self.contract_proof().source_digest()
    }

    pub fn planning_provenance_digest(&self) -> &str {
        self.contract_proof().planning_provenance_digest()
    }

    pub fn planning_summary_digest(&self) -> &str {
        self.contract_proof().planning_summary_digest()
    }

    pub fn lowering_provenance_digest(&self) -> &str {
        self.contract_proof().lowering_provenance_digest()
    }

    pub fn lowering_summary_digest(&self) -> &str {
        self.contract_proof().lowering_summary_digest()
    }

    pub fn subscription_slice_identity(&self) -> &BridgeSubscriptionSliceIdentity {
        self.lowering.subscription_slice_identity()
    }

    pub fn entries(&self) -> &[BridgeRouteRecordEntry] {
        self.routing.entries()
    }

    pub fn subscription_slices(&self) -> &[BridgeSubscriptionSlice] {
        self.lowering.subscription_slices()
    }

    pub fn invalidation_targets(&self) -> &[BridgeInvalidationTarget] {
        self.lowering.invalidation_targets()
    }

    pub fn counters(&self) -> &BridgeRoutingCounters {
        self.routing.counters()
    }

    pub fn source_record(&self) -> &BridgeRouteSourceRecord {
        &self.source
    }

    pub fn routing_record(&self) -> &BridgeRoutingDiagnosticsRecord {
        &self.routing
    }

    pub fn lowering_record(&self) -> &BridgeLoweringDiagnosticsRecord {
        &self.lowering
    }

    pub fn contract_record(&self) -> &BridgeContractDiagnosticsRecord {
        &self.contract
    }
}

impl FineGrainedMatchOutcome {
    pub fn truth_surface_kind(&self) -> TruthDeltaSurfaceKind {
        match self {
            Self::Matched {
                truth_surface_kind, ..
            }
            | Self::FallbackAdmitted {
                truth_surface_kind, ..
            }
            | Self::SuppressedByRegistrationPolicy { truth_surface_kind }
            | Self::UnsupportedSurfaceCategory { truth_surface_kind }
            | Self::AmbiguousRegistration { truth_surface_kind } => *truth_surface_kind,
        }
    }

    pub fn aspect_registration_id(&self) -> Option<&BridgeAspectRegistrationId> {
        match self {
            Self::Matched {
                aspect_registration_id,
                ..
            }
            | Self::FallbackAdmitted {
                aspect_registration_id,
                ..
            } => Some(aspect_registration_id),
            Self::SuppressedByRegistrationPolicy { .. }
            | Self::UnsupportedSurfaceCategory { .. }
            | Self::AmbiguousRegistration { .. } => None,
        }
    }

    pub fn slice_fallback_policy(&self) -> Option<SliceFallbackPolicy> {
        match self {
            Self::Matched { .. } => Some(SliceFallbackPolicy::Disallow),
            Self::FallbackAdmitted {
                fallback_policy,
                ..
            } => Some(*fallback_policy),
            Self::SuppressedByRegistrationPolicy { .. }
            | Self::UnsupportedSurfaceCategory { .. }
            | Self::AmbiguousRegistration { .. } => None,
        }
    }
}
