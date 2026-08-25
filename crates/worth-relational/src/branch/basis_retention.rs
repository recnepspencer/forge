use super::{
    basis_identity_validation::require_local_branch_identity, AdmittedRelationalBranchBasis,
    RelationalBranchBasisDenial,
};
use crate::history::retention::{
    RelationalComponentBasisRetentionLease, RelationalComponentBasisRetentionReleaseDenial,
    RelationalComponentBasisRetentionReleaseReceipt,
};
use crate::runtime::RelationalRuntime;

impl RelationalRuntime {
    pub fn retain_component_basis(
        &self,
        basis: &AdmittedRelationalBranchBasis,
    ) -> Result<RelationalComponentBasisRetentionLease, RelationalBranchBasisDenial> {
        require_local_branch_identity(self, basis.identity())?;
        let lease = RelationalComponentBasisRetentionLease::new(
            basis.observation(),
            self.services
                .instrumentation
                .external_retention_terminal_accounting(),
        );
        self.services.instrumentation.count_basis(|counters| {
            counters.external_retention_acquires =
                counters.external_retention_acquires.saturating_add(1);
        });
        Ok(lease)
    }

    pub fn release_component_basis(
        &self,
        lease: RelationalComponentBasisRetentionLease,
    ) -> Result<
        RelationalComponentBasisRetentionReleaseReceipt,
        RelationalComponentBasisRetentionReleaseDenial,
    > {
        if lease.descriptor().runtime_instance_id() != self.runtime_instance_id() {
            let denial = RelationalBranchBasisDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: lease.descriptor().runtime_instance_id(),
            };
            return Err(RelationalComponentBasisRetentionReleaseDenial::new(
                denial, lease,
            ));
        }
        Ok(lease.release())
    }
}
