use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::RecordArtifactFile;

use crate::physical_runtime::record_serving::residency::artifact_tree::PhysicalRecordArtifactTree;
use crate::physical_runtime::WalDurablePhysicalMutation;

pub(super) fn cleanup_extent_candidate_data(
    media: &QualifiedFilesystemMedia,
    durable: &WalDurablePhysicalMutation,
) -> Option<Vec<RecordArtifactFile>> {
    let mut artifacts = Vec::new();
    for frame in durable.data_frames() {
        let basis = frame.basis();
        let target = basis.target();
        let prior = basis.prior().image();
        if prior.is_materialized() || prior.identity() != target {
            return None;
        }
        let artifact = target.coordinate().artifact();
        if !matches!(artifact, RecordArtifactFile::Extent { .. }) {
            return None;
        }
        if !artifacts.contains(&artifact) {
            artifacts.push(artifact);
        }
    }
    let tree = PhysicalRecordArtifactTree::new(media);
    for artifact in artifacts.iter().copied() {
        match tree.file_exists(artifact) {
            Ok(true) if tree.remove_file_durably(artifact).is_err() => return None,
            Ok(_) => {}
            Err(_) => return None,
        }
    }
    artifacts
        .iter()
        .copied()
        .all(|artifact| matches!(tree.file_exists(artifact), Ok(false)))
        .then_some(artifacts)
}
