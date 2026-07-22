use std::path::{Path, PathBuf};

pub(super) struct MutationSandbox {
    root: PathBuf,
    workspace: PathBuf,
    target: PathBuf,
}

impl MutationSandbox {
    pub(super) fn create(workspace_root: &Path) -> Result<Self, String> {
        let forge_root = workspace_root
            .parent()
            .and_then(Path::parent)
            .ok_or_else(|| "worth-store workspace must live under forge/workspaces".to_owned())?;
        let root = std::env::temp_dir().join("worth-store-c5-mutants");
        if root.exists() {
            verified_cleanup(&root)?;
        }
        let workspace = root.join("workspaces/worth-store");
        copy_tree(workspace_root, &workspace)?;
        copy_tree(&forge_root.join("crates"), &root.join("crates"))?;
        std::fs::copy(forge_root.join("Cargo.toml"), root.join("Cargo.toml"))
            .map_err(|error| format!("cannot copy forge workspace manifest: {error}"))?;
        Ok(Self {
            root,
            workspace,
            target: workspace_root.join("target"),
        })
    }

    pub(super) fn workspace(&self) -> &Path {
        &self.workspace
    }

    pub(super) fn target(&self) -> &Path {
        &self.target
    }
}

impl Drop for MutationSandbox {
    fn drop(&mut self) {
        let temporary_root = std::env::temp_dir();
        if self.root.starts_with(&temporary_root) {
            let _ = verified_cleanup(&self.root);
        }
    }
}

fn verified_cleanup(root: &Path) -> Result<(), String> {
    let temporary_root = std::env::temp_dir();
    if !root.starts_with(&temporary_root)
        || root.file_name().and_then(|name| name.to_str()) != Some("worth-store-c5-mutants")
    {
        return Err(format!(
            "refusing to remove unrecognized mutation sandbox {}",
            root.display()
        ));
    }
    std::fs::remove_dir_all(root)
        .map_err(|error| format!("cannot clean mutation sandbox {}: {error}", root.display()))
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    std::fs::create_dir_all(destination)
        .map_err(|error| format!("cannot create {}: {error}", destination.display()))?;
    for entry in std::fs::read_dir(source)
        .map_err(|error| format!("cannot list {}: {error}", source.display()))?
    {
        let entry = entry.map_err(|error| format!("cannot read directory entry: {error}"))?;
        if entry.file_name() == "target" || entry.file_name() == ".git" {
            continue;
        }
        let target = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .map_err(|error| format!("cannot inspect {}: {error}", entry.path().display()))?;
        if file_type.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else if file_type.is_file() {
            std::fs::copy(entry.path(), &target)
                .map_err(|error| format!("cannot copy {}: {error}", entry.path().display()))?;
        } else {
            return Err(format!(
                "mutation sandbox refuses non-file entry {}",
                entry.path().display()
            ));
        }
    }
    Ok(())
}
