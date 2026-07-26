#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryConvergenceEpochCounters {
    operation_authority_check_count: usize,
    contract_authority_check_count: usize,
    managed_run_authority_check_count: usize,
    graph_authority_check_count: usize,
    iteration_count: usize,
    provider_work_unit_count: u64,
    comparator_call_count: usize,
    progress_check_count: usize,
    repeated_state_probe_count: usize,
    incumbent_retention_count: usize,
    incumbent_replacement_count: usize,
    yield_count: usize,
    resume_count: usize,
    cleanup_count: usize,
}

impl WorthQueryConvergenceEpochCounters {
    pub(super) fn checked_operation_authority(&mut self) {
        self.operation_authority_check_count += 1;
    }

    pub(super) fn checked_contract_authority(&mut self) {
        self.contract_authority_check_count += 1;
    }

    pub(super) fn checked_managed_run_authority(&mut self) {
        self.managed_run_authority_check_count += 1;
    }

    pub(super) fn checked_graph_authority(&mut self) {
        self.graph_authority_check_count += 1;
    }

    pub(super) fn began_iteration(&mut self) {
        self.iteration_count += 1;
    }

    pub(super) fn recorded_provider_work(&mut self, provider_work_units: u64) {
        self.provider_work_unit_count = self
            .provider_work_unit_count
            .saturating_add(provider_work_units);
    }

    pub(super) fn reconciled_provider_work_total(&mut self, provider_work_units: u64) {
        self.provider_work_unit_count = provider_work_units;
    }

    pub(super) fn recorded_domain_work(
        &mut self,
        work: &super::WorthQueryConvergenceDomainWorkEvidence,
    ) {
        self.comparator_call_count = self
            .comparator_call_count
            .saturating_add(work.comparator_call_count());
        self.progress_check_count = self
            .progress_check_count
            .saturating_add(work.progress_check_count());
        self.repeated_state_probe_count = self
            .repeated_state_probe_count
            .saturating_add(work.repeated_state_probe_count());
    }

    pub(super) fn retained_incumbent(&mut self) {
        self.incumbent_retention_count += 1;
    }

    pub(super) fn replaced_incumbent(&mut self) {
        self.incumbent_replacement_count += 1;
    }

    pub(super) fn yielded(&mut self) {
        self.yield_count += 1;
    }

    pub(super) fn resumed(&mut self) {
        self.resume_count += 1;
    }

    pub(super) fn cleaned_up(&mut self) {
        self.cleanup_count += 1;
    }

    pub fn operation_authority_check_count(&self) -> usize {
        self.operation_authority_check_count
    }

    pub fn contract_authority_check_count(&self) -> usize {
        self.contract_authority_check_count
    }

    pub fn managed_run_authority_check_count(&self) -> usize {
        self.managed_run_authority_check_count
    }

    pub fn graph_authority_check_count(&self) -> usize {
        self.graph_authority_check_count
    }

    pub fn iteration_count(&self) -> usize {
        self.iteration_count
    }

    pub fn provider_work_unit_count(&self) -> u64 {
        self.provider_work_unit_count
    }

    pub fn comparator_call_count(&self) -> usize {
        self.comparator_call_count
    }

    pub fn progress_check_count(&self) -> usize {
        self.progress_check_count
    }

    pub fn repeated_state_probe_count(&self) -> usize {
        self.repeated_state_probe_count
    }

    pub fn incumbent_retention_count(&self) -> usize {
        self.incumbent_retention_count
    }

    pub fn incumbent_replacement_count(&self) -> usize {
        self.incumbent_replacement_count
    }

    pub fn yield_count(&self) -> usize {
        self.yield_count
    }

    pub fn resume_count(&self) -> usize {
        self.resume_count
    }

    pub fn cleanup_count(&self) -> usize {
        self.cleanup_count
    }
}
