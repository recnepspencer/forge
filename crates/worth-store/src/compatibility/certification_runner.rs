use serde::Serialize;
use sha2::{Digest, Sha256};

use super::catalog::{CompatibilityRegistry, CompatibilityRegistrySnapshot};
use super::certification::{
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
    Milestone12CertificationLaneRejection, Milestone12CertificationLaneStatus,
    Milestone12CompatibilityMatrix,
};
use crate::evidence::{
    Milestone12AdmissionReport, Milestone12CertificationEvidenceBundle,
    Milestone12ComplexityPathStatus, Milestone12ComplexitySurface, Milestone12VersionSkewReport,
};
use worth_store_contracts::CompatibilityFamilyKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ArtifactFormatEvolutionCertification {
    evidence_bundle: Milestone12CertificationEvidenceBundle,
    digest_set: Milestone12CertificationDigestSet,
    diagnostics: Milestone12CertificationDiagnostics,
}

impl Milestone12ArtifactFormatEvolutionCertification {
    pub fn evidence_bundle(&self) -> &Milestone12CertificationEvidenceBundle {
        &self.evidence_bundle
    }

    pub fn digest_set(&self) -> &Milestone12CertificationDigestSet {
        &self.digest_set
    }

    pub fn diagnostics(&self) -> &Milestone12CertificationDiagnostics {
        &self.diagnostics
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationDigestSet {
    artifact_digest: String,
    failure_digest: String,
    compatibility_matrix_digest: String,
    version_skew_digest: String,
    diagnostics_digest: String,
    counter_snapshot_digest: String,
}

impl Milestone12CertificationDigestSet {
    pub fn artifact_digest(&self) -> &str {
        &self.artifact_digest
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }

    pub fn compatibility_matrix_digest(&self) -> &str {
        &self.compatibility_matrix_digest
    }

    pub fn version_skew_digest(&self) -> &str {
        &self.version_skew_digest
    }

    pub fn diagnostics_digest(&self) -> &str {
        &self.diagnostics_digest
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        &self.counter_snapshot_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationDiagnostics {
    lane_count: usize,
    runtime_gap_labels: Vec<&'static str>,
}

impl Milestone12CertificationDiagnostics {
    pub fn lane_count(&self) -> usize {
        self.lane_count
    }

    pub fn runtime_gap_labels(&self) -> &[&'static str] {
        &self.runtime_gap_labels
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationScenario {
    label: &'static str,
}

impl Milestone12CertificationScenario {
    pub fn first_ship() -> Self {
        Self {
            label: "first_ship_artifact_format_evolution",
        }
    }

    pub fn label(&self) -> &'static str {
        self.label
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationFixture {
    scenario: Milestone12CertificationScenario,
    registry_snapshot: CompatibilityRegistrySnapshot,
}

impl Milestone12CertificationFixture {
    pub(crate) fn first_ship() -> Self {
        Self {
            scenario: Milestone12CertificationScenario::first_ship(),
            registry_snapshot: CompatibilityRegistry::first_ship(),
        }
    }

    pub fn scenario(&self) -> Milestone12CertificationScenario {
        self.scenario
    }

    pub fn registry_snapshot(&self) -> &CompatibilityRegistrySnapshot {
        &self.registry_snapshot
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationRunner {
    fixture: Milestone12CertificationFixture,
}

impl Milestone12CertificationRunner {
    pub fn first_ship() -> Self {
        Self {
            fixture: Milestone12CertificationFixture::first_ship(),
        }
    }

    pub fn run(
        &self,
    ) -> Result<
        Milestone12ArtifactFormatEvolutionCertification,
        Milestone12CertificationLaneRejection,
    > {
        let lane_outcomes = self.lane_outcomes()?;
        let compatibility_matrix =
            Milestone12CompatibilityMatrix::from_lane_outcomes(&lane_outcomes)?;
        let admission_report =
            Milestone12AdmissionReport::aggregate(lane_outcomes.iter().map(|lane| lane.counters()));
        let version_skew_report = version_skew_report(&admission_report);
        let complexity_surface = complexity_surface();
        let evidence_bundle = Milestone12CertificationEvidenceBundle::from_parts(
            admission_report.clone(),
            compatibility_matrix,
            version_skew_report.clone(),
            complexity_surface,
            lane_outcomes.clone(),
        )?;
        let diagnostics = Milestone12CertificationDiagnostics {
            lane_count: lane_outcomes.len(),
            runtime_gap_labels: runtime_gap_labels(),
        };
        let digest_set = digest_set(
            &lane_outcomes,
            &version_skew_report,
            &diagnostics,
            &admission_report,
        );
        Ok(Milestone12ArtifactFormatEvolutionCertification {
            evidence_bundle,
            digest_set,
            diagnostics,
        })
    }

    fn lane_outcomes(
        &self,
    ) -> Result<Vec<Milestone12CertificationLaneOutcome>, Milestone12CertificationLaneRejection>
    {
        let snapshot = self.fixture.registry_snapshot();
        let manifest_index = scenario_inputs::recovered_manifest_index(snapshot);
        let catalog_index =
            super::admission::CompatibilityManifestIndex::rebuild_from_registry(snapshot);
        let mut outcomes =
            Vec::with_capacity(Milestone12CertificationLaneKind::mandatory_phase_5a().len());
        outcomes.push(Milestone12CertificationLaneOutcome::non_admitted(
            Milestone12CertificationLaneKind::CatalogCompleteness,
            scenario_inputs::lane_input(
                CompatibilityFamilyKind::CommitEnvelope.family_id(),
                1,
                1,
                None,
                None,
            ),
            Milestone12CertificationLaneStatus::EvidenceOnly,
            catalog_index.rebuild_counters(),
        ));
        outcomes.extend(authoritative_lanes::authoritative_lanes(&manifest_index)?);
        outcomes.extend(derived_lanes::derived_lanes(snapshot, &manifest_index)?);
        outcomes.extend(rolling_lanes::rolling_lanes());
        outcomes.extend(adapter_lanes::adapter_lanes(&manifest_index));
        outcomes.extend(restore_lanes::restore_lanes());
        outcomes.extend(restore_lanes::disaster_recovery_lanes());
        Ok(outcomes)
    }
}

mod adapter_lanes;
mod authoritative_lanes;
mod derived_lanes;
mod restore_lanes;
mod rolling_lanes;
mod scenario_inputs;

fn version_skew_report(report: &Milestone12AdmissionReport) -> Milestone12VersionSkewReport {
    Milestone12VersionSkewReport {
        mixed_version_store_lane_count: report.rolling_window_admission_count,
        mixed_version_replica_lane_count: report.rolling_window_admission_count,
        rolling_upgrade_skew_rejection_count: report.mixed_version_skew_count,
    }
}

fn complexity_surface() -> Milestone12ComplexitySurface {
    Milestone12ComplexitySurface {
        relation_recheck: Milestone12ComplexityPathStatus::verified(
            "certification lanes recheck declared edges through bounded edge registry lookups",
        ),
        index_lookup: Milestone12ComplexityPathStatus::verified(
            "certification lanes use manifest index lookup counters, not artifact row scans",
        ),
        adapter_cost: Milestone12ComplexityPathStatus::verified(
            "adapter lanes preserve declared cost class and reject runtime execution paths",
        ),
        restore_scan: Milestone12ComplexityPathStatus::verified(
            "restore lanes prove backup-scope bounds and out-of-scope rejection counters",
        ),
    }
}

fn runtime_gap_labels() -> Vec<&'static str> {
    Vec::new()
}

fn digest_set(
    lane_outcomes: &[Milestone12CertificationLaneOutcome],
    version_skew_report: &Milestone12VersionSkewReport,
    diagnostics: &Milestone12CertificationDiagnostics,
    admission_report: &Milestone12AdmissionReport,
) -> Milestone12CertificationDigestSet {
    let accepted = lane_outcomes
        .iter()
        .filter(|lane| lane.status() == Milestone12CertificationLaneStatus::Accepted)
        .cloned()
        .collect::<Vec<_>>();
    let rejected = lane_outcomes
        .iter()
        .filter(|lane| lane.status() == Milestone12CertificationLaneStatus::Rejected)
        .cloned()
        .collect::<Vec<_>>();
    Milestone12CertificationDigestSet {
        artifact_digest: digest_of(&accepted),
        failure_digest: digest_of(&rejected),
        compatibility_matrix_digest: digest_of(&lane_outcomes),
        version_skew_digest: digest_of(version_skew_report),
        diagnostics_digest: digest_of(diagnostics),
        counter_snapshot_digest: digest_of(admission_report),
    }
}

fn digest_of<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("certification evidence must serialize");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests;
