use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use worth_store_physical_format::RecordArtifactFile;
use worth_store_recovery_physics::WalSegmentArtifactIdentity;
use worth_store_recovery_runtime::{
    RecoveryCleanupDispositionKind, RecoveryCleanupEvidence, RecoveryCleanupTarget,
};

#[path = "artifact_snapshot/selected_records.rs"]
mod selected_records;

pub(super) struct ArtifactSnapshot {
    paths: BTreeSet<PathBuf>,
}

impl ArtifactSnapshot {
    pub(super) fn capture(root: &Path) -> Self {
        let mut paths = BTreeSet::new();
        selected_records::capture(root, &mut paths);
        collect_file(&root.join("families/checkpoint.current"), &mut paths);
        collect_files(&root.join("families/wal"), &mut paths);
        Self { paths }
    }

    pub(super) fn assert_reconciled(&self, root: &Path, evidence: &RecoveryCleanupEvidence) {
        let dispositions = disposition_paths(root, evidence);
        if let Some(path) = missing_preexisting_path(&self.paths, &dispositions) {
            panic!(
                "pre-recovery artifact has no cleanup disposition: {}",
                path.display()
            );
        }
        for (path, kind) in dispositions {
            let exists = path.exists();
            match kind {
                RecoveryCleanupDispositionKind::SafelyRemoved => assert!(
                    !exists,
                    "safely removed artifact still exists: {}",
                    path.display()
                ),
                RecoveryCleanupDispositionKind::Current
                | RecoveryCleanupDispositionKind::Retained
                | RecoveryCleanupDispositionKind::Deferred(_)
                | RecoveryCleanupDispositionKind::QuarantinedOrUnsupported => assert!(
                    exists,
                    "retained cleanup artifact is missing: {}",
                    path.display()
                ),
                RecoveryCleanupDispositionKind::Eligible => {
                    panic!("terminal cleanup evidence retained Eligible")
                }
            }
        }
    }
}

fn disposition_paths(
    root: &Path,
    evidence: &RecoveryCleanupEvidence,
) -> BTreeMap<PathBuf, RecoveryCleanupDispositionKind> {
    let mut paths = BTreeMap::new();
    for disposition in evidence.dispositions() {
        let path = match disposition.target() {
            RecoveryCleanupTarget::Record(artifact) => record_artifact_path(root, *artifact),
            RecoveryCleanupTarget::Wal(artifact) => {
                root.join("families/wal").join(artifact.file_name())
            }
            RecoveryCleanupTarget::Checkpoint(_) => root.join("families/checkpoint.current"),
            RecoveryCleanupTarget::Residue { name, .. } => {
                root.join("families/wal").join(name.as_ref())
            }
        };
        assert!(
            paths.insert(path, disposition.kind()).is_none(),
            "duplicate cleanup disposition path"
        );
    }
    paths
}

fn missing_preexisting_path(
    snapshot: &BTreeSet<PathBuf>,
    dispositions: &BTreeMap<PathBuf, RecoveryCleanupDispositionKind>,
) -> Option<PathBuf> {
    snapshot
        .iter()
        .find(|path| !dispositions.contains_key(*path))
        .cloned()
}

fn collect_files(directory: &Path, paths: &mut BTreeSet<PathBuf>) {
    if !directory.exists() {
        return;
    }
    for entry in std::fs::read_dir(directory).expect("enumerate persisted artifacts") {
        let path = entry.expect("persisted artifact entry").path();
        if path.is_dir() {
            collect_files(&path, paths);
        } else if path.is_file() {
            paths.insert(path);
        }
    }
}

fn collect_file(path: &Path, paths: &mut BTreeSet<PathBuf>) {
    if path.is_file() {
        paths.insert(path.to_path_buf());
    }
}

fn record_artifact_path(root: &Path, artifact: RecordArtifactFile) -> PathBuf {
    let records = root.join("families/records");
    let directory = match artifact {
        RecordArtifactFile::BootstrapCatalog
        | RecordArtifactFile::CurrentRootSelector
        | RecordArtifactFile::PreviousRootSelector => records,
        RecordArtifactFile::RootSelectorCandidate { .. }
        | RecordArtifactFile::CatalogCandidate { .. } => root.join("staging/records"),
        RecordArtifactFile::RootManifest { .. } | RecordArtifactFile::RootRoutingBlock { .. } => {
            records.join("roots")
        }
        RecordArtifactFile::Segment { .. } => records.join("segments"),
        RecordArtifactFile::SegmentManifest { .. }
        | RecordArtifactFile::SegmentMembershipBlock { .. } => records.join("segment-manifests"),
        RecordArtifactFile::Extent { .. } => records.join("extents"),
        RecordArtifactFile::ExtentManifest { .. } => records.join("extent-manifests"),
        RecordArtifactFile::FreeSpaceManifest { .. }
        | RecordArtifactFile::FreeSpaceMembershipBlock { .. } => records.join("free-space"),
    };
    directory.join(artifact.file_name())
}

#[test]
fn an_omitted_preexisting_artifact_cannot_be_hidden_by_the_cleanup_oracle() {
    let path = PathBuf::from("families/wal").join(
        WalSegmentArtifactIdentity::parse("segment-1-generation-1.wal")
            .unwrap()
            .file_name(),
    );
    let snapshot = BTreeSet::from([path.clone()]);
    assert_eq!(
        missing_preexisting_path(&snapshot, &BTreeMap::new()),
        Some(path)
    );
}
