use std::path::Path;

use worth_store_process_bundle::{target_parent, FreshProcessCargoTarget};

pub(super) struct FreshnessTarget {
    target: FreshProcessCargoTarget,
}

impl FreshnessTarget {
    pub(super) fn allocate(workspace: &Path) -> Result<Self, String> {
        FreshProcessCargoTarget::allocate(&target_parent(workspace))
            .map(|target| Self { target })
            .map_err(|error| format!("allocate freshness proof Cargo target: {error}"))
    }

    pub(super) fn process_target(&self) -> &FreshProcessCargoTarget {
        &self.target
    }

    pub(super) fn close(self) -> Result<(), String> {
        self.target.close()
    }
}
