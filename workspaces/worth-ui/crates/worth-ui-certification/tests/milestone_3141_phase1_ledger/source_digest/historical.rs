use std::path::{Component, Path};
use std::process::Command;

use super::repository_root;

pub(crate) fn file_at_revision(revision: &str, identity: &str) -> Result<Vec<u8>, String> {
    if !is_repository_identity(identity) {
        return Err(format!("repository identity escapes its root: {identity}"));
    }
    let object = format!("{revision}:{identity}");
    let output = Command::new("git")
        .args(["show", object.as_str()])
        .current_dir(repository_root())
        .output()
        .map_err(|error| format!("cannot read historical source: {error}"))?;
    output
        .status
        .success()
        .then_some(output.stdout)
        .ok_or_else(|| format!("missing historical repository file {identity} at {revision}"))
}

fn is_repository_identity(identity: &str) -> bool {
    let path = Path::new(identity);
    !path.is_absolute()
        && !path.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
}
