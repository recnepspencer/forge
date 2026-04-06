//! Canonical truth envelope shapes for bridge-owned committed patch input.

use std::sync::Arc;

use crate::clone_budget::CheapClone;
use crate::identity::{
    BridgeIdentity, CommittedPatchDigestTag, TruthBranchTag, TruthCommitTag, TruthPatchTag,
};
use crate::snapshot::TruthSnapshotIdentity;

pub const BRIDGE_PRODUCER_EXPORT_SCHEMA_V1: &str = "forge-runtime-bridge.producer-envelope.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeProducerAuthorityKind {
    RelationalPublication,
    BridgeHarnessFixture,
    Unknown,
}

impl BridgeProducerAuthorityKind {
    pub fn canonical_label(self) -> &'static str {
        match self {
            Self::RelationalPublication => "relational-publication",
            Self::BridgeHarnessFixture => "bridge-harness-fixture",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeProducerMetadata {
    authority_kind: BridgeProducerAuthorityKind,
    export_schema_version: Arc<str>,
    producer_semantics_version: Option<Arc<str>>,
}

impl BridgeProducerMetadata {
    pub fn relational_publication() -> Self {
        Self::new(
            BridgeProducerAuthorityKind::RelationalPublication,
            BRIDGE_PRODUCER_EXPORT_SCHEMA_V1,
        )
    }

    pub fn bridge_harness_fixture() -> Self {
        Self::new(
            BridgeProducerAuthorityKind::BridgeHarnessFixture,
            BRIDGE_PRODUCER_EXPORT_SCHEMA_V1,
        )
    }

    pub fn new(
        authority_kind: BridgeProducerAuthorityKind,
        export_schema_version: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            authority_kind,
            export_schema_version: export_schema_version.into(),
            producer_semantics_version: None,
        }
    }

    pub fn with_producer_semantics_version(
        mut self,
        producer_semantics_version: impl Into<Arc<str>>,
    ) -> Self {
        self.producer_semantics_version = Some(producer_semantics_version.into());
        self
    }

    pub fn authority_kind(&self) -> BridgeProducerAuthorityKind {
        self.authority_kind
    }

    pub fn export_schema_version(&self) -> &str {
        self.export_schema_version.as_ref()
    }

    pub fn producer_semantics_version(&self) -> Option<&str> {
        self.producer_semantics_version.as_deref()
    }
}

impl CheapClone for BridgeProducerMetadata {}

pub type TruthCommitIdentity = BridgeIdentity<TruthCommitTag>;
pub type TruthPatchIdentity = BridgeIdentity<TruthPatchTag>;
pub type TruthBranchIdentity = BridgeIdentity<TruthBranchTag>;
pub type BridgeCommittedPatchDigest = BridgeIdentity<CommittedPatchDigestTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgePatchEnvelopeHeader {
    producer_metadata: BridgeProducerMetadata,
    commit_identity: TruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    branch_identity: TruthBranchIdentity,
}

impl BridgePatchEnvelopeHeader {
    fn new(
        producer_metadata: BridgeProducerMetadata,
        commit_identity: TruthCommitIdentity,
        patch_identity: TruthPatchIdentity,
        snapshot_identity: TruthSnapshotIdentity,
        branch_identity: TruthBranchIdentity,
    ) -> Self {
        Self {
            producer_metadata,
            commit_identity,
            patch_identity,
            snapshot_identity,
            branch_identity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BridgeEnvelopeCore<Payload> {
    header: BridgePatchEnvelopeHeader,
    payload: Payload,
}

impl<Payload> BridgeEnvelopeCore<Payload> {
    fn new(header: BridgePatchEnvelopeHeader, payload: Payload) -> Self {
        Self { header, payload }
    }

    fn producer_metadata(&self) -> &BridgeProducerMetadata {
        &self.header.producer_metadata
    }

    fn commit_identity(&self) -> &TruthCommitIdentity {
        &self.header.commit_identity
    }

    fn patch_identity(&self) -> &TruthPatchIdentity {
        &self.header.patch_identity
    }

    fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.header.snapshot_identity
    }

    fn branch_identity(&self) -> &TruthBranchIdentity {
        &self.header.branch_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RawPatchPayload {
    patch_items: Vec<BridgeCommittedPatchItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CanonicalPatchPayload {
    patch_summary: BridgeCommittedPatchSummary,
    patch_body: BridgeCommittedPatchBody,
    digest: BridgeCommittedPatchDigest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawCommittedPatchEnvelope(BridgeEnvelopeCore<RawPatchPayload>);

impl RawCommittedPatchEnvelope {
    pub fn new(
        commit_identity: TruthCommitIdentity,
        patch_identity: TruthPatchIdentity,
        snapshot_identity: TruthSnapshotIdentity,
        branch_identity: TruthBranchIdentity,
        patch_items: Vec<BridgeCommittedPatchItem>,
    ) -> Self {
        Self::new_with_metadata(
            BridgeProducerMetadata::relational_publication(),
            commit_identity,
            patch_identity,
            snapshot_identity,
            branch_identity,
            patch_items,
        )
    }

    pub fn new_with_metadata(
        producer_metadata: BridgeProducerMetadata,
        commit_identity: TruthCommitIdentity,
        patch_identity: TruthPatchIdentity,
        snapshot_identity: TruthSnapshotIdentity,
        branch_identity: TruthBranchIdentity,
        patch_items: Vec<BridgeCommittedPatchItem>,
    ) -> Self {
        Self(BridgeEnvelopeCore::new(
            BridgePatchEnvelopeHeader::new(
                producer_metadata,
                commit_identity,
                patch_identity,
                snapshot_identity,
                branch_identity,
            ),
            RawPatchPayload { patch_items },
        ))
    }

    pub fn producer_metadata(&self) -> &BridgeProducerMetadata {
        self.0.producer_metadata()
    }

    pub fn commit_identity(&self) -> &TruthCommitIdentity {
        self.0.commit_identity()
    }

    pub fn patch_identity(&self) -> &TruthPatchIdentity {
        self.0.patch_identity()
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.0.snapshot_identity()
    }

    pub fn branch_identity(&self) -> &TruthBranchIdentity {
        self.0.branch_identity()
    }

    pub fn patch_items(&self) -> &[BridgeCommittedPatchItem] {
        &self.0.payload.patch_items
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCommittedPatchItem {
    entity_identity: Arc<str>,
    aspect_label: Arc<str>,
    surface_label: Arc<str>,
}

impl BridgeCommittedPatchItem {
    pub fn new(
        entity_identity: impl Into<Arc<str>>,
        aspect_label: impl Into<Arc<str>>,
        surface_label: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            aspect_label: aspect_label.into(),
            surface_label: surface_label.into(),
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn aspect_label(&self) -> &str {
        self.aspect_label.as_ref()
    }

    pub fn surface_label(&self) -> &str {
        self.surface_label.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCommittedPatchBody {
    canonical_items: Vec<BridgeCommittedPatchItem>,
}

impl BridgeCommittedPatchBody {
    pub fn new(canonical_items: Vec<BridgeCommittedPatchItem>) -> Self {
        Self { canonical_items }
    }

    pub fn canonical_items(&self) -> &[BridgeCommittedPatchItem] {
        &self.canonical_items
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCommittedPatchSummary {
    patch_item_count: usize,
    normalized_patch_item_count: usize,
}

impl BridgeCommittedPatchSummary {
    pub fn new(patch_item_count: usize, normalized_patch_item_count: usize) -> Self {
        Self {
            patch_item_count,
            normalized_patch_item_count,
        }
    }

    pub fn patch_item_count(&self) -> usize {
        self.patch_item_count
    }

    pub fn normalized_patch_item_count(&self) -> usize {
        self.normalized_patch_item_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCommittedPatchEnvelope(BridgeEnvelopeCore<CanonicalPatchPayload>);

impl BridgeCommittedPatchEnvelope {
    pub(crate) fn from_normalized(normalized: NormalizedBridgePatchEnvelope) -> Self {
        Self(normalized.0)
    }

    pub fn producer_metadata(&self) -> &BridgeProducerMetadata {
        self.0.producer_metadata()
    }

    pub fn commit_identity(&self) -> &TruthCommitIdentity {
        self.0.commit_identity()
    }

    pub fn patch_identity(&self) -> &TruthPatchIdentity {
        self.0.patch_identity()
    }

    pub fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.0.snapshot_identity()
    }

    pub fn branch_identity(&self) -> &TruthBranchIdentity {
        self.0.branch_identity()
    }

    pub fn patch_summary(&self) -> &BridgeCommittedPatchSummary {
        &self.0.payload.patch_summary
    }

    pub fn patch_body(&self) -> &BridgeCommittedPatchBody {
        &self.0.payload.patch_body
    }

    pub fn digest(&self) -> &BridgeCommittedPatchDigest {
        &self.0.payload.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NormalizedBridgePatchEnvelope(BridgeEnvelopeCore<CanonicalPatchPayload>);

impl NormalizedBridgePatchEnvelope {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        producer_metadata: BridgeProducerMetadata,
        commit_identity: TruthCommitIdentity,
        patch_identity: TruthPatchIdentity,
        snapshot_identity: TruthSnapshotIdentity,
        branch_identity: TruthBranchIdentity,
        patch_summary: BridgeCommittedPatchSummary,
        patch_body: BridgeCommittedPatchBody,
        digest: BridgeCommittedPatchDigest,
    ) -> Self {
        Self(BridgeEnvelopeCore::new(
            BridgePatchEnvelopeHeader::new(
                producer_metadata,
                commit_identity,
                patch_identity,
                snapshot_identity,
                branch_identity,
            ),
            CanonicalPatchPayload {
                patch_summary,
                patch_body,
                digest,
            },
        ))
    }

    pub(crate) fn producer_metadata(&self) -> &BridgeProducerMetadata {
        self.0.producer_metadata()
    }

    pub(crate) fn commit_identity(&self) -> &TruthCommitIdentity {
        self.0.commit_identity()
    }

    pub(crate) fn patch_identity(&self) -> &TruthPatchIdentity {
        self.0.patch_identity()
    }

    pub(crate) fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        self.0.snapshot_identity()
    }

    pub(crate) fn branch_identity(&self) -> &TruthBranchIdentity {
        self.0.branch_identity()
    }

    pub(crate) fn patch_summary(&self) -> &BridgeCommittedPatchSummary {
        &self.0.payload.patch_summary
    }

    pub(crate) fn patch_body(&self) -> &BridgeCommittedPatchBody {
        &self.0.payload.patch_body
    }

    pub(crate) fn digest(&self) -> &BridgeCommittedPatchDigest {
        &self.0.payload.digest
    }
}
