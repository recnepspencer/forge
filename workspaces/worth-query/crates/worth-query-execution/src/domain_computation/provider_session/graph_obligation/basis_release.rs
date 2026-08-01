pub(in crate::domain_computation) trait WorthQueryGraphWorkBasisRelease {
    fn release_graph_work_basis(self) -> bool;
}

pub(in crate::domain_computation) trait WorthQueryManagedGraphWorkBasisHandoff:
    WorthQueryGraphWorkBasisRelease
{
    fn take_managed_relational_basis(
        &mut self,
    ) -> Option<worth_relational::facade::runtime::RelationalExecutionBasisLease>;

    fn restore_managed_relational_basis(
        &mut self,
        basis: worth_relational::facade::runtime::RelationalExecutionBasisLease,
    ) -> Result<(), worth_relational::facade::runtime::RelationalExecutionBasisLease>;
}

impl WorthQueryGraphWorkBasisRelease
    for crate::domain_computation::primary_graph::WorthQueryApplicationBasisLease
{
    fn release_graph_work_basis(self) -> bool {
        self.release().released()
    }
}

impl WorthQueryManagedGraphWorkBasisHandoff
    for crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease
{
    fn take_managed_relational_basis(
        &mut self,
    ) -> Option<worth_relational::facade::runtime::RelationalExecutionBasisLease> {
        self.take_relational_basis()
    }

    fn restore_managed_relational_basis(
        &mut self,
        basis: worth_relational::facade::runtime::RelationalExecutionBasisLease,
    ) -> Result<(), worth_relational::facade::runtime::RelationalExecutionBasisLease> {
        self.restore_relational_basis(basis)
    }
}

impl WorthQueryGraphWorkBasisRelease
    for crate::domain_computation::primary_graph::WorthQueryApplicationSnapshotLease
{
    fn release_graph_work_basis(self) -> bool {
        self.release()
    }
}
