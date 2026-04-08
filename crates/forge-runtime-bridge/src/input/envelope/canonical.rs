use super::*;

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
