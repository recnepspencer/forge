use crate::identity::hash_parts;

use super::super::super::materialization::QueryCausalInspectionArtifact;
use super::super::error::CausalInspectionCertificationError;
use super::super::validation::validate_scale_slope;
use super::lane::CausalInspectionScaleFixtureSize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionScaleCounterSnapshot {
    fixture_size: CausalInspectionScaleFixtureSize,
    artifact_digest: String,
    evidence_reference_width: usize,
    anchor_derivation_slope_counter: usize,
    reference_resolution_slope_counter: usize,
    admission_slope_counter: usize,
    bridge_envelope_slope_counter: usize,
    materialization_slope_counter: usize,
    artifact_serialization_slope_counter: usize,
    bridge_unindexed_scan_count: usize,
    bridge_readmission_proof_digest: Option<String>,
    snapshot_digest: String,
}

impl CausalInspectionScaleCounterSnapshot {
    pub fn from_artifact(
        fixture_size: CausalInspectionScaleFixtureSize,
        artifact: &QueryCausalInspectionArtifact,
    ) -> Self {
        let performance = artifact.performance();
        let artifact_digest = artifact.artifact_digest().to_string();
        let evidence_reference_width = performance.bridge_binding_count();
        let anchor_derivation_slope_counter = performance.anchor_derivation_count();
        let reference_resolution_slope_counter = performance.evidence_reference_resolution_count();
        let admission_slope_counter = performance.admission_count();
        let bridge_envelope_slope_counter = performance.bridge_envelope_assembly_count();
        let materialization_slope_counter = performance.materialization_count();
        let artifact_serialization_slope_counter = performance.artifact_serialization_count();
        let bridge_unindexed_scan_count = performance.bridge_unindexed_scan_count();
        let bridge_readmission_proof_digest = artifact
            .bridge_readmission_proof_digest()
            .map(str::to_string);
        let snapshot_digest = snapshot_digest(SnapshotDigestParts {
            fixture_size,
            artifact_digest: &artifact_digest,
            evidence_reference_width,
            anchor_derivation_slope_counter,
            reference_resolution_slope_counter,
            admission_slope_counter,
            bridge_envelope_slope_counter,
            materialization_slope_counter,
            artifact_serialization_slope_counter,
            bridge_unindexed_scan_count,
            bridge_readmission_proof_digest: bridge_readmission_proof_digest.as_deref(),
        });
        Self {
            fixture_size,
            artifact_digest,
            evidence_reference_width,
            anchor_derivation_slope_counter,
            reference_resolution_slope_counter,
            admission_slope_counter,
            bridge_envelope_slope_counter,
            materialization_slope_counter,
            artifact_serialization_slope_counter,
            bridge_unindexed_scan_count,
            bridge_readmission_proof_digest,
            snapshot_digest,
        }
    }

