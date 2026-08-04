use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use super::inventory::{
    WalArtifactInventory, WalArtifactInventoryIdentity, WalArtifactInventoryScan,
    WalArtifactObservation, WalArtifactScanCounters,
};
use super::prefix_scan::{scan_segment_reader, WalFrameObservation, WAL_SCAN_BUFFER_BYTES};
use super::WalArtifactStoreDenial;

pub(super) fn scan(
    inventory: &WalArtifactInventory,
) -> Result<WalArtifactInventoryScan, WalArtifactStoreDenial> {
    let mut artifacts = Vec::new();
    let mut counters = WalArtifactScanCounters::default();
    scan_wal_segment(inventory, &mut artifacts, &mut counters)?;
    scan_checkpoint_artifacts(inventory, &mut artifacts, &mut counters)?;
    artifacts.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.offset.cmp(&right.offset))
    });
    Ok(WalArtifactInventoryScan {
        identity: inventory.identity.clone(),
        artifacts,
        counters,
    })
}

fn scan_wal_segment(
    inventory: &WalArtifactInventory,
    artifacts: &mut Vec<WalArtifactObservation>,
    counters: &mut WalArtifactScanCounters,
) -> Result<(), WalArtifactStoreDenial> {
    let path = segment_path(&inventory.identity);
    let mut file = match std::fs::File::open(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(_) => return Err(WalArtifactStoreDenial::Io),
    };
    let mut buffer = vec![0; WAL_SCAN_BUFFER_BYTES];
    let artifact_start = artifacts.len();
    let summary = scan_segment_reader(
        &mut file,
        0,
        None,
        inventory.identity.segment_id,
        inventory.identity.generation,
        &mut buffer,
        |frame: WalFrameObservation| {
            artifacts.push(WalArtifactObservation {
                path: path.clone(),
                offset: frame.payload_offset,
                byte_count: frame.payload_bytes,
                digest: frame.payload_digest,
            });
        },
    )?;
    if summary.observed_file_bytes > 0 {
        counters.directories_examined = counters.directories_examined.saturating_add(1);
        counters.artifacts_read = counters
            .artifacts_read
            .saturating_add((artifacts.len() - artifact_start) as u64);
        counters.bytes_read = counters.bytes_read.saturating_add(summary.bytes_scanned);
    }
    Ok(())
}

fn scan_checkpoint_artifacts(
    inventory: &WalArtifactInventory,
    artifacts: &mut Vec<WalArtifactObservation>,
    counters: &mut WalArtifactScanCounters,
) -> Result<(), WalArtifactStoreDenial> {
    for entry in
        std::fs::read_dir(&inventory.identity.root).map_err(|_| WalArtifactStoreDenial::Io)?
    {
        let entry = entry.map_err(|_| WalArtifactStoreDenial::Io)?;
        if !entry
            .file_type()
            .map_err(|_| WalArtifactStoreDenial::Io)?
            .is_dir()
            || !entry
                .file_name()
                .to_string_lossy()
                .starts_with("worth-store-durability-")
        {
            continue;
        }
        counters.directories_examined = counters.directories_examined.saturating_add(1);
        for candidate in std::fs::read_dir(entry.path()).map_err(|_| WalArtifactStoreDenial::Io)? {
            let candidate = candidate.map_err(|_| WalArtifactStoreDenial::Io)?;
            let path =
                std::fs::canonicalize(candidate.path()).map_err(|_| WalArtifactStoreDenial::Io)?;
            if !candidate
                .file_type()
                .map_err(|_| WalArtifactStoreDenial::Io)?
                .is_file()
                || !is_checkpoint_artifact(&inventory.identity.root, &path)
            {
                continue;
            }
            let (byte_count, digest) = digest_file(&path)?;
            counters.artifacts_read = counters.artifacts_read.saturating_add(1);
            counters.bytes_read = counters.bytes_read.saturating_add(byte_count);
            artifacts.push(WalArtifactObservation {
                path,
                offset: 0,
                byte_count,
                digest,
            });
        }
    }
    Ok(())
}

fn digest_file(path: &Path) -> Result<(u64, [u8; 32]), WalArtifactStoreDenial> {
    let mut file = std::fs::File::open(path).map_err(|_| WalArtifactStoreDenial::Io)?;
    let expected = file
        .metadata()
        .map_err(|_| WalArtifactStoreDenial::Io)?
        .len();
    let mut buffer = vec![0; WAL_SCAN_BUFFER_BYTES];
    let mut digest = Sha256::new();
    let mut bytes_read = 0u64;
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| WalArtifactStoreDenial::Io)?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
        bytes_read = bytes_read.saturating_add(read as u64);
    }
    if bytes_read != expected {
        return Err(WalArtifactStoreDenial::Io);
    }
    Ok((bytes_read, digest.finalize().into()))
}

pub(super) fn is_inventory_artifact(
    identity: &WalArtifactInventoryIdentity,
    artifact: &Path,
) -> bool {
    artifact == segment_path(identity) || is_checkpoint_artifact(&identity.root, artifact)
}

fn segment_path(identity: &WalArtifactInventoryIdentity) -> std::path::PathBuf {
    identity.root.join("wal").join(format!(
        "segment-{}-generation-{}.wal",
        identity.segment_id, identity.generation
    ))
}

fn is_checkpoint_artifact(root: &Path, artifact: &Path) -> bool {
    let Some(directory) = artifact.parent() else {
        return false;
    };
    directory.parent() == Some(root)
        && directory.file_name().is_some_and(|name| {
            name.to_string_lossy()
                .starts_with("worth-store-durability-")
        })
        && matches!(
            artifact.file_name().and_then(|name| name.to_str()),
            Some("staged" | "published")
        )
}
