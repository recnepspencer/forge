use std::path::{Path, PathBuf};

use super::source_inventory::{self, MutationSourceBinding, MutationSourceInventory};

pub(super) struct MutationWorkspaceSnapshot {
    directory: tempfile::TempDir,
    workspace: PathBuf,
    source: MutationSourceBinding,
}

impl MutationWorkspaceSnapshot {
    pub(super) fn materialize(live_workspace: &Path, parent: &Path) -> Result<Self, String> {
        let inventory = source_inventory::capture(live_workspace)?;
        let directory = tempfile::Builder::new()
            .prefix(".worth-store-mutation-snapshot-")
            .tempdir_in(parent)
            .map_err(|error| format!("cannot allocate mutation source snapshot: {error}"))?;
        if let Err(copy_error) = copy_inventory(&inventory, directory.path()) {
            return Err(match directory.close() {
                Ok(()) => copy_error,
                Err(cleanup) => {
                    format!("{copy_error}; mutation source snapshot cleanup failed: {cleanup}")
                }
            });
        }
        let workspace = directory.path().join(inventory.workspace_relative());
        if !workspace.join("Cargo.toml").is_file() {
            return Err("mutation source snapshot omitted the Store workspace manifest".into());
        }
        Ok(Self {
            directory,
            workspace,
            source: inventory.binding().clone(),
        })
    }

    pub(super) fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub(super) fn source(&self) -> &MutationSourceBinding {
        &self.source
    }

    pub(super) fn close(self) -> Result<(), String> {
        let path = self.directory.path().to_path_buf();
        self.directory
            .close()
            .map_err(|error| format!("cannot close mutation source snapshot: {error}"))?;
        if path.exists() {
            return Err(format!(
                "mutation source snapshot survived explicit close: {}",
                path.display()
            ));
        }
        Ok(())
    }
}

fn copy_inventory(inventory: &MutationSourceInventory, destination: &Path) -> Result<(), String> {
    for source in inventory.sources() {
        let relative = source
            .strip_prefix(inventory.repository())
            .map_err(|_| format!("mutation source escaped repository: {}", source.display()))?;
        let target = destination.join(relative);
        let parent = target.parent().ok_or_else(|| {
            format!(
                "mutation snapshot target has no parent: {}",
                target.display()
            )
        })?;
        std::fs::create_dir_all(parent).map_err(|error| {
            format!(
                "cannot create mutation snapshot directory {}: {error}",
                parent.display()
            )
        })?;
        let bytes = std::fs::read(source).map_err(|error| {
            format!("cannot read mutation source {}: {error}", source.display())
        })?;
        std::fs::write(&target, bytes).map_err(|error| {
            format!(
                "cannot write mutation snapshot source {}: {error}",
                target.display()
            )
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::MutationWorkspaceSnapshot;

    #[test]
    fn current_workspace_snapshots_are_private_distinct_and_close_exactly() {
        let live = crate::workspace_root().canonicalize().unwrap();
        let live_manifest = live.join("Cargo.toml");
        let live_bytes = std::fs::read(&live_manifest).unwrap();
        let parent = tempfile::tempdir().unwrap();
        let first = MutationWorkspaceSnapshot::materialize(&live, parent.path()).unwrap();
        let second = MutationWorkspaceSnapshot::materialize(&live, parent.path()).unwrap();
        if first.workspace().canonicalize().unwrap() == live {
            panic!("MUTANT_PREDICATE:c8-mutation-live-workspace-alias");
        }
        assert_ne!(
            first.workspace(),
            second.workspace(),
            "MUTANT_PREDICATE:c8-mutation-live-workspace-alias"
        );
        assert_eq!(
            std::fs::read(first.workspace().join("Cargo.toml")).unwrap(),
            live_bytes
        );
        std::fs::write(first.workspace().join("Cargo.toml"), b"private mutation").unwrap();
        assert_eq!(std::fs::read(&live_manifest).unwrap(), live_bytes);
        assert_eq!(
            std::fs::read(second.workspace().join("Cargo.toml")).unwrap(),
            live_bytes
        );
        let first_root = first.directory.path().to_path_buf();
        let second_root = second.directory.path().to_path_buf();
        first.close().unwrap();
        second.close().unwrap();
        assert!(!first_root.exists());
        assert!(!second_root.exists());
    }
}
