use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::facade::{
    BridgeAspectRegistration, BridgeAspectRegistrationId, BridgeMappingId,
    BridgeMappingRegistration, BridgeProducerMetadata, CoarseRoutingMode, InvalidationSink,
    MappingSelector, RawCommittedPatchEnvelope, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SliceFallbackPolicy, SnapshotReadRecord, SnapshotReaderPool,
    SubscriptionSliceKind, TruthBranchIdentity, TruthCommitIdentity, TruthDeltaSurfaceKind,
    TruthPatchIdentity, TruthPatchScope, TruthSnapshotIdentity,
};

use crate::harness::fixtures::{InMemoryRelationalBridgeSource, SnapshotFixture};

pub(super) fn registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-name"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        SignalInvalidationScope::new("signal.profile"),
        CoarseRoutingMode::Direct,
    )
}

pub(super) fn surface_fallback_registration() -> BridgeMappingRegistration {
    BridgeMappingRegistration::new(
        BridgeMappingId::new("profile-surface-fallback"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::any(),
        ),
        SignalInvalidationScope::new("signal.profile.fallback"),
        CoarseRoutingMode::Direct,
    )
}

pub(super) fn committed_patch(
    commit: &str,
    patch: &str,
    snapshot: &str,
    surface: &str,
) -> RawCommittedPatchEnvelope {
    committed_patch_on_branch("main", commit, patch, snapshot, surface)
}

pub(super) fn committed_patch_on_branch(
    branch: &str,
    commit: &str,
    patch: &str,
    snapshot: &str,
    surface: &str,
) -> RawCommittedPatchEnvelope {
    RawCommittedPatchEnvelope::new_with_metadata(
        BridgeProducerMetadata::bridge_harness_fixture(),
        TruthCommitIdentity::new(commit),
        TruthPatchIdentity::new(patch),
        TruthSnapshotIdentity::new(snapshot),
        TruthBranchIdentity::new(branch),
        vec![crate::facade::BridgeCommittedPatchItem::new(
            "user", "profile", surface,
        )],
    )
}

pub(super) fn committed_patch_items(
    commit: &str,
    patch: &str,
    snapshot: &str,
    items: Vec<crate::facade::BridgeCommittedPatchItem>,
) -> RawCommittedPatchEnvelope {
    RawCommittedPatchEnvelope::new_with_metadata(
        BridgeProducerMetadata::bridge_harness_fixture(),
        TruthCommitIdentity::new(commit),
        TruthPatchIdentity::new(patch),
        TruthSnapshotIdentity::new(snapshot),
        TruthBranchIdentity::new("main"),
        items,
    )
}

pub(super) fn snapshot(snapshot: &str, value: &str) -> SnapshotFixture {
    SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot),
        vec![SnapshotReadRecord::new(
            "user:profile",
            value.as_bytes().to_vec(),
        )],
    )
}

pub(super) fn field_slice_snapshot(snapshot: &str, value: &str) -> SnapshotFixture {
    SnapshotFixture::new(
        TruthSnapshotIdentity::new(snapshot),
        vec![SnapshotReadRecord::new(
            "user:profile:signal-field:name",
            value.as_bytes().to_vec(),
        )],
    )
}

pub(super) fn build_runtime<S>(
    source: InMemoryRelationalBridgeSource,
    sink: S,
    mappings: Vec<BridgeMappingRegistration>,
) -> crate::facade::RuntimeBridge
where
    S: InvalidationSink,
{
    build_runtime_with_aspects(source, sink, mappings, vec![])
}

pub(super) fn build_runtime_with_aspects<S>(
    source: InMemoryRelationalBridgeSource,
    sink: S,
    mappings: Vec<BridgeMappingRegistration>,
    aspect_mappings: Vec<BridgeAspectRegistration>,
) -> crate::facade::RuntimeBridge
where
    S: InvalidationSink,
{
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(sink);
    let mut mappings = mappings.into_iter();
    let first_mapping = mappings
        .next()
        .expect("bridge harness tests require at least one mapping");
    let mut builder = builder.register_mapping(first_mapping);
    for mapping in mappings {
        builder = builder.register_mapping(mapping);
    }
    for aspect_mapping in aspect_mappings {
        builder = builder.register_aspect_mapping(aspect_mapping);
    }
    builder
        .build()
        .expect("bridge runtime should build for harness tests")
}

#[derive(Debug, Clone, Default)]
pub(super) struct RejectingSignalSink;

impl InvalidationSink for RejectingSignalSink {
    fn deliver_invalidation(
        &self,
        _delivery: crate::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<crate::facade::BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Err(SignalBridgeSinkError::new("forced sink rejection"))
    }
}

#[derive(Clone)]
pub(super) struct CountingSnapshotReaderPool {
    source: InMemoryRelationalBridgeSource,
    acquire_count: Arc<AtomicUsize>,
    release_count: Arc<AtomicUsize>,
}

impl CountingSnapshotReaderPool {
    pub(super) fn new(source: InMemoryRelationalBridgeSource) -> Self {
        Self {
            source,
            acquire_count: Arc::new(AtomicUsize::new(0)),
            release_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub(super) fn acquire_count(&self) -> usize {
        self.acquire_count.load(Ordering::SeqCst)
    }

    pub(super) fn release_count(&self) -> usize {
        self.release_count.load(Ordering::SeqCst)
    }
}

impl SnapshotReaderPool for CountingSnapshotReaderPool {
    fn acquire(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<
        Box<dyn crate::facade::TruthSnapshotReader>,
        crate::facade::RelationalBridgeSourceError,
    > {
        self.acquire_count.fetch_add(1, Ordering::SeqCst);
        crate::facade::SnapshotReadSource::open_snapshot(&self.source, identity)
    }

    fn release(&self, _reader: Box<dyn crate::facade::TruthSnapshotReader>) {
        self.release_count.fetch_add(1, Ordering::SeqCst);
    }
}

pub(super) fn field_aspect_registration() -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new("profile-name-field"),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        TruthDeltaSurfaceKind::EntityField,
        SubscriptionSliceKind::SignalField,
        SliceFallbackPolicy::Disallow,
    )
}

pub(super) fn field_aspect_registration_with_kind(
    registration_id: &str,
    surface_kind: TruthDeltaSurfaceKind,
    slice_kind: SubscriptionSliceKind,
) -> BridgeAspectRegistration {
    BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::new(registration_id),
        TruthPatchScope::new(
            MappingSelector::exact("user"),
            MappingSelector::exact("profile"),
            MappingSelector::exact("name"),
        ),
        surface_kind,
        slice_kind,
        SliceFallbackPolicy::Disallow,
    )
}

pub(super) fn merge_declaration(
    id: &str,
    class: crate::facade::BridgeMergeConsumptionClass,
    parents: Vec<&str>,
) -> crate::facade::MergeHistoryDeclaration {
    crate::facade::MergeHistoryDeclaration::new(
        crate::facade::MergeHistoryDeclarationIdentity::new(id),
        class,
        crate::facade::BridgeMergeOntologyMappingSurface::direct_phase_m9_0("rel-merge-v1"),
        crate::facade::BridgeMergeAuthorityBasis::new(
            crate::facade::BridgeMergeAuthorityBasisKind::OrderedMergeCommit,
            format!("merge-artifact:{id}"),
            "rel-merge-v1",
            "schema-policy-v1",
            crate::facade::BridgeMergeParentOrderProof::new(
                parents
                    .into_iter()
                    .map(crate::facade::TruthCommitIdentity::new)
                    .collect(),
            ),
        ),
    )
}
