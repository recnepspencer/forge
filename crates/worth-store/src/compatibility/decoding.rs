use super::admission::{
    CompatibilityAdmissionBatch, CompatibilityDecision, CompatibilityRejection,
    CompatibilityRejectionKind,
};
use super::manifests::{
    ArtifactFamilyId, ArtifactFormatVersion, ArtifactSemanticVersion, CompatibilityManifestDigest,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RawArtifactBytes {
    family_id: ArtifactFamilyId,
    bytes: Vec<u8>,
}

impl RawArtifactBytes {
    pub fn new(family_id: ArtifactFamilyId, bytes: Vec<u8>) -> Self {
        Self { family_id, bytes }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityArtifactFrameHeader {
    family_id: ArtifactFamilyId,
    format_version: ArtifactFormatVersion,
    semantic_version: ArtifactSemanticVersion,
    manifest_digest: CompatibilityManifestDigest,
    declared_payload_len: usize,
}

impl CompatibilityArtifactFrameHeader {
    pub fn new(
        family_id: ArtifactFamilyId,
        format_version: ArtifactFormatVersion,
        semantic_version: ArtifactSemanticVersion,
        manifest_digest: CompatibilityManifestDigest,
        declared_payload_len: usize,
    ) -> Self {
        Self {
            family_id,
            format_version,
            semantic_version,
            manifest_digest,
            declared_payload_len,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FramedArtifactRecord {
    family_id: ArtifactFamilyId,
    format_version: ArtifactFormatVersion,
    structural_digest: String,
}

pub(crate) fn decode_artifact_to_quarantine(
    batch: &mut CompatibilityAdmissionBatch,
    raw: RawArtifactBytes,
    header: CompatibilityArtifactFrameHeader,
) -> Result<QuarantinedDecodedArtifact, CompatibilityRejection> {
    if raw.bytes().is_empty() {
        batch.counters_mut().record_malformed_frame();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MalformedFrame,
            raw.family_id().clone(),
            "artifact frame is empty",
        ));
    }
    if raw.family_id() != &header.family_id {
        batch.counters_mut().record_malformed_frame();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MalformedFrame,
            raw.family_id().clone(),
            "artifact frame family does not match raw artifact family",
        ));
    }
    if raw.bytes().len() < header.declared_payload_len {
        batch.counters_mut().record_malformed_frame();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::TruncatedFrame,
            raw.family_id().clone(),
            "artifact frame is shorter than its declared payload length",
        ));
    }
    if raw.bytes().len() > header.declared_payload_len {
        batch.counters_mut().record_malformed_frame();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::MalformedFrame,
            raw.family_id().clone(),
            "artifact frame is longer than its declared payload length",
        ));
    }
    let structural_digest = format!("{:x}", Sha256::digest(raw.bytes()));
    Ok(QuarantinedDecodedArtifact::new(
        header.family_id,
        header.format_version,
        header.semantic_version,
        header.manifest_digest,
        structural_digest,
        "decoded artifact remains quarantined until compatibility admission",
    ))
}

impl FramedArtifactRecord {
    pub fn new(
        family_id: ArtifactFamilyId,
        format_version: ArtifactFormatVersion,
        structural_digest: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            format_version,
            structural_digest: structural_digest.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QuarantinedDecodedArtifact {
    family_id: ArtifactFamilyId,
    format_version: ArtifactFormatVersion,
    semantic_version: ArtifactSemanticVersion,
    manifest_digest: CompatibilityManifestDigest,
    structural_digest: String,
    diagnostic_context: String,
}

impl QuarantinedDecodedArtifact {
    pub(crate) fn new(
        family_id: ArtifactFamilyId,
        format_version: ArtifactFormatVersion,
        semantic_version: ArtifactSemanticVersion,
        manifest_digest: CompatibilityManifestDigest,
        structural_digest: impl Into<String>,
        diagnostic_context: impl Into<String>,
    ) -> Self {
        Self {
            family_id,
            format_version,
            semantic_version,
            manifest_digest,
            structural_digest: structural_digest.into(),
            diagnostic_context: diagnostic_context.into(),
        }
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        &self.family_id
    }

    pub fn format_version(&self) -> ArtifactFormatVersion {
        self.format_version
    }

    pub fn semantic_version(&self) -> ArtifactSemanticVersion {
        self.semantic_version
    }

    pub fn manifest_digest(&self) -> &CompatibilityManifestDigest {
        &self.manifest_digest
    }

    pub fn structural_digest(&self) -> &str {
        &self.structural_digest
    }

    pub fn diagnostic_context(&self) -> &str {
        &self.diagnostic_context
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityCheckedArtifact {
    quarantined: QuarantinedDecodedArtifact,
    decision: CompatibilityDecision,
}

impl CompatibilityCheckedArtifact {
    pub(crate) fn new(
        quarantined: QuarantinedDecodedArtifact,
        decision: CompatibilityDecision,
    ) -> Self {
        Self {
            quarantined,
            decision,
        }
    }

    pub fn decision(&self) -> &CompatibilityDecision {
        &self.decision
    }

    pub fn family_id(&self) -> &ArtifactFamilyId {
        self.quarantined.family_id()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SemanticArtifactView {
    family_id: ArtifactFamilyId,
    semantic_label: String,
}

impl SemanticArtifactView {
    pub(crate) fn new(family_id: ArtifactFamilyId, semantic_label: impl Into<String>) -> Self {
        Self {
            family_id,
            semantic_label: semantic_label.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct CompatibilityAdmittedArtifact {
    family_id: ArtifactFamilyId,
    view: SemanticArtifactView,
}

impl CompatibilityAdmittedArtifact {
    pub(crate) fn new(family_id: ArtifactFamilyId, view: SemanticArtifactView) -> Self {
        Self { family_id, view }
    }

    pub fn view(&self) -> &SemanticArtifactView {
        &self.view
    }
}
