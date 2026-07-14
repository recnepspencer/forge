use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const S0_CANONICAL_ARTIFACT_DIR: &str = "_docs/worth-store/artifacts/storage-foundation-s0";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct S0StableDigest(String);

impl S0StableDigest {
    pub fn new(value: impl Into<String>) -> Result<Self, S0ArtifactSchemaCompatibility> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(S0ArtifactSchemaCompatibility::MissingDigest);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct S0EvidenceRef {
    artifact_kind: S0ArtifactKind,
    digest: S0StableDigest,
}

impl S0EvidenceRef {
    pub fn new(artifact_kind: S0ArtifactKind, digest: S0StableDigest) -> Self {
        Self {
            artifact_kind,
            digest,
        }
    }

    pub fn artifact_kind(&self) -> S0ArtifactKind {
        self.artifact_kind
    }

    pub fn digest(&self) -> &S0StableDigest {
        &self.digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum S0ArtifactKind {
    BackendCapabilityMatrix,
    MilestonePhysicalStatusMatrix,
    SemanticPhysicalClaimReport,
    DeferredPhysicalGuaranteeMap,
    TerminologyRiskReport,
    TestMigrationNotes,
    HarnessMaturityReport,
    S1HandoffReadiness,
    S0EvidenceBundle,
}

impl S0ArtifactKind {
    pub const fn file_name(self) -> &'static str {
        match self {
            Self::BackendCapabilityMatrix => "backend-capability-matrix.json",
            Self::MilestonePhysicalStatusMatrix => "milestone-physical-status-matrix.json",
            Self::SemanticPhysicalClaimReport => "semantic-physical-claim-report.json",
            Self::DeferredPhysicalGuaranteeMap => "deferred-physical-guarantee-map.json",
            Self::TerminologyRiskReport => "terminology-risk-report.json",
            Self::TestMigrationNotes => "test-migration-notes.json",
            Self::HarnessMaturityReport => "harness-maturity-report.json",
            Self::S1HandoffReadiness => "s1-handoff-readiness.json",
            Self::S0EvidenceBundle => "s0-evidence-bundle.json",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S0ArtifactSchemaCompatibility {
    Compatible,
    MissingDigest,
    SchemaVersionMismatch,
    DeterministicDigestMismatch,
    RequiredRowsMissing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0CanonicalArtifactSpec {
    artifact_kind: S0ArtifactKind,
    schema_digest: S0StableDigest,
    schema_compatibility: S0ArtifactSchemaCompatibility,
}

impl S0CanonicalArtifactSpec {
    pub fn new(
        artifact_kind: S0ArtifactKind,
        schema_digest: S0StableDigest,
        schema_compatibility: S0ArtifactSchemaCompatibility,
    ) -> Self {
        Self {
            artifact_kind,
            schema_digest,
            schema_compatibility,
        }
    }

    pub fn artifact_kind(&self) -> S0ArtifactKind {
        self.artifact_kind
    }

    pub fn schema_compatibility(&self) -> S0ArtifactSchemaCompatibility {
        self.schema_compatibility
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct S0RequiredArtifactSet {
    required: Vec<S0ArtifactKind>,
}

impl S0RequiredArtifactSet {
    pub fn canonical_artifact_dir(&self) -> &'static str {
        S0_CANONICAL_ARTIFACT_DIR
    }

    pub fn canonical() -> Self {
        Self {
            required: vec![
                S0ArtifactKind::BackendCapabilityMatrix,
                S0ArtifactKind::MilestonePhysicalStatusMatrix,
                S0ArtifactKind::SemanticPhysicalClaimReport,
                S0ArtifactKind::DeferredPhysicalGuaranteeMap,
                S0ArtifactKind::TerminologyRiskReport,
                S0ArtifactKind::TestMigrationNotes,
                S0ArtifactKind::HarnessMaturityReport,
                S0ArtifactKind::S1HandoffReadiness,
                S0ArtifactKind::S0EvidenceBundle,
            ],
        }
    }

    pub fn canonical_complexity_contracts() -> Vec<S0ComplexityContractName> {
        vec![
            S0ComplexityContractName("s0_input_manifest_construction"),
            S0ComplexityContractName("s0_terminology_scan"),
            S0ComplexityContractName("s0_backend_inventory"),
            S0ComplexityContractName("s0_milestone_status_matrix_build"),
            S0ComplexityContractName("s0_evidence_reference_resolution"),
            S0ComplexityContractName("s0_deferred_guarantee_validation"),
            S0ComplexityContractName("s0_artifact_schema_validation"),
            S0ComplexityContractName("s0_digest_construction"),
            S0ComplexityContractName("s0_s1_handoff_validation"),
        ]
    }

    pub fn required(&self) -> &[S0ArtifactKind] {
        &self.required
    }

    pub fn validate_present_artifacts(
        &self,
        present: impl IntoIterator<Item = S0CanonicalArtifactSpec>,
    ) -> S0ArtifactValidationReport {
        let present = present.into_iter().collect::<Vec<_>>();
        let present_kinds = present
            .iter()
            .map(S0CanonicalArtifactSpec::artifact_kind)
            .collect::<BTreeSet<_>>();
        let missing_required = self
            .required
            .iter()
            .copied()
            .filter(|kind| !present_kinds.contains(kind))
            .collect::<Vec<_>>();
        let schema_incompatible = present
            .iter()
            .filter(|spec| spec.schema_compatibility() != S0ArtifactSchemaCompatibility::Compatible)
            .map(S0CanonicalArtifactSpec::artifact_kind)
            .collect::<Vec<_>>();
        S0ArtifactValidationReport {
            required_artifact_count: self.required.len() as u64,
            present_artifact_count: present_kinds.len() as u64,
            missing_required,
            schema_incompatible,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct S0ArtifactValidationReport {
    required_artifact_count: u64,
    present_artifact_count: u64,
    missing_required: Vec<S0ArtifactKind>,
    schema_incompatible: Vec<S0ArtifactKind>,
}

impl S0ArtifactValidationReport {
    pub fn required_artifact_count(&self) -> u64 {
        self.required_artifact_count
    }

    pub fn present_artifact_count(&self) -> u64 {
        self.present_artifact_count
    }

    pub fn missing_required_artifact_count(&self) -> u64 {
        self.missing_required.len() as u64
    }

    pub fn schema_incompatible_artifact_count(&self) -> u64 {
        self.schema_incompatible.len() as u64
    }

    pub fn missing_required(&self) -> &[S0ArtifactKind] {
        &self.missing_required
    }

    pub fn is_complete(&self) -> bool {
        self.missing_required.is_empty() && self.schema_incompatible.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct S0ComplexityContractName(pub(crate) &'static str);

impl S0ComplexityContractName {
    pub fn as_str(&self) -> &'static str {
        self.0
    }
}
