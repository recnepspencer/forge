use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use super::{derive_family_at, FAMILIES};
use crate::fresh_process_recovery_boundary_gate::repository_root;

const PRE_C8_REVISION: &str = "edd24e18c7221b3305bd9d6aa907e5b31e6ad5a8";
const SNAPSHOT_ROOTS: &[&str] = &[
    "workspaces/worth-store/crates/worth-store-physical-backend/src",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime",
    "workspaces/worth-store/crates/worth-store-physical-format/src",
    "workspaces/worth-store/crates/worth-store-wal/src",
];

static NEXT_SNAPSHOT: AtomicU64 = AtomicU64::new(0);

pub(super) fn pre_c8_surfaces() -> Result<BTreeSet<(String, String)>, String> {
    let snapshot = HistoricalSnapshot::materialize()?;
    let mut surfaces = BTreeSet::new();
    for family in &FAMILIES[1..] {
        if snapshot.root.join(family.facade).is_file() {
            surfaces.extend(derive_family_at(&snapshot.root, family)?);
        }
    }
    Ok(surfaces)
}

struct HistoricalSnapshot {
    root: PathBuf,
}

impl HistoricalSnapshot {
    fn materialize() -> Result<Self, String> {
        let sequence = NEXT_SNAPSHOT.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "worth-c8-pre-api-{}-{sequence}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root)
            .map_err(|error| format!("cannot create {}: {error}", root.display()))?;
        extract_revision(&root)?;
        Ok(Self { root })
    }
}

impl Drop for HistoricalSnapshot {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.root);
    }
}

fn extract_revision(destination: &PathBuf) -> Result<(), String> {
    let mut archive = Command::new("git")
        .arg("archive")
        .arg("--format=tar")
        .arg(PRE_C8_REVISION)
        .args(SNAPSHOT_ROOTS)
        .current_dir(repository_root())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot archive pre-C.8 revision: {error}"))?;
    let stream = archive
        .stdout
        .take()
        .ok_or_else(|| "pre-C.8 archive stdout was not piped".to_owned())?;
    let extraction = Command::new("tar")
        .args(["-xf", "-", "-C"])
        .arg(destination)
        .stdin(Stdio::from(stream))
        .status()
        .map_err(|error| format!("cannot extract pre-C.8 archive: {error}"))?;
    let archived = archive
        .wait()
        .map_err(|error| format!("cannot await pre-C.8 archive: {error}"))?;
    if !archived.success() || !extraction.success() {
        return Err(format!(
            "pre-C.8 archive or extraction failed: git={archived} tar={extraction}"
        ));
    }
    Ok(())
}
