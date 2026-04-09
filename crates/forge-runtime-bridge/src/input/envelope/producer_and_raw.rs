pub const BRIDGE_PRODUCER_EXPORT_SCHEMA_V1: &str = "forge-runtime-bridge.producer-envelope.v1";

use super::*;

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
pub(super) struct BridgePatchEnvelopeHeader {
    producer_metadata: BridgeProducerMetadata,
    commit_identity: TruthCommitIdentity,
    patch_identity: TruthPatchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
    branch_identity: TruthBranchIdentity,
}

impl BridgePatchEnvelopeHeader {
    pub(super) fn new(
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
pub(super) struct BridgeEnvelopeCore<Payload> {
    pub(super) header: BridgePatchEnvelopeHeader,
    pub(super) payload: Payload,
}

impl<Payload> BridgeEnvelopeCore<Payload> {
    pub(super) fn new(header: BridgePatchEnvelopeHeader, payload: Payload) -> Self {
        Self { header, payload }
    }

    pub(super) fn producer_metadata(&self) -> &BridgeProducerMetadata {
        &self.header.producer_metadata
    }

    pub(super) fn commit_identity(&self) -> &TruthCommitIdentity {
        &self.header.commit_identity
    }

    pub(super) fn patch_identity(&self) -> &TruthPatchIdentity {
        &self.header.patch_identity
    }

    pub(super) fn snapshot_identity(&self) -> &TruthSnapshotIdentity {
        &self.header.snapshot_identity
    }

    pub(super) fn branch_identity(&self) -> &TruthBranchIdentity {
        &self.header.branch_identity
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RawPatchPayload {
    patch_items: Vec<BridgeCommittedPatchItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CanonicalPatchPayload {
    pub(super) patch_summary: BridgeCommittedPatchSummary,
    pub(super) patch_body: BridgeCommittedPatchBody,
    pub(super) digest: BridgeCommittedPatchDigest,
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
