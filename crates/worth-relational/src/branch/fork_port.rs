use crate::runtime::{RelationalForkOwnerBinding, RelationalRuntimeOwnerBinding};

/// Cloneable branch-fork service bound to one live relational runtime owner.
#[derive(Debug, Clone)]
pub struct RelationalForkPort {
    pub(super) runtime_instance_id: u64,
    pub(super) lifecycle: RelationalRuntimeOwnerBinding,
    pub(super) owner: RelationalForkOwnerBinding,
}

impl RelationalForkPort {
    pub(crate) fn new(
        runtime_instance_id: u64,
        lifecycle: RelationalRuntimeOwnerBinding,
        owner: RelationalForkOwnerBinding,
    ) -> Self {
        Self {
            runtime_instance_id,
            lifecycle,
            owner,
        }
    }
}
