use std::sync::Arc;

use super::{
    authority::issue_relational_branch_observation_authority,
    basis_axis_validation::require_root_matches_reference,
    basis_identity_validation::require_local_branch_identity, AdmittedRelationalBranchBasis,
    AdmittedRelationalBranchBasisInner, RelationalBranchBasisDenial,
    RelationalBranchBasisDescriptor, RelationalBranchIdentity, RelationalBranchRoot,
};
use crate::runtime::RelationalRuntime;

pub(crate) fn issue_admitted_relational_branch_basis(
    descriptor: RelationalBranchBasisDescriptor,
    identity: RelationalBranchIdentity,
    root: Arc<RelationalBranchRoot>,
) -> AdmittedRelationalBranchBasis {
    AdmittedRelationalBranchBasis {
        inner: Arc::new(AdmittedRelationalBranchBasisInner {
            descriptor,
            identity,
            root,
            _authority: issue_relational_branch_observation_authority(),
            retention: crate::history::retention::RelationalObservationRetentionObligation::new(),
            registry_lease: std::sync::OnceLock::new(),
        }),
    }
}

impl RelationalRuntime {
    /// Observe and admit one exact owner branch basis from one cell snapshot.
    pub fn observe_branch(
        &self,
        identity: &RelationalBranchIdentity,
    ) -> Result<
        (
            RelationalBranchBasisDescriptor,
            AdmittedRelationalBranchBasis,
        ),
        RelationalBranchBasisDenial,
    > {
        require_local_branch_identity(self, identity)?;
        let (descriptor, root) = {
            let cell = self
                .history
                .branch_cell(identity.branch_id())
                .ok_or_else(|| {
                    RelationalBranchBasisDenial::UnknownBranch(identity.branch_id().clone())
                })?;
            let root = cell.root().cloned().unwrap_or_else(|| {
                RelationalBranchRoot::empty_with_schema(
                    &self.config.schema.registry,
                    crate::schema::data::runtime_descriptor_semantics_policy()
                        .current_write_version(),
                )
            });
            let descriptor = descriptor_for_cell(cell, &root)?;
            (descriptor, root)
        };
        let basis =
            issue_admitted_relational_branch_basis(descriptor.clone(), identity.clone(), root);
        let basis = self
            .history
            .branch_cell(identity.branch_id())
            .expect("observed branch remains registered")
            .register_basis(basis)?;
        self.services.instrumentation.count_basis(|counters| {
            counters.basis_observations = counters.basis_observations.saturating_add(1);
        });
        Ok((descriptor, basis))
    }
}

fn descriptor_for_cell(
    cell: &super::RelationalBranchReferenceCell,
    root: &Arc<RelationalBranchRoot>,
) -> Result<RelationalBranchBasisDescriptor, RelationalBranchBasisDenial> {
    let visibility_commitment = root
        .axes()
        .map(|axes| axes.visibility.digest())
        .unwrap_or([0; 32]);
    let schema_commitment = root.schema_authority().registry().authority_digest_bytes();
    require_root_matches_reference(cell.observation(), root)?;
    Ok(RelationalBranchBasisDescriptor::live(
        super::basis::RelationalLiveBranchBasisDescriptorAxes {
            runtime_instance_id: cell.identity().runtime_instance_id(),
            branch_id: cell.identity().branch_id().clone(),
            reference: cell.observation().clone(),
            truth_version: cell.truth_version(),
            root_identity: root.id(),
            schema_commitment,
            visibility_commitment,
        },
    ))
}
