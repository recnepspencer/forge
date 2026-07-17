use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use super::ReplicaBootstrapDenial;

pub const REPLICA_TARGET_DIGEST_BUFFER_BYTES: usize = 64 * 1024;

/// Computes the owner-side identity of a closed replica target.
///
/// Offline verification deliberately reimplements this walk instead of trusting
/// this producer-side result. Paths, lengths, and bytes are all bound so file
/// substitution and boundary ambiguity cannot preserve the identity.
pub fn durable_replica_target_identity(root: &Path) -> Result<[u8; 32], ReplicaBootstrapDenial> {
    let files = canonical_files(root)?;
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-target-v1");
    let mut buffer = vec![0; REPLICA_TARGET_DIGEST_BUFFER_BYTES];
    for (relative, path) in files {
        let relative = relative.to_string_lossy();
        let mut file =
            std::fs::File::open(path).map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
        let length = file
            .metadata()
            .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?
            .len();
        digest.update((relative.len() as u64).to_be_bytes());
        digest.update(relative.as_bytes());
        digest.update(length.to_be_bytes());
        loop {
            let read = file
                .read(&mut buffer)
                .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
            if read == 0 {
                break;
            }
            digest.update(&buffer[..read]);
        }
    }
    Ok(digest.finalize().into())
}

fn canonical_files(root: &Path) -> Result<Vec<(PathBuf, PathBuf)>, ReplicaBootstrapDenial> {
    let root = root
        .canonicalize()
        .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
    if !root.is_dir() {
        return Err(ReplicaBootstrapDenial::ExecutionFailed);
    }
    let mut pending = vec![root.clone()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in
            std::fs::read_dir(directory).map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?
        {
            let path = entry
                .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?
                .path();
            let metadata = std::fs::symlink_metadata(&path)
                .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
            if metadata.file_type().is_symlink() {
                return Err(ReplicaBootstrapDenial::ExecutionFailed);
            }
            if metadata.is_dir() {
                pending.push(path);
            } else if metadata.is_file() {
                let relative = path
                    .strip_prefix(&root)
                    .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?
                    .to_path_buf();
                files.push((relative, path));
            } else {
                return Err(ReplicaBootstrapDenial::ExecutionFailed);
            }
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(files)
}
