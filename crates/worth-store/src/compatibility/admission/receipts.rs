use super::*;

macro_rules! proof_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
        pub struct $name {
            family_id: ArtifactFamilyId,
        }

        impl $name {
            pub(crate) fn new(family_id: ArtifactFamilyId) -> Self {
                Self { family_id }
            }

            pub fn family_id(&self) -> &ArtifactFamilyId {
                &self.family_id
            }
        }
    };
}
proof_wrapper!(SemanticMeaningPreservationWitness);
proof_wrapper!(ForwardReadCompatibilityWitness);
proof_wrapper!(BackwardReadCompatibilityWitness);
proof_wrapper!(UpgradeAdmissionWitness);

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdapterParityWitness {
    adapter_id: CompatibilityAdapterId,
    adapter_digest: CompatibilityAdapterDigest,
    cost_class: CompatibilityAdapterCostClass,
}

impl CompatibilityAdapterParityWitness {
    pub(crate) fn new(
        adapter_id: CompatibilityAdapterId,
        adapter_digest: CompatibilityAdapterDigest,
        cost_class: CompatibilityAdapterCostClass,
    ) -> Self {
        Self {
            adapter_id,
            adapter_digest,
            cost_class,
        }
    }

    pub fn adapter_id(&self) -> &CompatibilityAdapterId {
        &self.adapter_id
    }

    pub fn adapter_digest(&self) -> &CompatibilityAdapterDigest {
        &self.adapter_digest
    }

    pub fn cost_class(&self) -> CompatibilityAdapterCostClass {
        self.cost_class
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmissionReceipt {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    registry_snapshot_identity: String,
    manifest_frontier_identity: String,
    observed_semantic_version: ArtifactSemanticVersion,
    target_semantic_version: ArtifactSemanticVersion,
    admission_path: CompatibilityAdmissionPath,
    relation: CompatibilityRelation,
}

impl CompatibilityAdmissionReceipt {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        manifest_digest: CompatibilityManifestDigest,
        registry_snapshot_identity: impl Into<String>,
        manifest_frontier_identity: impl Into<String>,
        observed_semantic_version: ArtifactSemanticVersion,
        target_semantic_version: ArtifactSemanticVersion,
        admission_path: CompatibilityAdmissionPath,
        relation: CompatibilityRelation,
    ) -> Self {
        Self {
            family_id,
            manifest_digest,
            registry_snapshot_identity: registry_snapshot_identity.into(),
            manifest_frontier_identity: manifest_frontier_identity.into(),
            observed_semantic_version,
            target_semantic_version,
            admission_path,
            relation,
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn registry_snapshot_identity(&self) -> &str {
        &self.registry_snapshot_identity
    }

    pub fn manifest_frontier_identity(&self) -> &str {
        &self.manifest_frontier_identity
    }

    pub fn observed_semantic_version(&self) -> ArtifactSemanticVersion {
        self.observed_semantic_version
    }

    pub fn target_semantic_version(&self) -> ArtifactSemanticVersion {
        self.target_semantic_version
    }

    pub fn admission_path(&self) -> CompatibilityAdmissionPath {
        self.admission_path
    }

    pub fn relation(&self) -> CompatibilityRelation {
        self.relation
    }
}

macro_rules! receipt_wrapper {
    ($name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
        pub struct $name {
            receipt: CompatibilityAdmissionReceipt,
        }

        impl $name {
            pub(crate) fn new(receipt: CompatibilityAdmissionReceipt) -> Self {
                Self { receipt }
            }

            pub fn receipt(&self) -> &CompatibilityAdmissionReceipt {
                &self.receipt
            }
        }
    };
}

receipt_wrapper!(ReadCompatibilityReceipt);
receipt_wrapper!(WriteCompatibilityReceipt);
receipt_wrapper!(DerivedReuseCompatibilityReceipt);
receipt_wrapper!(RestoreCompatibilityReceipt);
receipt_wrapper!(RollingWindowCompatibilityReceipt);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub(super) struct ReceiptKey {
    family_id: ArtifactFamilyId,
    manifest_digest: CompatibilityManifestDigest,
    registry_snapshot_identity: String,
    manifest_frontier_identity: String,
    observed_semantic_version: ArtifactSemanticVersion,
    capability_family_id: ArtifactFamilyId,
    target_semantic_version: ArtifactSemanticVersion,
    admission_path: CompatibilityAdmissionPath,
    direction: ReceiptDirection,
}

impl ReceiptKey {
    pub(super) fn read(
        manifest_index: &CompatibilityManifestIndex,
        artifact: &QuarantinedDecodedArtifact,
        reader_capabilities: &ReaderCapabilitySet,
        intent: &CompatibilityReadIntent,
        path: CompatibilityAdmissionPath,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            registry_snapshot_identity: manifest_index.registry_snapshot_identity().to_string(),
            manifest_frontier_identity: manifest_index.manifest_frontier_identity().to_string(),
            observed_semantic_version: artifact.semantic_version(),
            capability_family_id: reader_capabilities.family_id().clone(),
            target_semantic_version: intent.target_semantic_version(),
            admission_path: path,
            direction: ReceiptDirection::Read,
        }
    }

    pub(super) fn write(
        manifest_index: &CompatibilityManifestIndex,
        artifact: &QuarantinedDecodedArtifact,
        writer_capabilities: &WriterCapabilitySet,
        intent: &CompatibilityWriteIntent,
        path: CompatibilityAdmissionPath,
    ) -> Self {
        Self {
            family_id: artifact.family_id().clone(),
            manifest_digest: artifact.manifest_digest().clone(),
            registry_snapshot_identity: manifest_index.registry_snapshot_identity().to_string(),
            manifest_frontier_identity: manifest_index.manifest_frontier_identity().to_string(),
            observed_semantic_version: artifact.semantic_version(),
            capability_family_id: writer_capabilities.family_id().clone(),
            target_semantic_version: intent.target_semantic_version(),
            admission_path: path,
            direction: ReceiptDirection::Write,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
enum ReceiptDirection {
    Read,
    Write,
}

pub(super) fn has_stale_receipt_basis<'a>(
    mut existing: impl Iterator<Item = &'a ReceiptKey>,
    candidate: &ReceiptKey,
) -> bool {
    existing.any(|key| {
        key.family_id == candidate.family_id
            && key.manifest_digest == candidate.manifest_digest
            && key.observed_semantic_version == candidate.observed_semantic_version
            && key.capability_family_id == candidate.capability_family_id
            && key.target_semantic_version == candidate.target_semantic_version
            && key.admission_path == candidate.admission_path
            && key.direction == candidate.direction
            && (key.registry_snapshot_identity != candidate.registry_snapshot_identity
                || key.manifest_frontier_identity != candidate.manifest_frontier_identity)
    })
}
