use std::path::{Component, Path, PathBuf};

pub const DISPOSABLE_ARTIFACT_ROOT_MARKER: &str = ".worth-store-artifact-root-v1";
const DISPOSABLE_MARKER_CONTENT: &str = "worth-store disposable artifact root v1\n";

#[derive(Debug, Clone)]
pub struct AdmittedArtifactRoot {
    workspace_root: PathBuf,
    target_root: PathBuf,
}

impl AdmittedArtifactRoot {
    pub fn admit(workspace_root: &Path, target_root: &Path) -> Result<Self, String> {
        if !target_root.is_absolute() {
            return Err(format!(
                "artifact target root must be an explicit absolute path: {}",
                target_root.display()
            ));
        }
        deny_lexical_escape(target_root)?;
        deny_symlink_path(workspace_root, target_root)?;
        let workspace_root = workspace_root.canonicalize().map_err(|error| {
            format!(
                "could not resolve workspace root {}: {error}",
                workspace_root.display()
            )
        })?;
        let target_root = target_root.canonicalize().map_err(|error| {
            format!(
                "could not resolve artifact target root {}: {error}",
                target_root.display()
            )
        })?;
        if target_root == workspace_root || !target_root.starts_with(&workspace_root) {
            return Err(format!(
                "artifact target root must be a strict workspace descendant: {}",
                target_root.display()
            ));
        }
        let cargo_target = workspace_root.join("target");
        let disposable = valid_disposable_marker(&target_root)?;
        if target_root != cargo_target && !disposable {
            return Err(format!(
                "artifact root is neither the workspace Cargo target nor a marked disposable root: {}",
                target_root.display()
            ));
        }
        Ok(Self {
            workspace_root,
            target_root,
        })
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn target_root(&self) -> &Path {
        &self.target_root
    }
}

pub fn mark_disposable_artifact_root(target_root: &Path) -> Result<(), String> {
    std::fs::create_dir_all(target_root)
        .map_err(|error| format!("could not create {}: {error}", target_root.display()))?;
    std::fs::write(
        target_root.join(DISPOSABLE_ARTIFACT_ROOT_MARKER),
        DISPOSABLE_MARKER_CONTENT,
    )
    .map_err(|error| format!("could not mark {}: {error}", target_root.display()))
}

fn deny_lexical_escape(path: &Path) -> Result<(), String> {
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::CurDir))
    {
        return Err(format!(
            "artifact target root contains path traversal: {}",
            path.display()
        ));
    }
    Ok(())
}

fn deny_symlink_path(workspace_root: &Path, target_root: &Path) -> Result<(), String> {
    let relative = target_root.strip_prefix(workspace_root).map_err(|_| {
        format!(
            "artifact root escaped workspace during symlink admission: {}",
            target_root.display()
        )
    })?;
    let mut cursor = workspace_root.to_path_buf();
    for component in relative.components() {
        cursor.push(component.as_os_str());
        let metadata = std::fs::symlink_metadata(&cursor)
            .map_err(|error| format!("could not inspect {}: {error}", cursor.display()))?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "artifact root admission denies symlink or junction {}",
                cursor.display()
            ));
        }
    }
    Ok(())
}

fn valid_disposable_marker(target_root: &Path) -> Result<bool, String> {
    let marker = target_root.join(DISPOSABLE_ARTIFACT_ROOT_MARKER);
    if !marker.exists() {
        return Ok(false);
    }
    let metadata = std::fs::symlink_metadata(&marker)
        .map_err(|error| format!("could not inspect {}: {error}", marker.display()))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!(
            "disposable artifact marker is not a regular file: {}",
            marker.display()
        ));
    }
    let contents = std::fs::read_to_string(&marker)
        .map_err(|error| format!("could not read {}: {error}", marker.display()))?;
    Ok(contents == DISPOSABLE_MARKER_CONTENT)
}
