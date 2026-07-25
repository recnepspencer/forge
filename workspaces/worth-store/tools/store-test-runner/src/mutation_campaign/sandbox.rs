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
        exclude_orchestrator_member(&workspace)?;
        copy_sibling_workspaces(
            workspace_root
                .parent()
                .ok_or_else(|| "Worth Store workspace has no workspace parent".to_owned())?,
            &root.join("workspaces"),
            workspace_root
                .file_name()
                .ok_or_else(|| "Worth Store workspace has no directory name".to_owned())?,
        )?;
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

fn copy_sibling_workspaces(
    source: &Path,
    destination: &Path,
    current: &std::ffi::OsStr,
) -> Result<(), String> {
    for entry in std::fs::read_dir(source).map_err(|error| {
        format!(
            "cannot list sibling workspaces {}: {error}",
            source.display()
        )
    })? {
        let entry = entry.map_err(|error| format!("cannot read sibling workspace: {error}"))?;
        let file_type = entry.file_type().map_err(|error| {
            format!(
                "cannot inspect sibling workspace {}: {error}",
                entry.path().display()
            )
        })?;
        if file_type.is_dir() && entry.file_name() != current {
            copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn exclude_orchestrator_member(workspace: &Path) -> Result<(), String> {
    let manifest = workspace.join("Cargo.toml");
    let source = std::fs::read_to_string(&manifest).map_err(|error| {
        format!(
            "cannot read sandbox manifest {}: {error}",
            manifest.display()
        )
    })?;
    let narrowed = without_orchestrator_member(&source)?;
    std::fs::write(&manifest, narrowed).map_err(|error| {
        format!(
            "cannot narrow sandbox manifest {}: {error}",
            manifest.display()
        )
    })
}

fn without_orchestrator_member(manifest: &str) -> Result<String, String> {
    let newline = if manifest.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    };
    let member = format!("    \"tools/store-test-runner\",{newline}");
    if manifest.matches(&member).count() != 1 {
        return Err(
            "Worth Store workspace must contain exactly one store-test-runner member".into(),
        );
    }
    Ok(manifest.replacen(&member, "", 1))
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

#[cfg(test)]
mod tests {
    use super::without_orchestrator_member;

    #[test]
    fn sandbox_workspace_excludes_only_the_orchestration_runner() {
        let manifest = "[workspace]\nmembers = [\n    \"crates/worth-store\",\n    \
                        \"tools/store-test-runner\",\n]\n";
        let narrowed = without_orchestrator_member(manifest).unwrap();
        assert!(narrowed.contains("\"crates/worth-store\""));
        assert!(!narrowed.contains("store-test-runner"));

        let crlf = manifest.replace('\n', "\r\n");
        let narrowed = without_orchestrator_member(&crlf).unwrap();
        assert!(narrowed.contains("\r\n"));
        assert!(!narrowed.contains("store-test-runner"));
    }

    #[test]
    fn sandbox_narrowing_denies_manifest_drift() {
        assert!(without_orchestrator_member("[workspace]\nmembers = []\n").is_err());
    }
}
