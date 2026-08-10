use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use sha2::{Digest, Sha256};

use super::workspace_source_inventory;

const LEDGER: &str = "_docs/worth-ui/milestone-3.14.1-proof-ledger.csv";
const EVIDENCE_ROOT: &str = "_docs/worth-ui/milestone-3.14.1-evidence/";

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

pub(crate) fn calculate_source_state(revision: &str) -> Result<String, String> {
    static SOURCE_STATE: OnceLock<(String, Result<String, String>)> = OnceLock::new();
    let (cached_revision, cached_result) = SOURCE_STATE.get_or_init(|| {
        (
            revision.to_owned(),
            calculate_source_state_uncached(revision),
        )
    });
    if cached_revision != revision {
        return Err("source revision changed during ledger validation".to_owned());
    }
    cached_result.clone()
}

fn calculate_source_state_uncached(revision: &str) -> Result<String, String> {
    let mut digest = Sha256::new();
    digest.update(revision.as_bytes());
    digest.update(b"\0repository-content-v2\0");
    let mut inventory_arguments = vec![
        "ls-files",
        "--cached",
        "--others",
        "--exclude-standard",
        "-z",
        "--",
    ];
    let source_paths = source_state_paths()?;
    inventory_arguments.extend(source_paths.iter().map(String::as_str));
    let inventory = git_output(&inventory_arguments)?;
    let mut identities = inventory
        .split(|byte| *byte == 0)
        .filter(|identity| !identity.is_empty())
        .collect::<Vec<_>>();
    identities.sort_unstable();
    identities.dedup();
    for encoded_identity in identities {
        let identity = std::str::from_utf8(encoded_identity)
            .map_err(|error| format!("untracked identity is not UTF-8: {error}"))?;
        if identity == LEDGER || identity.starts_with(EVIDENCE_ROOT) {
            continue;
        }
        let path = repository_root().join(identity);
        let bytes = path
            .is_file()
            .then(|| std::fs::read(&path))
            .transpose()
            .map_err(|error| format!("cannot digest {identity}: {error}"))?;
        update_source_state_entry(&mut digest, encoded_identity, bytes.as_deref());
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn update_source_state_entry(digest: &mut Sha256, identity: &[u8], bytes: Option<&[u8]>) {
    digest.update(if bytes.is_some() {
        b"\0file\0".as_slice()
    } else {
        b"\0missing\0".as_slice()
    });
    digest.update(identity);
    digest.update([0]);
    if let Some(bytes) = bytes {
        digest.update(bytes);
    }
}

const GOVERNANCE_PATHS: &[&str] = &[
    "workspaces/worth-ui",
    "_docs/worth-ui/milestone-3.14.1.md",
    "_docs/worth-ui/native-host-platform.md",
    "_docs/worth-ui/worth_ui_roadmap.md",
    "scripts/ci/run_worth_ui_ledger_test.py",
    "scripts/ci/run_worth_ui_shared_ledger_control.py",
    "scripts/ci/close_worth_ui_3141_ledger.py",
    "scripts/ci/verify_worth_ui_3141_ledger.py",
    "scripts/ci/worth_ui_3141_ledger_contracts.py",
    "scripts/ci/worth_ui_3141_p1_proofs.py",
    "scripts/ci/worth_ui_3141_p2_proofs.py",
    "scripts/ci/worth_ui_ledger_source_state.py",
    "scripts/ci/run_worth_ui_compile_contracts.py",
    "scripts/ci/test_worth_ui_ledger_races.py",
    "tools/boundary-check/config/road1.toml",
];

#[test]
fn governed_source_state_matches_every_transitive_local_package_root() {
    let governed = source_state_paths().unwrap();
    let local = local_package_paths().unwrap();
    assert!(!local.is_empty());
    for package in local {
        assert!(governed.contains(&package), "missing local root {package}");
    }
    assert!(governed.iter().any(|path| path.contains("worth-query")));
    assert!(governed.iter().any(|path| path.contains("worth-signal")));
    assert!(governed
        .iter()
        .any(|path| path == "scripts/ci/verify_worth_ui_3141_ledger.py"));
}

#[test]
fn query_dependency_byte_mutation_changes_the_governed_state_digest() {
    let identity = b"workspaces/worth-query/crates/worth-query/src/lib.rs";
    let mut original = Sha256::new();
    let mut mutant = Sha256::new();
    update_source_state_entry(&mut original, identity, Some(b"query-v1"));
    update_source_state_entry(&mut mutant, identity, Some(b"query-v2"));
    assert_ne!(original.finalize(), mutant.finalize());
}

fn source_state_paths() -> Result<Vec<String>, String> {
    let mut paths = GOVERNANCE_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    paths.extend(local_package_paths()?);
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn local_package_paths() -> Result<Vec<String>, String> {
    let output = Command::new("cargo")
        .args([
            "metadata",
            "--manifest-path",
            "workspaces/worth-ui/Cargo.toml",
            "--format-version",
            "1",
            "--locked",
        ])
        .current_dir(repository_root())
        .output()
        .map_err(|error| format!("cannot inventory local Cargo packages: {error}"))?;
    if !output.status.success() {
        return Err("cargo metadata failed while inventorying local source roots".to_owned());
    }
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("cargo metadata is invalid: {error}"))?;
    let packages = metadata["packages"]
        .as_array()
        .ok_or_else(|| "cargo metadata omits packages".to_owned())?;
    let mut paths = Vec::new();
    for package in packages {
        if !package["source"].is_null() {
            continue;
        }
        let manifest = package["manifest_path"]
            .as_str()
            .ok_or_else(|| "local package omits manifest path".to_owned())?;
        add_local_package_paths(&mut paths, Path::new(manifest))?;
    }
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn add_local_package_paths(paths: &mut Vec<String>, manifest: &Path) -> Result<(), String> {
    let root = repository_root()
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let mut current = manifest
        .parent()
        .ok_or_else(|| "local package manifest has no parent".to_owned())?
        .canonicalize()
        .map_err(|error| error.to_string())?;
    let package = current
        .strip_prefix(&root)
        .map_err(|_| format!("local package escapes repository: {}", manifest.display()))?;
    paths.push(normalize(package));
    while current != root && current.starts_with(&root) {
        for name in ["Cargo.toml", "Cargo.lock"] {
            let candidate = current.join(name);
            if candidate.is_file() {
                paths.push(normalize(
                    candidate.strip_prefix(&root).expect("root checked"),
                ));
            }
        }
        current = current
            .parent()
            .ok_or_else(|| "local package ancestor escaped repository".to_owned())?
            .to_owned();
    }
    Ok(())
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
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
