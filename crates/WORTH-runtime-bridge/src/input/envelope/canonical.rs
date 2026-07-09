use super::*;
use crate::error::BridgeRouteError;
use crate::mapping::TruthDeltaSurfaceKind;
use crate::relational_identity::RelationalBridgeRecordIdentityParts;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCommittedPatchItem {
    entity_identity: Arc<str>,
    relational_record_identity: Option<RelationalBridgeRecordIdentityParts>,
    target: BridgeCommittedPatchTarget,
}

impl BridgeCommittedPatchItem {
    pub fn with_target(
        entity_identity: impl Into<Arc<str>>,
        target: BridgeCommittedPatchTarget,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            relational_record_identity: None,
            target,
        }
    }

    pub fn entity_identity(&self) -> &str {
        self.entity_identity.as_ref()
    }

    pub fn relational_record_identity_parts(&self) -> Option<RelationalBridgeRecordIdentityParts> {
        self.relational_record_identity
    }

    pub(crate) fn from_relational_record_parts(
        entity_identity: impl Into<Arc<str>>,
        relational_record_identity: RelationalBridgeRecordIdentityParts,
        target: BridgeCommittedPatchTarget,
    ) -> Self {
        Self {
            entity_identity: entity_identity.into(),
            relational_record_identity: Some(relational_record_identity),
            target,
        }
    }

    pub fn target(&self) -> &BridgeCommittedPatchTarget {
        &self.target
    }

    pub fn aspect_key(&self) -> &AspectKey {
        self.target.aspect_key()
    }

    pub fn aspect_locator(&self) -> &AspectLocator {
        self.target.aspect_locator()
    }

    pub fn field_locator(&self) -> Option<&AspectFieldLocator> {
        self.target.field_locator()
    }

    pub fn mutation_mask(&self) -> &AspectMask<MutationMask> {
        self.target.mutation_mask()
    }

    pub fn projection_mask(&self) -> &AspectMask<ProjectionMask> {
        self.target.projection_mask()
    }

    pub fn surface_kind(&self) -> TruthDeltaSurfaceKind {
        self.target.surface_kind()
    }

    pub(crate) fn target_canonical_basis(&self) -> String {
        self.target.canonical_basis()
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
pub struct BridgeCommittedPatchEnvelope(BridgeEnvelopeCore<CanonicalPatchEnvelopeBody>);

impl BridgeCommittedPatchEnvelope {
    pub(super) fn from_core(core: BridgeEnvelopeCore<CanonicalPatchEnvelopeBody>) -> Self {
        Self(core)
    }

    pub fn new(
        envelope_identity: BridgeCommittedPatchEnvelopeIdentity,
        patch_items: Vec<BridgeCommittedPatchItem>,
    ) -> Result<Self, BridgeRouteError> {
        construction::construct_committed_patch_envelope(envelope_identity, patch_items)
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
        &self.0.body.patch_summary
    }

    pub fn patch_body(&self) -> &BridgeCommittedPatchBody {
        &self.0.body.patch_body
    }

    pub fn digest(&self) -> &BridgeCommittedPatchDigest {
        &self.0.body.digest
    }
}
