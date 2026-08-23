use std::sync::Arc;

use super::RelationalBranchReferenceCell;

impl RelationalBranchReferenceCell {
    pub(crate) fn root(&self) -> Option<&Arc<super::super::RelationalBranchRoot>> {
        self.root.as_ref()
    }

    pub(crate) fn install_root(&mut self, root: Arc<super::super::RelationalBranchRoot>) {
        self.root = Some(root);
    }

    pub(crate) fn register_basis(
        &self,
        basis: super::super::AdmittedRelationalBranchBasis,
    ) -> Result<
        super::super::AdmittedRelationalBranchBasis,
        super::super::RelationalBranchBasisDenial,
    > {
        self.basis_registry.register(basis)
    }

    pub(crate) fn readmit_retained_basis(
        &self,
        descriptor: &super::super::RelationalBranchBasisDescriptor,
    ) -> Option<super::super::AdmittedRelationalBranchBasis> {
        self.basis_registry.readmit_retained(descriptor)
    }

    pub(crate) fn bind_basis_registry_metrics(
        &mut self,
        metrics: Arc<super::super::RelationalBranchBasisRegistryMetrics>,
    ) {
        self.basis_registry.bind_metrics(metrics);
    }

    #[cfg(test)]
    pub(crate) fn clear_root_for_test(&mut self) {
        self.root = None;
    }
}
