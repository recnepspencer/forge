mod boundary;
mod outputs;
mod performance;
mod phase_manifest;
mod proof_shape;
mod representatives;

use crate::identity::hash_parts;

use super::counters::BasisEligibilityCounters;
use super::taxonomy::BasisFamily;
pub use boundary::{
    basis_lifecycle_public_boundary_audit, basis_lifecycle_public_boundary_audit_digest,
    BasisLifecyclePublicBoundaryAudit, BasisLifecyclePublicBoundaryAuditRow,
    BasisLifecyclePublicBoundarySurface,
};
use outputs::certification_output_digests;
pub use performance::{
    certify_basis_lifecycle_performance_slopes, BasisLifecyclePerformanceSlopeReport,
    BasisLifecycleSlopeDigest, BasisLifecycleSlopeFamily,
};
pub use phase_manifest::{
    basis_lifecycle_phase_artifact_manifest_digest, basis_lifecycle_phase_manifest,
    basis_lifecycle_typestate_transition_digest, BasisLifecyclePhaseArtifact,
    BasisLifecyclePhaseManifest, BasisLifecyclePhaseManifestRow,
};
pub use proof_shape::{
    basis_lifecycle_phase_progression_digest, basis_lifecycle_proof_shape_audit,
    basis_lifecycle_proof_shape_audit_digest, BasisLifecycleProofShapeAudit,
    BasisLifecycleProofShapeAuditRow, BasisLifecycleProofShapeEnforcement,
    BasisLifecycleProofShapeViolation,
};
use representatives::certification_rows;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleCertificationLane {
    Admitted,
    Advisory,
    Denied,
    LowerRuntimeMismatch,
    FutureNeighborDenial,
    Performance,
}

impl BasisLifecycleCertificationLane {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
            Self::LowerRuntimeMismatch => "lower_runtime_mismatch",
            Self::FutureNeighborDenial => "future_neighbor_denial",
            Self::Performance => "performance",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLifecycleCertificationOutputPosture {
    Certified,
    Deferred,
}

impl BasisLifecycleCertificationOutputPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleCertificationRow {
    lane: BasisLifecycleCertificationLane,
    basis_family: BasisFamily,
    operation_lane: &'static str,
    artifact_digest: String,
    failure_digest: Option<String>,
    counter_snapshot_digest: String,
    row_digest: String,
}

impl BasisLifecycleCertificationRow {
    fn new(
        lane: BasisLifecycleCertificationLane,
        basis_family: BasisFamily,
        operation_lane: &'static str,
        artifact_digest: String,
        failure_digest: Option<String>,
        counter_snapshot_digest: String,
    ) -> Self {
        let row_digest = hash_parts(&[
            "basis_lifecycle_certification_row_v1".to_string(),
            format!("lane:{}", lane.as_str()),
            format!("family:{}", basis_family.as_str()),
            format!("operation_lane:{operation_lane}"),
            format!("artifact:{artifact_digest}"),
            format!("failure:{}", failure_digest.as_deref().unwrap_or("none")),
            format!("counters:{counter_snapshot_digest}"),
        ]);
        Self {
            lane,
            basis_family,
            operation_lane,
            artifact_digest,
            failure_digest,
            counter_snapshot_digest,
            row_digest,
        }
    }

    pub fn lane(&self) -> BasisLifecycleCertificationLane {
        self.lane
    }

    pub fn basis_family(&self) -> BasisFamily {
        self.basis_family
    }

    pub fn operation_lane(&self) -> &'static str {
        self.operation_lane
    }

    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleCertificationOutputDigest {
    name: &'static str,
    posture: BasisLifecycleCertificationOutputPosture,
    digest: String,
}

impl BasisLifecycleCertificationOutputDigest {
    fn certified(name: &'static str, digest: impl Into<String>) -> Self {
        Self::with_posture(
            name,
            BasisLifecycleCertificationOutputPosture::Certified,
            digest,
        )
    }

    fn with_posture(
        name: &'static str,
        posture: BasisLifecycleCertificationOutputPosture,
        digest: impl Into<String>,
    ) -> Self {
        Self {
            name,
            posture,
            digest: digest.into(),
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn posture(&self) -> BasisLifecycleCertificationOutputPosture {
        self.posture
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleCertificationBundle {
    rows: Vec<BasisLifecycleCertificationRow>,
    output_digests: Vec<BasisLifecycleCertificationOutputDigest>,
    certification_bundle_digest: String,
    counters: BasisEligibilityCounters,
}

impl BasisLifecycleCertificationBundle {
    fn new(
        rows: Vec<BasisLifecycleCertificationRow>,
        output_digests: Vec<BasisLifecycleCertificationOutputDigest>,
    ) -> Self {
        let counters = BasisEligibilityCounters::certification_bundle_assembly(rows.len());
        let certification_bundle_digest = hash_parts(&[
            "basis_lifecycle_certification_bundle_v1".to_string(),
            format!("rows:{}", rows_digest(&rows)),
            format!("outputs:{}", outputs_digest(&output_digests)),
            format!("counters:{}", counters.digest()),
        ]);
        Self {
            rows,
            output_digests,
            certification_bundle_digest,
            counters,
        }
    }

    pub fn rows(&self) -> &[BasisLifecycleCertificationRow] {
        &self.rows
    }

    pub fn output_digests(&self) -> &[BasisLifecycleCertificationOutputDigest] {
        &self.output_digests
    }

    pub fn output_digest(&self, name: &str) -> Option<&str> {
        self.output_digests
            .iter()
            .find(|output| output.name() == name)
            .map(|output| output.digest())
    }

    pub fn certification_bundle_digest(&self) -> &str {
        &self.certification_bundle_digest
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

pub fn certify_basis_lifecycle() -> BasisLifecycleCertificationBundle {
    let rows = certification_rows();
    let output_digests = certification_output_digests(&rows);
    BasisLifecycleCertificationBundle::new(rows, output_digests)
}

fn rows_digest(rows: &[BasisLifecycleCertificationRow]) -> String {
    hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect::<Vec<_>>(),
    )
}

fn outputs_digest(outputs: &[BasisLifecycleCertificationOutputDigest]) -> String {
    hash_parts(
        &outputs
            .iter()
            .map(|output| {
                format!(
                    "{}:{}:{}",
                    output.name(),
                    output.posture().as_str(),
                    output.digest()
                )
            })
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests;
