use crate::history::data::BranchId;

/// Runtime-affine owner identity for a relational branch.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RelationalBranchIdentity {
    runtime_instance_id: u64,
    branch_id: BranchId,
}

impl RelationalBranchIdentity {
    pub(crate) fn new(runtime_instance_id: u64, branch_id: BranchId) -> Self {
        Self {
            runtime_instance_id,
            branch_id,
        }
    }

    pub const fn runtime_instance_id(&self) -> u64 {
        self.runtime_instance_id
    }

    pub fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) fn rebind(&self, runtime_instance_id: u64) -> Self {
        Self::new(runtime_instance_id, self.branch_id.clone())
    }
}
