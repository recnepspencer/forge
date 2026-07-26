#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryConvergenceAdmissionCounters {
    installed_authority_check_count: usize,
    operation_evidence_check_count: usize,
    convergence_contract_check_count: usize,
}

impl WorthQueryConvergenceAdmissionCounters {
    pub(super) fn checked_installed_authority(&mut self) {
        self.installed_authority_check_count += 1;
    }

    pub(super) fn checked_operation_evidence(&mut self) {
        self.operation_evidence_check_count += 1;
    }

    pub(super) fn checked_convergence_contract(&mut self) {
        self.convergence_contract_check_count += 1;
    }

    pub const fn installed_authority_check_count(self) -> usize {
        self.installed_authority_check_count
    }

    pub const fn operation_evidence_check_count(self) -> usize {
        self.operation_evidence_check_count
    }

    pub const fn convergence_contract_check_count(self) -> usize {
        self.convergence_contract_check_count
    }
}
