use std::path::Path;

use worth_store_process_bundle::{target_parent, FreshProcessCargoTarget};

pub(super) struct MutationCampaignTarget {
    target: FreshProcessCargoTarget,
}

impl MutationCampaignTarget {
    pub(super) fn allocate(workspace: &Path) -> Result<Self, String> {
        FreshProcessCargoTarget::allocate(&target_parent(workspace))
            .map(|target| Self { target })
            .map_err(|error| format!("allocate mutation campaign target: {error}"))
    }

    pub(super) fn path(&self) -> &Path {
        self.target.path()
    }

    #[cfg(test)]
    pub(crate) fn allocate_at(parent: &Path) -> Result<Self, String> {
        FreshProcessCargoTarget::allocate(parent)
            .map(|target| Self { target })
            .map_err(|error| format!("allocate mutation campaign target: {error}"))
    }

    pub(super) fn close(self) -> Result<(), String> {
        self.target.close()
    }
}

#[cfg(test)]
mod tests {
    use super::MutationCampaignTarget;

    #[test]
    fn campaign_target_is_an_exclusive_child_and_closes() {
        let workspace = std::env::temp_dir().join(format!(
            "worth-store-mutation-target-test-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&workspace);
        std::fs::create_dir(&workspace).unwrap();
        let target = MutationCampaignTarget::allocate_at(&workspace.join("target")).unwrap();
        assert!(target.path().starts_with(workspace.join("target")));
        assert_ne!(target.path(), workspace.join("target").as_path());
        let path = target.path().to_owned();
        target.close().unwrap();
        assert!(!path.exists());
        std::fs::remove_dir_all(workspace).unwrap();
    }
}
