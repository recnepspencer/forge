use std::collections::BTreeMap;
use std::path::{Component, Path};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

use super::repository_root;

pub(crate) fn file_at_revision(revision: &str, identity: &str) -> Result<Vec<u8>, String> {
    if !is_repository_identity(identity) {
        return Err(format!("repository identity escapes its root: {identity}"));
    }
    static FILES: OnceLock<Mutex<BTreeMap<(String, String), Result<Vec<u8>, String>>>> =
        OnceLock::new();
    let files = FILES.get_or_init(|| Mutex::new(BTreeMap::new()));
    let key = (revision.to_owned(), identity.to_owned());
    if let Some(result) = files
        .lock()
        .map_err(|_| "historical source cache is poisoned".to_owned())?
        .get(&key)
        .cloned()
    {
        return result;
    }
    let object = format!("{revision}:{identity}");
    let output = Command::new("git")
        .args(["show", object.as_str()])
        .current_dir(repository_root())
        .output()
        .map_err(|error| format!("cannot read historical source: {error}"))?;
    let result = output
        .status
        .success()
        .then_some(output.stdout)
        .ok_or_else(|| format!("missing historical repository file {identity} at {revision}"));
    files
        .lock()
        .map_err(|_| "historical source cache is poisoned".to_owned())?
        .insert(key, result.clone());
    result
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
