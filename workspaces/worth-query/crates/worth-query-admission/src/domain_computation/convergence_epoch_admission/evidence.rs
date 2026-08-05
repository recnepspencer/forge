use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryCandidateOptimalityPosture, WorthQueryCandidateSearchPosture,
    WorthQueryConvergenceIncumbentPosture, WorthQueryConvergenceOscillationPosture,
    WorthQueryInstallationAdmissionIdentity, WorthQueryInstallationGeneration,
    WorthQueryInstalledArtifactContractAuthority,
};

use super::WorthQueryConvergenceAdmissionCounters;

pub struct WorthQueryAdmittedConvergenceContract {
    identity: Arc<str>,
    operation_identity: Arc<str>,
    operation_owner: Arc<str>,
    runtime_ordinal: u64,
    generation: WorthQueryInstallationGeneration,
    artifact_authority: WorthQueryInstalledArtifactContractAuthority,
    artifact_contract_identity: Arc<str>,
    evidence_stage_identity: Option<Arc<str>>,
    resource_contract_identity: Arc<str>,
    universe_family: Arc<str>,
    termination_family: Arc<str>,
    feasibility_family: Arc<str>,
    comparison_family: Arc<str>,
    incumbent_family: Arc<str>,
    progress_measure_family: Arc<str>,
    comparator_family: Arc<str>,
    repeated_state_family: Arc<str>,
    search_posture: WorthQueryCandidateSearchPosture,
    optimality_posture: WorthQueryCandidateOptimalityPosture,
    incumbent_posture: WorthQueryConvergenceIncumbentPosture,
    oscillation_posture: WorthQueryConvergenceOscillationPosture,
    iteration_bound: usize,
    counters: WorthQueryConvergenceAdmissionCounters,
}

pub(super) struct WorthQueryConvergenceContractBinding {
    pub artifact_contract_identity: Arc<str>,
    pub evidence_stage_identity: Option<Arc<str>>,
    pub resource_contract_identity: Arc<str>,
    pub universe_family: Arc<str>,
    pub termination_family: Arc<str>,
    pub feasibility_family: Arc<str>,
    pub comparison_family: Arc<str>,
    pub incumbent_family: Arc<str>,
    pub progress_measure_family: Arc<str>,
    pub comparator_family: Arc<str>,
    pub repeated_state_family: Arc<str>,
    pub search_posture: WorthQueryCandidateSearchPosture,
    pub optimality_posture: WorthQueryCandidateOptimalityPosture,
    pub incumbent_posture: WorthQueryConvergenceIncumbentPosture,
    pub oscillation_posture: WorthQueryConvergenceOscillationPosture,
    pub iteration_bound: usize,
}

impl WorthQueryAdmittedConvergenceContract {
    pub(super) fn new(
        identity: Arc<str>,
        operation_identity: Arc<str>,
        operation_owner: Arc<str>,
        runtime_ordinal: u64,
        generation: WorthQueryInstallationGeneration,
        artifact_authority: WorthQueryInstalledArtifactContractAuthority,
        binding: WorthQueryConvergenceContractBinding,
        counters: WorthQueryConvergenceAdmissionCounters,
    ) -> Self {
        Self {
            identity,
            operation_identity,
            operation_owner,
            runtime_ordinal,
            generation,
            artifact_authority,
            artifact_contract_identity: binding.artifact_contract_identity,
            evidence_stage_identity: binding.evidence_stage_identity,
            resource_contract_identity: binding.resource_contract_identity,
            universe_family: binding.universe_family,
            termination_family: binding.termination_family,
            feasibility_family: binding.feasibility_family,
            comparison_family: binding.comparison_family,
            incumbent_family: binding.incumbent_family,
            progress_measure_family: binding.progress_measure_family,
            comparator_family: binding.comparator_family,
            repeated_state_family: binding.repeated_state_family,
            search_posture: binding.search_posture,
            optimality_posture: binding.optimality_posture,
            incumbent_posture: binding.incumbent_posture,
            oscillation_posture: binding.oscillation_posture,
            iteration_bound: binding.iteration_bound,
            counters,
        }
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }

    pub fn operation_owner(&self) -> &str {
        &self.operation_owner
    }

    pub const fn runtime_ordinal(&self) -> u64 {
        self.runtime_ordinal
    }

    pub const fn generation(&self) -> WorthQueryInstallationGeneration {
        self.generation
    }

    pub fn artifact_admission_identity(&self) -> &WorthQueryInstallationAdmissionIdentity {
        self.artifact_authority.admission_identity()
    }

    pub fn artifact_contract_identity(&self) -> &str {
        &self.artifact_contract_identity
    }

    pub fn evidence_stage_identity(&self) -> Option<&str> {
        self.evidence_stage_identity.as_deref()
    }

    pub fn resource_contract_identity(&self) -> &str {
        &self.resource_contract_identity
    }

    pub fn universe_family(&self) -> &str {
        &self.universe_family
    }

    pub fn termination_family(&self) -> &str {
        &self.termination_family
    }

    pub fn feasibility_family(&self) -> &str {
        &self.feasibility_family
    }

    pub fn comparison_family(&self) -> &str {
        &self.comparison_family
    }

    pub fn incumbent_family(&self) -> &str {
        &self.incumbent_family
    }

    pub fn progress_measure_family(&self) -> &str {
        &self.progress_measure_family
    }

    pub fn comparator_family(&self) -> &str {
        &self.comparator_family
    }

    pub fn repeated_state_family(&self) -> &str {
        &self.repeated_state_family
    }

    pub fn search_posture(&self) -> &WorthQueryCandidateSearchPosture {
        &self.search_posture
    }

    pub fn optimality_posture(&self) -> &WorthQueryCandidateOptimalityPosture {
        &self.optimality_posture
    }

    pub const fn incumbent_posture(&self) -> WorthQueryConvergenceIncumbentPosture {
        self.incumbent_posture
    }

    pub const fn oscillation_posture(&self) -> WorthQueryConvergenceOscillationPosture {
        self.oscillation_posture
    }

    pub const fn iteration_bound(&self) -> usize {
        self.iteration_bound
    }

    pub const fn counters(&self) -> WorthQueryConvergenceAdmissionCounters {
        self.counters
    }
}
