use std::collections::BTreeSet;

use serde::Serialize;

use super::admission::Milestone12AdmissionReport;
use super::{
    admission::Milestone12VersionSkewReport,
    contracts::{Milestone12ComplexitySurface, Milestone12CounterContract},
    matrix::Milestone12CompatibilityMatrixRow,
};
use crate::compatibility::{
    Milestone12CertificationLaneKind, Milestone12CertificationLaneOutcome,
    Milestone12CertificationLaneRejection, Milestone12CertificationRunSummary,
    Milestone12CompatibilityMatrix,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationSummary {
    pub phase_1_registry_declared: bool,
    pub phase_1_counters_declared: bool,
    pub phase_1_witness_boundaries_declared: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationBundle {
    pub summary: Milestone12CertificationSummary,
    pub counter_contract: Milestone12CounterContract,
    pub complexity_surface: Milestone12ComplexitySurface,
    pub admission_report: Milestone12AdmissionReport,
    pub matrix_rows: Vec<Milestone12CompatibilityMatrixRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12ArtifactFormatEvolutionEvidence {
    lane_count: u64,
}

impl Milestone12ArtifactFormatEvolutionEvidence {
    pub fn lane_count(&self) -> u64 {
        self.lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12RollingCompatibilityEvidence {
    admitted_lane_count: u64,
    rejected_lane_count: u64,
}

impl Milestone12RollingCompatibilityEvidence {
    pub fn admitted_lane_count(&self) -> u64 {
        self.admitted_lane_count
    }

    pub fn rejected_lane_count(&self) -> u64 {
        self.rejected_lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12RestoreCompatibilityEvidence {
    admitted_lane_count: u64,
    rejected_lane_count: u64,
}

impl Milestone12RestoreCompatibilityEvidence {
    pub fn admitted_lane_count(&self) -> u64 {
        self.admitted_lane_count
    }

    pub fn rejected_lane_count(&self) -> u64 {
        self.rejected_lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12DerivedCompatibilityEvidence {
    admitted_lane_count: u64,
    non_admitted_lane_count: u64,
}

impl Milestone12DerivedCompatibilityEvidence {
    pub fn admitted_lane_count(&self) -> u64 {
        self.admitted_lane_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone12CertificationEvidenceBundle {
    admission_report: Milestone12AdmissionReport,
    compatibility_matrix: Milestone12CompatibilityMatrix,
    version_skew_report: Milestone12VersionSkewReport,
    complexity_surface: Milestone12ComplexitySurface,
    lane_outcomes: Vec<Milestone12CertificationLaneOutcome>,
    run_summary: Milestone12CertificationRunSummary,
    artifact_format_evidence: Milestone12ArtifactFormatEvolutionEvidence,
    rolling_evidence: Milestone12RollingCompatibilityEvidence,
    restore_evidence: Milestone12RestoreCompatibilityEvidence,
    derived_evidence: Milestone12DerivedCompatibilityEvidence,
}

impl Milestone12CertificationEvidenceBundle {
    pub fn from_parts(
        admission_report: Milestone12AdmissionReport,
        compatibility_matrix: Milestone12CompatibilityMatrix,
        version_skew_report: Milestone12VersionSkewReport,
        complexity_surface: Milestone12ComplexitySurface,
        mut lane_outcomes: Vec<Milestone12CertificationLaneOutcome>,
    ) -> Result<Self, Milestone12CertificationLaneRejection> {
        let matrix_ids = compatibility_matrix
            .entries()
            .iter()
            .map(|entry| entry.lane_id().clone())
            .collect::<BTreeSet<_>>();
        let outcome_ids = lane_outcomes
            .iter()
            .map(|outcome| outcome.lane_id().clone())
            .collect::<BTreeSet<_>>();
        if matrix_ids != outcome_ids {
            return Err(Milestone12CertificationLaneRejection::MatrixLaneMismatch);
        }
        for outcome in &lane_outcomes {
            if !outcome.counters().has_counter_evidence() {
                return Err(Milestone12CertificationLaneRejection::CounterEvidenceMissing);
            }
        }
        lane_outcomes.sort_by_key(|outcome| outcome.lane_id().clone());
        let run_summary = Milestone12CertificationRunSummary::from_outcomes(&lane_outcomes);
        let artifact_format_evidence = Milestone12ArtifactFormatEvolutionEvidence {
            lane_count: lane_outcomes.len() as u64,
        };
        let rolling_evidence = rolling_evidence(&lane_outcomes);
        let restore_evidence = restore_evidence(&lane_outcomes);
        let derived_evidence = derived_evidence(&lane_outcomes);
        Ok(Self {
            admission_report,
            compatibility_matrix,
            version_skew_report,
            complexity_surface,
            lane_outcomes,
            run_summary,
            artifact_format_evidence,
            rolling_evidence,
            restore_evidence,
            derived_evidence,
        })
    }

    pub fn lane_outcomes(&self) -> &[Milestone12CertificationLaneOutcome] {
        &self.lane_outcomes
    }

    pub fn run_summary(&self) -> &Milestone12CertificationRunSummary {
        &self.run_summary
    }

    pub fn rolling_evidence(&self) -> &Milestone12RollingCompatibilityEvidence {
        &self.rolling_evidence
    }

    pub fn restore_evidence(&self) -> &Milestone12RestoreCompatibilityEvidence {
        &self.restore_evidence
    }
}

fn rolling_evidence(
    outcomes: &[Milestone12CertificationLaneOutcome],
) -> Milestone12RollingCompatibilityEvidence {
    let mut admitted_lane_count = 0;
    let mut rejected_lane_count = 0;
    for outcome in outcomes {
        match outcome.lane_kind() {
            Milestone12CertificationLaneKind::RollingTwoCapabilityAdmitted => {
                admitted_lane_count += 1;
            }
            Milestone12CertificationLaneKind::RollingMultiWriterRejected
            | Milestone12CertificationLaneKind::RollingMissingEdgeRejected
            | Milestone12CertificationLaneKind::RollingAdapterEdgeRejected => {
                rejected_lane_count += 1;
            }
            _ => {}
        }
    }
    Milestone12RollingCompatibilityEvidence {
        admitted_lane_count,
        rejected_lane_count,
    }
}

fn restore_evidence(
    outcomes: &[Milestone12CertificationLaneOutcome],
) -> Milestone12RestoreCompatibilityEvidence {
    let mut admitted_lane_count = 0;
    let mut rejected_lane_count = 0;
    for outcome in outcomes {
        match outcome.lane_kind() {
            Milestone12CertificationLaneKind::RestoreScopedBackupAdmitted => {
                admitted_lane_count += 1;
            }
            Milestone12CertificationLaneKind::RestoreOutOfScopeRejected
            | Milestone12CertificationLaneKind::RestorePublicationConflictRejected
            | Milestone12CertificationLaneKind::RestoreMissingEdgeRejected => {
                rejected_lane_count += 1;
            }
            _ => {}
        }
    }
    Milestone12RestoreCompatibilityEvidence {
        admitted_lane_count,
        rejected_lane_count,
    }
}

fn derived_evidence(
    outcomes: &[Milestone12CertificationLaneOutcome],
) -> Milestone12DerivedCompatibilityEvidence {
    let mut admitted_lane_count = 0;
    let mut non_admitted_lane_count = 0;
    for outcome in outcomes {
        match outcome.lane_kind() {
            Milestone12CertificationLaneKind::DerivedSnapshotReuseAccepted
            | Milestone12CertificationLaneKind::MaintenanceSummaryRebuildAdmitted
            | Milestone12CertificationLaneKind::TierManifestNonAuthorityPreserved => {
                admitted_lane_count += 1;
            }
            Milestone12CertificationLaneKind::DerivedLayoutBasisRejected
            | Milestone12CertificationLaneKind::DerivedBulkResumeRejected => {
                non_admitted_lane_count += 1;
            }
            _ => {}
        }
    }
    Milestone12DerivedCompatibilityEvidence {
        admitted_lane_count,
        non_admitted_lane_count,
    }
}
