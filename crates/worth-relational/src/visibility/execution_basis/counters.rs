#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RelationalExecutionBasisCounters {
    version_availability_check_count: usize,
    snapshot_identity_allocation_count: usize,
    lease_registry_insert_count: usize,
}

impl RelationalExecutionBasisCounters {
    pub(crate) fn checked_version_availability(&mut self) {
        self.version_availability_check_count += 1;
    }

    pub(crate) fn allocated_snapshot_identity(&mut self) {
        self.snapshot_identity_allocation_count += 1;
    }

    pub(crate) fn inserted_lease_registry_entry(&mut self) {
        self.lease_registry_insert_count += 1;
    }

    pub fn version_availability_check_count(&self) -> usize {
        self.version_availability_check_count
    }

    pub fn snapshot_identity_allocation_count(&self) -> usize {
        self.snapshot_identity_allocation_count
    }

    pub fn lease_registry_insert_count(&self) -> usize {
        self.lease_registry_insert_count
    }
}
