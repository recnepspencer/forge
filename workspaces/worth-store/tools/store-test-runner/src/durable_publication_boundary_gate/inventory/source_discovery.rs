use std::collections::BTreeMap;
use std::path::Path;

use crate::workspace_root;

const GATE_ROOT: &str = "tools/store-test-runner/src/durable_publication_boundary_gate";

pub(super) const TRACKED_FAMILIES: &[TrackedFamily] = &[
    TrackedFamily::new(
        "durability-vocabulary",
        &["Durability", "durability", "Durable", "durable"],
    ),
    TrackedFamily::new(
        "wal-topology",
        &[
            "WalAppend",
            "WalLsn",
            "WalFrame",
            "WalSegment",
            "WalReplay",
            "WalRecord",
            "WalTail",
        ],
    ),
    TrackedFamily::new(
        "durability-barrier",
        &[
            "DurabilityBarrier",
            "Fsync",
            "DirectorySync",
            "sync_all",
            "sync_data",
        ],
    ),
    TrackedFamily::new(
        "root-publication-topology",
        &[
            "RootPublication",
            "root_publication",
            "root-publications",
            "CatalogReplacementEligibility",
        ],
    ),
    TrackedFamily::new(
        "acknowledgment-topology",
        &["Acknowledgment", "acknowledgment"],
    ),
    TrackedFamily::new(
        "ordinary-record-mutation-entry",
        &[
            "PhysicalRecordSubmission",
            "record_submission()",
            "prepare_append",
            "append_batch",
        ],
    ),
    TrackedFamily::new("store-durability-runtime", &["StoreDurabilityRuntime"]),
    TrackedFamily::new("wal-direct-execution", &["execute_wal_durability"]),
    TrackedFamily::new(
        "durable-publication-declaration",
        &["DurablePublicationDeclaration"],
    ),
    TrackedFamily::new("wal-acknowledgment", &["DurableAckReceipt"]),
    TrackedFamily::new(
        "acknowledgment-precondition",
        &["AcknowledgmentPrecondition"],
    ),
    TrackedFamily::new(
        "parallel-root-publication",
        &[
            "PhysicalRootPublicationStore",
            "PhysicalRootPublicationRuntime",
            "root-publications.log",
        ],
    ),
    TrackedFamily::new("page-lsn", &["PageLsn"]),
    TrackedFamily::new("wal-commit", &["WalCommit"]),
    TrackedFamily::new("writeback-settlement", &["PhysicalWritebackSettlement"]),
    TrackedFamily::new("published-record-batch", &["PublishedRecordBatch"]),
    TrackedFamily::new("append-batch-api", &["append_batch"]),
    TrackedFamily::new(
        "checkpoint-publication",
        &[
            "CheckpointPublication",
            "PhysicalCheckpoint",
            "CheckpointCutover",
            "RetainedWalTail",
            "WalRetention",
        ],
    ),
    TrackedFamily::new(
        "barrier-receipt",
        &[
            "WalDurabilityBarrierReceipt",
            "StoreDurabilityOrderingBarrierDurable",
            "StoreDurabilityExecutionProof",
        ],
    ),
];

pub(super) struct TrackedFamily {
    pub(super) id: &'static str,
    anchors: &'static [&'static str],
}

impl TrackedFamily {
    const fn new(id: &'static str, anchors: &'static [&'static str]) -> Self {
        Self { id, anchors }
    }

    fn match_count(&self, source: &str) -> usize {
        self.anchors
            .iter()
            .map(|anchor| source.match_indices(anchor).count())
            .sum()
    }
}

pub(super) fn discover_tracked_consumers(
) -> Result<BTreeMap<String, BTreeMap<String, usize>>, String> {
    let root = workspace_root();
    let mut discovered = BTreeMap::new();
    for source_root in ["crates", "tools", "examples"] {
        let source_root = root.join(source_root);
        if source_root.exists() {
            discover_under(&root, &source_root, &mut discovered)?;
        }
    }
    Ok(discovered)
}

fn discover_under(
    workspace: &Path,
    source_root: &Path,
    discovered: &mut BTreeMap<String, BTreeMap<String, usize>>,
) -> Result<(), String> {
    let mut pending = vec![source_root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?;
        for entry in entries {
            let path = entry
                .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                if !is_ignored_directory(&path) {
                    pending.push(path);
                }
                continue;
            }
            if !is_inventory_source(&path) {
                continue;
            }
            let relative = relative_path(workspace, &path)?;
            if relative.starts_with(GATE_ROOT) {
                continue;
            }
            let source = std::fs::read_to_string(&path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
            let families = TRACKED_FAMILIES
                .iter()
                .filter_map(|family| {
                    let count = family.match_count(&source);
                    (count > 0).then(|| (family.id.to_owned(), count))
                })
                .collect::<BTreeMap<_, _>>();
            if !families.is_empty() {
                discovered.insert(relative, families);
            }
        }
    }
    Ok(())
}

fn is_ignored_directory(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some("target" | ".git")
    )
}

fn is_inventory_source(path: &Path) -> bool {
    path.extension().and_then(|value| value.to_str()) == Some("rs")
        || path.file_name().and_then(|value| value.to_str()) == Some("Cargo.toml")
}

fn relative_path(workspace: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(workspace)
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
        .map_err(|_| {
            format!(
                "{} is outside workspace {}",
                path.display(),
                workspace.display()
            )
        })
}

#[cfg(test)]
mod tests {
    use super::TRACKED_FAMILIES;

    #[test]
    fn every_family_has_a_unique_nonempty_identity_and_anchor_set() {
        let identities = TRACKED_FAMILIES
            .iter()
            .map(|family| family.id)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(identities.len(), TRACKED_FAMILIES.len());
        assert!(TRACKED_FAMILIES
            .iter()
            .all(|family| !family.id.is_empty() && !family.anchors.is_empty()));
        for required in [
            "durability-vocabulary",
            "wal-topology",
            "durability-barrier",
            "root-publication-topology",
            "acknowledgment-topology",
            "ordinary-record-mutation-entry",
        ] {
            assert!(
                identities.contains(required),
                "C.7 semantic inventory lost `{required}`"
            );
        }
    }

    #[test]
    fn family_counts_expose_same_file_growth() {
        let family = TRACKED_FAMILIES
            .iter()
            .find(|family| family.id == "wal-direct-execution")
            .unwrap();
        assert_eq!(family.match_count("execute_wal_durability"), 1);
        assert_eq!(
            family.match_count("execute_wal_durability execute_wal_durability"),
            2
        );
    }
}
