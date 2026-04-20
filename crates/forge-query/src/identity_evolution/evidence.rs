#![cfg_attr(not(test), allow(dead_code))]

use crate::identity::{
    BasisDigest, CanonicalQueryDigest, CounterSnapshotDigest, FailureDigest, LineageDigest,
    ResultDigest,
};

use super::{
    admission::IdentityEvolutionAdmissionError,
    contracts::IdentityEvolutionComplexityStatus,
    execution::IdentityEvolutionExecutionArtifact,
    families::IdentityEvolutionOutcomeFamily,
    metadata::{BranchLocalityClass, IdentityEvolutionMetadata},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionCounterSnapshot {
    exact_counter_values: Vec<String>,
    counter_snapshot_digest: CounterSnapshotDigest,
}

impl IdentityEvolutionCounterSnapshot {
    pub fn exact_counter_values(&self) -> &[String] {
        &self.exact_counter_values
    }

    pub fn counter_snapshot_digest(&self) -> &CounterSnapshotDigest {
        &self.counter_snapshot_digest
    }

    pub(crate) fn from_execution_artifact(
        artifact: &IdentityEvolutionExecutionArtifact,
    ) -> Self {
        let counters = artifact.counters();
        let exact_counter_values = vec![
            format!(
                "declared_lineage_complexity_contract_count:{}",
                counters.declared_lineage_complexity_contract_count()
            ),
            format!(
                "declared_correspondence_complexity_contract_count:{}",
                counters.declared_correspondence_complexity_contract_count()
            ),
            format!(
                "lineage_anchor_lookup_count:{}",
                counters.lineage_anchor_lookup_count()
            ),
            format!("lineage_step_count:{}", counters.lineage_step_count()),
            format!("predicted_lineage_width:{}", counters.predicted_lineage_width()),
            format!("realized_lineage_width:{}", counters.realized_lineage_width()),
            format!(
                "lineage_width_drift_count:{}",
                counters.lineage_width_drift_count()
            ),
            format!(
                "split_successor_fanout_width:{}",
                counters.split_successor_fanout_width()
            ),
            format!(
                "branch_local_boundary_check_count:{}",
                counters.branch_local_boundary_check_count()
            ),
            format!(
                "branch_local_divergence_count:{}",
                counters.branch_local_divergence_count()
            ),
            format!(
                "promotion_or_merge_authority_proof_check_count:{}",
                counters.promotion_or_merge_authority_proof_check_count()
            ),
            format!("identity_break_count:{}", counters.identity_break_count()),
            format!(
                "unsupported_lineage_denial_count:{}",
                counters.unsupported_lineage_denial_count()
            ),
            format!(
                "broad_lineage_scan_denial_count:{}",
                counters.broad_lineage_scan_denial_count()
            ),
            format!(
                "correspondence_candidate_count:{}",
                counters.correspondence_candidate_count()
            ),
            format!(
                "ambiguous_correspondence_count:{}",
                counters.ambiguous_correspondence_count()
            ),
            format!(
                "advisory_as_authoritative_denial_count:{}",
                counters.advisory_as_authoritative_denial_count()
            ),
            format!(
                "branch_crossing_denial_count:{}",
                counters.branch_crossing_denial_count()
            ),
            format!(
                "lineage_to_correspondence_fallback_count:{}",
                counters.lineage_to_correspondence_fallback_count()
            ),
            format!(
                "identity_evolution_metadata_attachment_count:{}",
                counters.identity_evolution_metadata_attachment_count()
            ),
            format!(
                "identity_evolution_replay_parity_count:{}",
                counters.identity_evolution_replay_parity_count()
            ),
            format!(
                "identity_evolution_executor_rediscovery_count:{}",
                counters.executor_rediscovery_count()
            ),
            format!(
                "identity_evolution_basis_rediscovery_count:{}",
                counters.identity_evolution_basis_rediscovery_count()
            ),
            format!(
                "complexity_contract_violation_denial_count:{}",
                counters.complexity_contract_violation_denial_count()
            ),
            format!(
                "complexity_status_debt_count:{}",
                counters.complexity_status_debt_count()
            ),
        ];
        let counter_snapshot_digest = CounterSnapshotDigest::from_parts(&exact_counter_values);
        Self {
            exact_counter_values,
            counter_snapshot_digest,
        }
    }

    pub(crate) fn from_admission_error(error: &IdentityEvolutionAdmissionError) -> Self {
        let exact_counter_values = vec![format!(
            "admission_failure_class:{}",
            error.failure_class().as_str()
        )];
        let counter_snapshot_digest = CounterSnapshotDigest::from_parts(&exact_counter_values);
        Self {
            exact_counter_values,
            counter_snapshot_digest,
        }
    }

    pub(crate) fn compile_fail(row_name: &'static str) -> Self {
        let exact_counter_values = vec![format!("compile_fail_row:{row_name}")];
        let counter_snapshot_digest = CounterSnapshotDigest::from_parts(&exact_counter_values);
        Self {
            exact_counter_values,
            counter_snapshot_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionCertificationResultEvidence {
    query_digest: CanonicalQueryDigest,
    basis_digest: BasisDigest,
    lineage_digest: LineageDigest,
    branch_locality_digest: ResultDigest,
    complexity_contract_digest: ResultDigest,
    result_digest: String,
    failure_digest: FailureDigest,
    outcome_family: IdentityEvolutionOutcomeFamily,
    branch_locality_class: BranchLocalityClass,
    complexity_status: IdentityEvolutionComplexityStatus,
    counter_snapshot: IdentityEvolutionCounterSnapshot,
}

impl IdentityEvolutionCertificationResultEvidence {
    pub fn query_digest(&self) -> &CanonicalQueryDigest {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn lineage_digest(&self) -> &LineageDigest {
        &self.lineage_digest
    }

    pub fn branch_locality_digest(&self) -> &ResultDigest {
        &self.branch_locality_digest
    }

    pub fn complexity_contract_digest(&self) -> &ResultDigest {
        &self.complexity_contract_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn failure_digest(&self) -> &FailureDigest {
        &self.failure_digest
    }

    pub fn outcome_family(&self) -> IdentityEvolutionOutcomeFamily {
        self.outcome_family
    }

    pub fn branch_locality_class(&self) -> BranchLocalityClass {
        self.branch_locality_class
    }

    pub fn complexity_status(&self) -> IdentityEvolutionComplexityStatus {
        self.complexity_status
    }

    pub fn counter_snapshot(&self) -> &IdentityEvolutionCounterSnapshot {
        &self.counter_snapshot
    }

    pub(crate) fn from_execution_artifact(
        artifact: &IdentityEvolutionExecutionArtifact,
    ) -> Self {
        let metadata = artifact.result_bundle().metadata();
        let failure_digest = if let Some(result) = artifact.result_bundle().as_ambiguity() {
            result.ambiguity_digest().clone()
        } else {
            FailureDigest::from_parts(&[format!(
                "result_digest:{}",
                artifact.result_digest()
            )])
        };
        Self::from_parts(
            metadata,
            artifact.result_digest().to_string(),
            failure_digest,
            IdentityEvolutionCounterSnapshot::from_execution_artifact(artifact),
        )
    }

    fn from_parts(
        metadata: &IdentityEvolutionMetadata,
        result_digest: String,
        failure_digest: FailureDigest,
        counter_snapshot: IdentityEvolutionCounterSnapshot,
    ) -> Self {
        Self {
            query_digest: metadata.query_digest().clone(),
            basis_digest: metadata.basis_digest().clone(),
            lineage_digest: metadata.lineage_digest().clone(),
            branch_locality_digest: metadata.branch_locality_digest().clone(),
            complexity_contract_digest: metadata
                .complexity_report()
                .complexity_contract_digest()
                .clone(),
            result_digest,
            failure_digest,
            outcome_family: metadata.outcome_family(),
            branch_locality_class: metadata.branch_locality_class(),
            complexity_status: metadata.complexity_report().status(),
            counter_snapshot,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdentityEvolutionCertificationDenialEvidence {
    query_digest: CanonicalQueryDigest,
    basis_digest: BasisDigest,
    lineage_digest: LineageDigest,
    branch_locality_digest: ResultDigest,
    complexity_contract_digest: ResultDigest,
    result_digest: String,
    failure_digest: FailureDigest,
    counter_snapshot: IdentityEvolutionCounterSnapshot,
}

impl IdentityEvolutionCertificationDenialEvidence {
    pub fn query_digest(&self) -> &CanonicalQueryDigest {
        &self.query_digest
    }

    pub fn basis_digest(&self) -> &BasisDigest {
        &self.basis_digest
    }

    pub fn lineage_digest(&self) -> &LineageDigest {
        &self.lineage_digest
    }

    pub fn branch_locality_digest(&self) -> &ResultDigest {
        &self.branch_locality_digest
    }

    pub fn complexity_contract_digest(&self) -> &ResultDigest {
        &self.complexity_contract_digest
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub fn failure_digest(&self) -> &FailureDigest {
        &self.failure_digest
    }

    pub fn counter_snapshot(&self) -> &IdentityEvolutionCounterSnapshot {
        &self.counter_snapshot
    }

    pub(crate) fn from_execution_artifact(
        artifact: &IdentityEvolutionExecutionArtifact,
    ) -> Self {
        let metadata = artifact.result_bundle().metadata();
        let failure_digest = if let Some(result) = artifact.result_bundle().as_denied() {
            result.denial_digest().clone()
        } else if let Some(result) = artifact.result_bundle().as_ambiguity() {
            result.ambiguity_digest().clone()
        } else {
            FailureDigest::from_parts(&[format!(
                "result_digest:{}",
                artifact.result_digest()
            )])
        };
        Self {
            query_digest: metadata.query_digest().clone(),
            basis_digest: metadata.basis_digest().clone(),
            lineage_digest: metadata.lineage_digest().clone(),
            branch_locality_digest: metadata.branch_locality_digest().clone(),
            complexity_contract_digest: metadata
                .complexity_report()
                .complexity_contract_digest()
                .clone(),
            result_digest: artifact.result_digest().to_string(),
            failure_digest,
            counter_snapshot: IdentityEvolutionCounterSnapshot::from_execution_artifact(artifact),
        }
    }

    pub(crate) fn from_admission_error(
        error: &IdentityEvolutionAdmissionError,
        query_digest: &CanonicalQueryDigest,
        basis_digest: &BasisDigest,
    ) -> Self {
        let result_digest = ResultDigest::from_parts(&[format!(
            "admission_failure_result:{}",
            error.failure_class().as_str()
        )])
        .as_str()
        .to_string();
        Self {
            query_digest: query_digest.clone(),
            basis_digest: basis_digest.clone(),
            lineage_digest: LineageDigest::from_parts(&[format!(
                "admission_failure:{}",
                error.failure_class().as_str()
            )]),
            branch_locality_digest: ResultDigest::from_parts(&[format!(
                "admission_failure_branch_locality:{}",
                error.failure_class().as_str()
            )]),
            complexity_contract_digest: ResultDigest::from_parts(&[format!(
                "admission_failure_contract:{}",
                error.failure_class().as_str()
            )]),
            result_digest,
            failure_digest: error.failure_digest().clone(),
            counter_snapshot: IdentityEvolutionCounterSnapshot::from_admission_error(error),
        }
    }

    pub(crate) fn compile_fail(
        row_name: &'static str,
        query_digest: &CanonicalQueryDigest,
        basis_digest: &BasisDigest,
    ) -> Self {
        let result_digest = ResultDigest::from_parts(&[format!(
            "compile_fail_result:{row_name}"
        )])
        .as_str()
        .to_string();
        Self {
            query_digest: query_digest.clone(),
            basis_digest: basis_digest.clone(),
            lineage_digest: LineageDigest::from_parts(&[format!(
                "compile_fail_lineage:{row_name}"
            )]),
            branch_locality_digest: ResultDigest::from_parts(&[format!(
                "compile_fail_branch_locality:{row_name}"
            )]),
            complexity_contract_digest: ResultDigest::from_parts(&[format!(
                "compile_fail_contract:{row_name}"
            )]),
            result_digest,
            failure_digest: FailureDigest::from_parts(&[format!("compile_fail:{row_name}")]),
            counter_snapshot: IdentityEvolutionCounterSnapshot::compile_fail(row_name),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityEvolutionCertificationEvidence {
    Result(IdentityEvolutionCertificationResultEvidence),
    Denial(IdentityEvolutionCertificationDenialEvidence),
}

impl IdentityEvolutionCertificationEvidence {
    pub fn as_result(&self) -> Option<&IdentityEvolutionCertificationResultEvidence> {
        match self {
            Self::Result(evidence) => Some(evidence),
            Self::Denial(_) => None,
        }
    }

    pub fn as_denial(&self) -> Option<&IdentityEvolutionCertificationDenialEvidence> {
        match self {
            Self::Result(_) => None,
            Self::Denial(evidence) => Some(evidence),
        }
    }
}