    pub fn fixture_size(&self) -> CausalInspectionScaleFixtureSize {
        self.fixture_size
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn evidence_reference_width(&self) -> usize {
        self.evidence_reference_width
    }

    pub fn anchor_derivation_slope_counter(&self) -> usize {
        self.anchor_derivation_slope_counter
    }

    pub fn reference_resolution_slope_counter(&self) -> usize {
        self.reference_resolution_slope_counter
    }

    pub fn admission_slope_counter(&self) -> usize {
        self.admission_slope_counter
    }

    pub fn bridge_envelope_slope_counter(&self) -> usize {
        self.bridge_envelope_slope_counter
    }

    pub fn materialization_slope_counter(&self) -> usize {
        self.materialization_slope_counter
    }

    pub fn artifact_serialization_slope_counter(&self) -> usize {
        self.artifact_serialization_slope_counter
    }

    pub fn bridge_unindexed_scan_count(&self) -> usize {
        self.bridge_unindexed_scan_count
    }

    pub fn bridge_readmission_proof_digest(&self) -> Option<&str> {
        self.bridge_readmission_proof_digest.as_deref()
    }

    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    #[cfg(test)]
    pub(in crate::runtime) fn with_bridge_envelope_slope_for_tests(
        mut self,
        bridge_envelope_slope_counter: usize,
    ) -> Self {
        self.bridge_envelope_slope_counter = bridge_envelope_slope_counter;
        self.snapshot_digest = snapshot_digest(SnapshotDigestParts {
            fixture_size: self.fixture_size,
            artifact_digest: &self.artifact_digest,
            evidence_reference_width: self.evidence_reference_width,
            anchor_derivation_slope_counter: self.anchor_derivation_slope_counter,
            reference_resolution_slope_counter: self.reference_resolution_slope_counter,
            admission_slope_counter: self.admission_slope_counter,
            bridge_envelope_slope_counter,
            materialization_slope_counter: self.materialization_slope_counter,
            artifact_serialization_slope_counter: self.artifact_serialization_slope_counter,
            bridge_unindexed_scan_count: self.bridge_unindexed_scan_count,
            bridge_readmission_proof_digest: self.bridge_readmission_proof_digest.as_deref(),
        });
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionPerformanceCertificationBundle {
    small_snapshot_digest: String,
    medium_snapshot_digest: String,
    large_snapshot_digest: String,
    bridge_readmission_proof_digest: String,
    scale_slope_digest: String,
    anchor_derivation_slope_digest: String,
    reference_resolution_slope_digest: String,
    admission_slope_digest: String,
    bridge_envelope_slope_digest: String,
    materialization_slope_digest: String,
    artifact_serialization_slope_digest: String,
    scale_slope_digest_part_count: usize,
    performance_certification_digest: String,
}

impl CausalInspectionPerformanceCertificationBundle {
    pub(in crate::runtime::inspection::causal::certification) fn from_snapshots(
        small: &CausalInspectionScaleCounterSnapshot,
        medium: &CausalInspectionScaleCounterSnapshot,
        large: &CausalInspectionScaleCounterSnapshot,
    ) -> Result<Self, CausalInspectionCertificationError> {
        validate_scale_slope(small, medium, large)?;
        let bridge_readmission_proof_digest = small
            .bridge_readmission_proof_digest()
            .expect("validated scale snapshots should carry readmission proof")
            .to_string();
        let anchor_derivation_slope_digest = slope_digest(
            "causal_anchor_derivation_slope_digest_v1",
            small.anchor_derivation_slope_counter(),
            medium.anchor_derivation_slope_counter(),
            large.anchor_derivation_slope_counter(),
        );
        let reference_resolution_slope_digest = slope_digest(
            "causal_reference_resolution_slope_digest_v1",
            small.reference_resolution_slope_counter(),
            medium.reference_resolution_slope_counter(),
            large.reference_resolution_slope_counter(),
        );
        let admission_slope_digest = slope_digest(
            "causal_admission_slope_digest_v1",
            small.admission_slope_counter(),
            medium.admission_slope_counter(),
            large.admission_slope_counter(),
        );
        let bridge_envelope_slope_digest = slope_digest(
            "causal_bridge_envelope_slope_digest_v1",
            small.bridge_envelope_slope_counter(),
            medium.bridge_envelope_slope_counter(),
            large.bridge_envelope_slope_counter(),
        );
        let materialization_slope_digest = slope_digest(
            "causal_materialization_slope_digest_v1",
            small.materialization_slope_counter(),
            medium.materialization_slope_counter(),
            large.materialization_slope_counter(),
        );
        let artifact_serialization_slope_digest = hash_parts(&[
            "causal_artifact_serialization_slope_digest_v1".to_string(),
            format!("small:{}", small.artifact_serialization_slope_counter()),
            format!("medium:{}", medium.artifact_serialization_slope_counter()),
            format!("large:{}", large.artifact_serialization_slope_counter()),
        ]);
        let scale_slope_digest = hash_parts(&[
            "causal_inspection_scale_slope_digest_v1".to_string(),
            format!("anchor:{anchor_derivation_slope_digest}"),
            format!("reference:{reference_resolution_slope_digest}"),
            format!("admission:{admission_slope_digest}"),
            format!("bridge-envelope:{bridge_envelope_slope_digest}"),
            format!("materialization:{materialization_slope_digest}"),
            format!("serialization:{artifact_serialization_slope_digest}"),
        ]);
        let scale_slope_digest_part_count = 3;
        let performance_certification_digest = hash_parts(&[
            "causal_inspection_performance_certification_bundle_v1".to_string(),
            format!("small:{}", small.snapshot_digest()),
            format!("medium:{}", medium.snapshot_digest()),
            format!("large:{}", large.snapshot_digest()),
            format!("readmission:{bridge_readmission_proof_digest}"),
            format!("scale-slope:{scale_slope_digest}"),
            format!("anchor:{anchor_derivation_slope_digest}"),
            format!("reference:{reference_resolution_slope_digest}"),
            format!("admission:{admission_slope_digest}"),
            format!("bridge-envelope:{bridge_envelope_slope_digest}"),
            format!("materialization:{materialization_slope_digest}"),
            format!("serialization:{artifact_serialization_slope_digest}"),
            format!("parts:{scale_slope_digest_part_count}"),
        ]);
        Ok(Self {
            small_snapshot_digest: small.snapshot_digest().to_string(),
            medium_snapshot_digest: medium.snapshot_digest().to_string(),
            large_snapshot_digest: large.snapshot_digest().to_string(),
            bridge_readmission_proof_digest,
            scale_slope_digest,
            anchor_derivation_slope_digest,
            reference_resolution_slope_digest,
            admission_slope_digest,
            bridge_envelope_slope_digest,
            materialization_slope_digest,
            artifact_serialization_slope_digest,
            scale_slope_digest_part_count,
            performance_certification_digest,
        })
    }

    pub fn small_snapshot_digest(&self) -> &str {
        &self.small_snapshot_digest
    }

    pub fn medium_snapshot_digest(&self) -> &str {
        &self.medium_snapshot_digest
    }

    pub fn large_snapshot_digest(&self) -> &str {
        &self.large_snapshot_digest
    }

    pub fn bridge_readmission_proof_digest(&self) -> &str {
        &self.bridge_readmission_proof_digest
    }

    pub fn scale_slope_digest(&self) -> &str {
        &self.scale_slope_digest
    }

    pub fn anchor_derivation_slope_digest(&self) -> &str {
        &self.anchor_derivation_slope_digest
    }

    pub fn reference_resolution_slope_digest(&self) -> &str {
        &self.reference_resolution_slope_digest
    }

    pub fn admission_slope_digest(&self) -> &str {
        &self.admission_slope_digest
    }

    pub fn bridge_envelope_slope_digest(&self) -> &str {
        &self.bridge_envelope_slope_digest
    }

    pub fn materialization_slope_digest(&self) -> &str {
        &self.materialization_slope_digest
    }

    pub fn artifact_serialization_slope_digest(&self) -> &str {
        &self.artifact_serialization_slope_digest
    }

    pub fn scale_slope_digest_part_count(&self) -> usize {
        self.scale_slope_digest_part_count
    }

    pub fn performance_certification_digest(&self) -> &str {
        &self.performance_certification_digest
    }
}

struct SnapshotDigestParts<'a> {
    fixture_size: CausalInspectionScaleFixtureSize,
    artifact_digest: &'a str,
    evidence_reference_width: usize,
    anchor_derivation_slope_counter: usize,
    reference_resolution_slope_counter: usize,
    admission_slope_counter: usize,
    bridge_envelope_slope_counter: usize,
    materialization_slope_counter: usize,
    artifact_serialization_slope_counter: usize,
    bridge_unindexed_scan_count: usize,
    bridge_readmission_proof_digest: Option<&'a str>,
}

fn snapshot_digest(parts: SnapshotDigestParts<'_>) -> String {
    hash_parts(&[
        "causal_inspection_scale_counter_snapshot_v1".to_string(),
        format!("size:{}", parts.fixture_size.as_str()),
        format!("artifact:{}", parts.artifact_digest),
        format!("evidence-width:{}", parts.evidence_reference_width),
        format!("anchor-slope:{}", parts.anchor_derivation_slope_counter),
        format!(
            "reference-slope:{}",
            parts.reference_resolution_slope_counter
        ),
        format!("admission-slope:{}", parts.admission_slope_counter),
        format!(
            "bridge-envelope-slope:{}",
            parts.bridge_envelope_slope_counter
        ),
        format!(
            "materialization-slope:{}",
            parts.materialization_slope_counter
        ),
        format!(
            "serialization-slope:{}",
            parts.artifact_serialization_slope_counter
        ),
        format!(
            "bridge-unindexed-scan:{}",
            parts.bridge_unindexed_scan_count
        ),
        format!(
            "readmission:{}",
            parts.bridge_readmission_proof_digest.unwrap_or("none")
        ),
    ])
}

fn slope_digest(label: &str, small: usize, medium: usize, large: usize) -> String {
    hash_parts(&[
        label.to_string(),
        format!("small:{small}"),
        format!("medium:{medium}"),
        format!("large:{large}"),
    ])
}
