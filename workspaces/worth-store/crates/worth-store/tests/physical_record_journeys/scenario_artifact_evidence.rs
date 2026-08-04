use std::path::Path;

use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub(super) fn artifact_manifest(root: &Path) -> Value {
    let mut pending = vec![root.to_path_buf()];
    let mut artifacts = Vec::new();
    while let Some(directory) = pending.pop() {
        let mut entries = std::fs::read_dir(&directory)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                pending.push(path);
            } else {
                artifacts.push(path);
            }
        }
    }
    artifacts.sort();
    let mut tree = Sha256::new();
    let mut total_bytes = 0_u64;
    for path in &artifacts {
        let relative = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = std::fs::read(path).unwrap();
        let file_digest = Sha256::digest(&bytes);
        total_bytes = total_bytes.saturating_add(bytes.len() as u64);
        tree.update((relative.len() as u64).to_le_bytes());
        tree.update(relative.as_bytes());
        tree.update((bytes.len() as u64).to_le_bytes());
        tree.update(file_digest);
    }
    json!({
        "artifacts": artifacts.len(),
        "bytes": total_bytes,
        "sha256": hex(&tree.finalize()),
    })
}

pub(super) fn source_identity() -> String {
    let mut digest = Sha256::new();
    digest.update(include_bytes!("scenario_evidence.rs"));
    digest.update(include_bytes!("scenario_process_evidence.rs"));
    digest.update(include_bytes!("scenario_artifact_evidence.rs"));
    digest.update(include_bytes!("../c5/courtrooms.rs"));
    digest.update(include_bytes!(
        "durability_admission/wal_group_continuation.rs"
    ));
    digest.update(include_bytes!(
        "durability_admission/data_durability/fault_matrix.rs"
    ));
    digest.update(include_bytes!(
        "durability_admission/data_durability/root_projection_carriage.rs"
    ));
    digest.update(include_bytes!("manifest_scale.rs"));
    hex(&digest.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
