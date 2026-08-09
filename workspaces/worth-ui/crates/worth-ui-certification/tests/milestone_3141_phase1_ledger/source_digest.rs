use std::path::{Component, Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use super::workspace_source_inventory;

const LEDGER: &str = "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv";

pub(super) fn repository_root() -> PathBuf {
    workspace_source_inventory()
        .root()
        .parent()
        .and_then(Path::parent)
        .expect("repository root")
        .to_owned()
}

pub(super) fn repository_file(identity: &str) -> Result<PathBuf, String> {
    let relative = Path::new(identity);
    if relative.is_absolute()
        || relative.components().any(|part| {
            matches!(
                part,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(format!("repository identity escapes its root: {identity}"));
    }
    let path = repository_root().join(relative);
    path.is_file()
        .then_some(path)
        .ok_or_else(|| format!("missing repository file {identity}"))
}

pub(super) fn calculate(source_identity: &str) -> Result<String, String> {
    let mut sources = source_identity.split(';').collect::<Vec<_>>();
    if sources.iter().any(|source| source.is_empty()) {
        return Err("source identity contains an empty path".to_owned());
    }
    sources.sort_unstable();
    if sources.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("source identity contains duplicates".to_owned());
    }
    let mut digest = Sha256::new();
    for source in sources {
        digest.update(source.as_bytes());
        digest.update([0]);
        digest.update(
            std::fs::read(repository_file(source)?)
                .map_err(|error| format!("cannot digest {source}: {error}"))?,
        );
        digest.update([0]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub(super) fn calculate_source_state(revision: &str) -> Result<String, String> {
    let mut digest = Sha256::new();
    digest.update(revision.as_bytes());
    digest.update(b"\0tracked-diff\0");
    digest.update(git_output(&[
        "diff",
        "--binary",
        "--no-ext-diff",
        "HEAD",
        "--",
        ".",
        ":(exclude)_docs/worth-ui/milestone-3.14.1-proof-ledger.csv",
    ])?);
    let untracked = git_output(&["ls-files", "--others", "--exclude-standard", "-z"])?;
    let mut identities = untracked
        .split(|byte| *byte == 0)
        .filter(|identity| !identity.is_empty())
        .collect::<Vec<_>>();
    identities.sort_unstable();
    for encoded_identity in identities {
        let identity = std::str::from_utf8(encoded_identity)
            .map_err(|error| format!("untracked identity is not UTF-8: {error}"))?;
        if identity == LEDGER {
            continue;
        }
        digest.update(b"\0untracked\0");
        digest.update(encoded_identity);
        digest.update([0]);
        digest.update(
            std::fs::read(repository_file(identity)?)
                .map_err(|error| format!("cannot digest {identity}: {error}"))?,
        );
    }
    Ok(format!("{:x}", digest.finalize()))
}

pub(super) fn file_digest(identity: &str) -> Result<String, String> {
    let bytes = std::fs::read(repository_file(identity)?)
        .map_err(|error| format!("cannot digest {identity}: {error}"))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

fn git_output(arguments: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(repository_root())
        .output()
        .map_err(|error| format!("cannot run git: {error}"))?;
    output
        .status
        .success()
        .then_some(output.stdout)
        .ok_or_else(|| format!("git {} failed", arguments.join(" ")))
}
