use std::path::{Component, Path};
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::evidence::{sha256_bytes, sha256_file};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RequestedSourceEdit {
    pub source_path: String,
    pub original_sha256: String,
    pub purpose: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservedSourceEditIdentity {
    pub source_path: String,
    pub original_sha256: String,
    pub edited_sha256: String,
    pub purpose: String,
    pub description: String,
}

impl RequestedSourceEdit {
    pub fn new(
        source_path: String,
        original_sha256: String,
        purpose: String,
        description: String,
    ) -> Self {
        Self {
            source_path,
            original_sha256,
            purpose,
            description,
        }
    }
}

pub(crate) fn observe(
    workspace_root: &Path,
    requested: Option<&RequestedSourceEdit>,
) -> Result<Option<ObservedSourceEditIdentity>, String> {
    requested
        .map(|requested| observe_one(workspace_root, requested))
        .transpose()
}

fn observe_one(
    workspace_root: &Path,
    requested: &RequestedSourceEdit,
) -> Result<ObservedSourceEditIdentity, String> {
    let relative = Path::new(&requested.source_path);
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
        || requested.source_path.trim().is_empty()
        || requested.purpose.trim().is_empty()
        || requested.description.trim().is_empty()
        || !is_sha256(&requested.original_sha256)
    {
        return Err("source edit declaration is incomplete or escapes the workspace".to_owned());
    }
    let normalized_source = requested.source_path.replace('\\', "/");
    let workspace = workspace_root.canonicalize().map_err(|error| {
        format!(
            "could not resolve workspace root {}: {error}",
            workspace_root.display()
        )
    })?;
    let path = workspace_root.join(relative);
    let metadata = std::fs::symlink_metadata(&path).map_err(|error| {
        format!(
            "could not inspect edited source {}: {error}",
            path.display()
        )
    })?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!(
            "edited source is not a regular file: {}",
            path.display()
        ));
    }
    let canonical = path.canonicalize().map_err(|error| {
        format!(
            "could not resolve edited source {}: {error}",
            path.display()
        )
    })?;
    if !canonical.starts_with(&workspace) {
        return Err(format!(
            "edited source escaped the workspace: {}",
            path.display()
        ));
    }
    validate_single_tracked_edit(workspace_root, &normalized_source)?;
    let committed_sha256 = committed_worktree_sha256(workspace_root, &normalized_source)?;
    if committed_sha256 != requested.original_sha256 {
        return Err(format!(
            "source edit original SHA-256 does not match HEAD for {normalized_source}"
        ));
    }
    let edited_sha256 = sha256_file(&canonical)?;
    if edited_sha256 == requested.original_sha256 {
        return Err("source edit declaration points at unchanged content".to_owned());
    }
    Ok(ObservedSourceEditIdentity {
        source_path: normalized_source,
        original_sha256: requested.original_sha256.clone(),
        edited_sha256,
        purpose: requested.purpose.clone(),
        description: requested.description.clone(),
    })
}

fn validate_single_tracked_edit(workspace_root: &Path, source_path: &str) -> Result<(), String> {
    let mut changed = nul_paths(&git_output(
        workspace_root,
        &["diff", "--name-only", "--relative", "-z", "HEAD", "--", "."],
    )?)?;
    changed.extend(nul_paths(&git_output(
        workspace_root,
        &[
            "ls-files",
            "--others",
            "--exclude-standard",
            "-z",
            "--",
            ".",
        ],
    )?)?);
    changed.sort();
    changed.dedup();
    if changed != [source_path] {
        return Err(format!(
            "source edit evidence requires exactly one dirty tracked Store source; declared {source_path}, observed {changed:?}"
        ));
    }
    let tracked = Command::new("git")
        .args(["ls-files", "--error-unmatch", "--", source_path])
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not inspect tracked source {source_path}: {error}"))?;
    if !tracked.status.success() {
        return Err(format!(
            "source edit evidence requires a source tracked at HEAD: {source_path}"
        ));
    }
    Ok(())
}

fn committed_worktree_sha256(workspace_root: &Path, source_path: &str) -> Result<String, String> {
    let prefix = String::from_utf8(git_output(workspace_root, &["rev-parse", "--show-prefix"])?)
        .map_err(|error| format!("Git returned a non-UTF-8 workspace prefix: {error}"))?;
    let repository_path = format!("{}{source_path}", prefix.trim());
    let path_option = format!("--path={repository_path}");
    let object = format!("HEAD:{repository_path}");
    let bytes = git_output(
        workspace_root,
        &["cat-file", "--filters", &path_option, &object],
    )?;
    Ok(sha256_bytes(&bytes))
}

fn git_output(workspace_root: &Path, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(workspace_root)
        .output()
        .map_err(|error| format!("could not launch Git source-edit observation: {error}"))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "Git source-edit observation failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

fn nul_paths(bytes: &[u8]) -> Result<Vec<String>, String> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| {
            String::from_utf8(path.to_vec())
                .map(|path| path.replace('\\', "/"))
                .map_err(|error| format!("Git returned a non-UTF-8 source path: {error}"))
        })
        .collect()
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    #[test]
    fn observation_requires_one_tracked_edit_with_a_head_backed_original() {
        let root = scratch_repository();
        let source = root.join("src/lib.rs");
        let other = root.join("src/other.rs");
        let original = sha256_file(&source).unwrap();
        fs::write(&source, b"pub fn value() -> u8 { 2 }\n").unwrap();
        let request = RequestedSourceEdit::new(
            "src/lib.rs".to_owned(),
            original.clone(),
            "private-leaf-owner".to_owned(),
            "change the leaf value".to_owned(),
        );
        let observed = observe(&root, Some(&request)).unwrap().unwrap();
        assert_eq!(observed.original_sha256, original);
        assert_ne!(observed.original_sha256, observed.edited_sha256);

        fs::write(&other, b"pub fn other() -> u8 { 3 }\n").unwrap();
        assert!(observe(&root, Some(&request))
            .unwrap_err()
            .contains("exactly one dirty tracked Store source"));

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn observation_rejects_a_forged_original_hash() {
        let root = scratch_repository();
        let source = root.join("src/lib.rs");
        fs::write(&source, b"pub fn value() -> u8 { 2 }\n").unwrap();
        let request = RequestedSourceEdit::new(
            "src/lib.rs".to_owned(),
            "f".repeat(64),
            "private-leaf-owner".to_owned(),
            "change the leaf value".to_owned(),
        );
        assert!(observe(&root, Some(&request))
            .unwrap_err()
            .contains("does not match HEAD"));
        fs::remove_dir_all(root).unwrap();
    }

    fn scratch_repository() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "worth-store-source-edit-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("src")).unwrap();
        run_git(&root, &["init", "--quiet"]);
        run_git(&root, &["config", "user.email", "c1@example.invalid"]);
        run_git(&root, &["config", "user.name", "C1 Test"]);
        run_git(&root, &["config", "core.autocrlf", "false"]);
        fs::write(root.join("src/lib.rs"), b"pub fn value() -> u8 { 1 }\n").unwrap();
        fs::write(root.join("src/other.rs"), b"pub fn other() -> u8 { 2 }\n").unwrap();
        run_git(&root, &["add", "."]);
        run_git(&root, &["commit", "--quiet", "-m", "baseline"]);
        root
    }

    fn run_git(root: &Path, arguments: &[&str]) {
        let output = Command::new("git")
            .args(arguments)
            .current_dir(root)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {arguments:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
