use super::{case::DirtyPlanarCleanFailCase, counters::DirtyPlanarCleanFailCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirtyPlanarCleanFailReceipt {
    clean_fail_digest: String,
    workload_identity: String,
    topology_clean_fail_identity: String,
    clean_fail_boundary_identity: String,
    dirty_case: DirtyPlanarCleanFailCase,
    counters: DirtyPlanarCleanFailCounters,
}

impl DirtyPlanarCleanFailReceipt {
    pub(crate) fn new(
        clean_fail_digest: String,
        workload_identity: String,
        topology_clean_fail_identity: String,
        clean_fail_boundary_identity: String,
        dirty_case: DirtyPlanarCleanFailCase,
        counters: DirtyPlanarCleanFailCounters,
    ) -> Self {
        Self {
            clean_fail_digest,
            workload_identity,
            topology_clean_fail_identity,
            clean_fail_boundary_identity,
            dirty_case,
            counters,
        }
    }

    pub fn clean_fail_digest(&self) -> &str {
        &self.clean_fail_digest
    }

    pub fn workload_identity(&self) -> &str {
        &self.workload_identity
    }

    pub fn topology_clean_fail_identity(&self) -> &str {
        &self.topology_clean_fail_identity
    }

    pub fn clean_fail_boundary_identity(&self) -> &str {
        &self.clean_fail_boundary_identity
    }

    pub fn dirty_case(&self) -> DirtyPlanarCleanFailCase {
        self.dirty_case
    }

    pub fn counters(&self) -> DirtyPlanarCleanFailCounters {
        self.counters
    }
}
