use super::{
    basis_axis_validation::{
        reject_cross_branch_target_substitution, require_current_descriptor_axes,
    },
    basis_descriptor_resolution::resolve_relational_branch_basis_descriptor,
    basis_identity_validation::identity_denial,
    basis_observation::issue_admitted_relational_branch_basis,
    AdmittedRelationalBranchBasis, RelationalBranchBasisDenial, RelationalBranchBasisDescriptor,
    RelationalBranchRoot,
};
use crate::runtime::RelationalRuntime;

impl RelationalRuntime {
    /// Resolve a transported descriptor and readmit it through live owner
    /// state or an already-retained exact immutable basis.
    pub fn readmit_branch_basis(
        &self,
        descriptor: &RelationalBranchBasisDescriptor,
    ) -> Result<AdmittedRelationalBranchBasis, RelationalBranchBasisDenial> {
        let result = self.readmit_branch_basis_inner(descriptor);
        self.services
            .instrumentation
            .count_basis(|counters| counters.record_readmission(&result));
        result
    }

    fn readmit_branch_basis_inner(
        &self,
        descriptor: &RelationalBranchBasisDescriptor,
    ) -> Result<AdmittedRelationalBranchBasis, RelationalBranchBasisDenial> {
        let descriptor =
            resolve_relational_branch_basis_descriptor(descriptor.clone())?.into_descriptor();
        if descriptor.runtime_instance_id() != self.runtime_instance_id() {
            return Err(RelationalBranchBasisDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: descriptor.runtime_instance_id(),
            });
        }
        let identity = self
            .branch_identity(descriptor.branch_id())
            .map_err(identity_denial)?;
        let branch_cell = self
            .history
            .branch_cell(descriptor.branch_id())
            .ok_or_else(|| {
                RelationalBranchBasisDenial::UnknownBranch(descriptor.branch_id().clone())
            })?;
        if let Some(retained) = branch_cell.readmit_retained_basis(&descriptor) {
            return Ok(retained);
        }
        let (current_reference, current_truth_version, root) = (
            branch_cell.observation().clone(),
            branch_cell.truth_version(),
            branch_cell.root().cloned().unwrap_or_else(|| {
                RelationalBranchRoot::empty_with_schema(
                    &self.config.schema.registry,
                    crate::schema::data::runtime_descriptor_semantics_policy()
                        .current_write_version(),
                )
            }),
        );
        reject_cross_branch_target_substitution(
            &self.history.commit_catalog,
            branch_cell,
            &descriptor,
            &current_reference,
        )?;
        require_current_descriptor_axes(
            &descriptor,
            &current_reference,
            current_truth_version,
            &root,
        )?;
        let basis = issue_admitted_relational_branch_basis(descriptor, identity, root);
        branch_cell.register_basis(basis)
    }
}
