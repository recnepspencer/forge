use std::path::Path;

use std::io::Read;

use sha2::{Digest, Sha256};

use super::frame_codec::parse_segment_filename;
use super::prefix_scan::{scan_segment_path_observing, WalFrameObservation, WAL_SCAN_BUFFER_BYTES};
use super::{
    AdmittedWalArtifactStore, WalArtifactScanCounters, WalArtifactStoreDenial,
    WalPersistedArtifact, WalPersistedArtifactSet,
};

pub(super) fn scan(
    store: &AdmittedWalArtifactStore,
) -> Result<WalPersistedArtifactSet, WalArtifactStoreDenial> {
    let mut artifacts = Vec::new();
    let mut counters = WalArtifactScanCounters::default();
    scan_wal_segment(store, &mut artifacts, &mut counters)?;
    scan_checkpoint_artifacts(store, &mut artifacts, &mut counters)?;
    artifacts.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then(left.offset.cmp(&right.offset))
    });
    Ok(WalPersistedArtifactSet {
        store: store.identity.clone(),
        artifacts,
        counters,
    })
}

fn scan_wal_segment(
    store: &AdmittedWalArtifactStore,
    artifacts: &mut Vec<WalPersistedArtifact>,
    counters: &mut WalArtifactScanCounters,
) -> Result<(), WalArtifactStoreDenial> {
    let path = store.identity.root.join("wal").join(format!(
        "segment-{}-generation-{}.wal",
        store.identity.segment_id, store.identity.generation
    ));
    let mut buffer = vec![0; WAL_SCAN_BUFFER_BYTES];
    let artifact_start = artifacts.len();
    let summary = scan_segment_path_observing(
        &store.identity.root,
        store.identity.segment_id,
        store.identity.generation,
        &mut buffer,
        |frame: WalFrameObservation| {
            artifacts.push(WalPersistedArtifact {
                path: path.clone(),
                offset: frame.payload_offset,
                byte_count: frame.payload_bytes,
                digest: frame.payload_digest,
            });
        },
    )?;
    if summary.observed_file_bytes > 0 {
        counters.directories_examined = counters.directories_examined.saturating_add(1);
        let frame_count = (artifacts.len() - artifact_start) as u64;
        counters.artifacts_read = counters.artifacts_read.saturating_add(frame_count);
        counters.bytes_read = counters.bytes_read.saturating_add(summary.bytes_scanned);
    }
    Ok(())
}

fn scan_checkpoint_artifacts(
    store: &AdmittedWalArtifactStore,
    artifacts: &mut Vec<WalPersistedArtifact>,
    counters: &mut WalArtifactScanCounters,
) -> Result<(), WalArtifactStoreDenial> {
    for entry in std::fs::read_dir(&store.identity.root).map_err(|_| WalArtifactStoreDenial::Io)? {
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
                || !is_checkpoint_artifact(&store.identity.root, &path)
            {
                continue;
            }
            let (byte_count, digest) = digest_file(&path)?;
            counters.artifacts_read = counters.artifacts_read.saturating_add(1);
            counters.bytes_read = counters.bytes_read.saturating_add(byte_count);
            artifacts.push(WalPersistedArtifact {
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

pub(super) fn is_segment_artifact(root: &Path, artifact: &Path) -> bool {
    let Some(wal_directory) = artifact.parent() else {
        return false;
    };
    wal_directory.parent() == Some(root)
        && wal_directory.file_name().is_some_and(|name| name == "wal")
        && parse_segment_filename(artifact).is_some()
}

pub(super) fn is_checkpoint_artifact(root: &Path, artifact: &Path) -> bool {
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
