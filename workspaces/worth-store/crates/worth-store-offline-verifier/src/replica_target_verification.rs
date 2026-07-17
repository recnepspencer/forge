use std::io::Read;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use worth_store_replication::{ReplicaBootstrapReceipt, ReplicaPromotionReceipt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplicaTargetVerificationBudget {
    buffer_bytes: usize,
}

impl ReplicaTargetVerificationBudget {
    pub const fn bounded(buffer_bytes: usize) -> Option<Self> {
        if buffer_bytes == 0 {
            return None;
        }
        Some(Self { buffer_bytes })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplicaTargetVerificationDenial {
    InvalidMedia,
    SymbolicLink,
    ReadFailure,
    TargetIdentityMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndependentlyVerifiedReplicaTarget {
    receipt_identity: [u8; 32],
    target_identity: [u8; 32],
    verification_identity: [u8; 32],
    files_checked: u64,
    bytes_checked: u64,
    peak_buffer_bytes: usize,
}

impl IndependentlyVerifiedReplicaTarget {
    pub const fn receipt_identity(&self) -> [u8; 32] {
        self.receipt_identity
    }

    pub const fn target_identity(&self) -> [u8; 32] {
        self.target_identity
    }

    pub const fn verification_identity(&self) -> [u8; 32] {
        self.verification_identity
    }

    pub const fn files_checked(&self) -> u64 {
        self.files_checked
    }

    pub const fn bytes_checked(&self) -> u64 {
        self.bytes_checked
    }

    pub const fn peak_buffer_bytes(&self) -> usize {
        self.peak_buffer_bytes
    }
}

/// Reopens a closed bootstrap target and independently recomputes its closure
/// identity. This implementation does not call the producer-side digest helper.
pub fn verify_replica_bootstrap_target(
    receipt: &ReplicaBootstrapReceipt,
    target_root: &Path,
    budget: ReplicaTargetVerificationBudget,
) -> Result<IndependentlyVerifiedReplicaTarget, ReplicaTargetVerificationDenial> {
    verify_target(
        receipt.receipt_identity(),
        receipt.durable_target_identity(),
        target_root,
        budget,
    )
}

pub fn verify_replica_promotion_target(
    receipt: &ReplicaPromotionReceipt,
    target_root: &Path,
    budget: ReplicaTargetVerificationBudget,
) -> Result<IndependentlyVerifiedReplicaTarget, ReplicaTargetVerificationDenial> {
    verify_target(
        receipt.receipt_identity(),
        receipt.durable_target_identity(),
        target_root,
        budget,
    )
}

fn verify_target(
    receipt_identity: [u8; 32],
    expected_target_identity: [u8; 32],
    target_root: &Path,
    budget: ReplicaTargetVerificationBudget,
) -> Result<IndependentlyVerifiedReplicaTarget, ReplicaTargetVerificationDenial> {
    let files = canonical_files(target_root)?;
    let mut digest = Sha256::new();
    digest.update(b"worth-store-replica-target-v1");
    let mut buffer = vec![0; budget.buffer_bytes];
    let mut bytes_checked = 0_u64;
    for (relative, path) in &files {
        let relative = relative.to_string_lossy();
        let mut file =
            std::fs::File::open(path).map_err(|_| ReplicaTargetVerificationDenial::ReadFailure)?;
        let length = file
            .metadata()
            .map_err(|_| ReplicaTargetVerificationDenial::ReadFailure)?
            .len();
        digest.update((relative.len() as u64).to_be_bytes());
        digest.update(relative.as_bytes());
        digest.update(length.to_be_bytes());
        loop {
            let read = file
                .read(&mut buffer)
                .map_err(|_| ReplicaTargetVerificationDenial::ReadFailure)?;
            if read == 0 {
                break;
            }
            bytes_checked = bytes_checked.saturating_add(read as u64);
            digest.update(&buffer[..read]);
        }
    }
    let target_identity: [u8; 32] = digest.finalize().into();
    if target_identity != expected_target_identity {
        return Err(ReplicaTargetVerificationDenial::TargetIdentityMismatch);
    }
    let files_checked = files.len() as u64;
    let verification_identity = Sha256::digest(
        [
            b"worth-store-replica-target-verification-v1".as_slice(),
            receipt_identity.as_slice(),
            target_identity.as_slice(),
            &files_checked.to_be_bytes(),
            &bytes_checked.to_be_bytes(),
        ]
        .concat(),
    )
    .into();
    Ok(IndependentlyVerifiedReplicaTarget {
        receipt_identity,
        target_identity,
        verification_identity,
        files_checked,
        bytes_checked,
        peak_buffer_bytes: buffer.len(),
    })
}

fn canonical_files(
    root: &Path,
) -> Result<Vec<(PathBuf, PathBuf)>, ReplicaTargetVerificationDenial> {
    let root = root
        .canonicalize()
        .map_err(|_| ReplicaTargetVerificationDenial::InvalidMedia)?;
    if !root.is_dir() {
        return Err(ReplicaTargetVerificationDenial::InvalidMedia);
    }
    let mut pending = vec![root.clone()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory)
            .map_err(|_| ReplicaTargetVerificationDenial::ReadFailure)?
        {
            let path = entry
                .map_err(|_| ReplicaTargetVerificationDenial::ReadFailure)?
                .path();
            let metadata = std::fs::symlink_metadata(&path)
                .map_err(|_| ReplicaTargetVerificationDenial::ReadFailure)?;
            if metadata.file_type().is_symlink() {
                return Err(ReplicaTargetVerificationDenial::SymbolicLink);
            }
            if metadata.is_dir() {
                pending.push(path);
            } else if metadata.is_file() {
                files.push((
                    path.strip_prefix(&root)
                        .map_err(|_| ReplicaTargetVerificationDenial::InvalidMedia)?
                        .to_path_buf(),
                    path,
                ));
            } else {
                return Err(ReplicaTargetVerificationDenial::InvalidMedia);
            }
        }
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_store_replication::durable_replica_target_identity;

    #[test]
    fn independent_walk_accepts_the_closed_owner_identity_with_bounded_memory() {
        let target = tempfile::tempdir().unwrap();
        std::fs::create_dir(target.path().join("nested")).unwrap();
        std::fs::write(target.path().join("page"), vec![1; 8192]).unwrap();
        std::fs::write(target.path().join("nested").join("blob"), vec![2; 4096]).unwrap();
        let expected = durable_replica_target_identity(target.path()).unwrap();
        let verified = verify_target(
            [3; 32],
            expected,
            target.path(),
            ReplicaTargetVerificationBudget::bounded(257).unwrap(),
        )
        .unwrap();
        assert_eq!(verified.target_identity(), expected);
        assert_eq!(verified.files_checked(), 2);
        assert_eq!(verified.bytes_checked(), 12_288);
        assert_eq!(verified.peak_buffer_bytes(), 257);
    }

    #[test]
    fn post_copy_corruption_is_rejected_by_the_independent_walk() {
        let target = tempfile::tempdir().unwrap();
        std::fs::write(target.path().join("page"), b"before").unwrap();
        let expected = durable_replica_target_identity(target.path()).unwrap();
        std::fs::write(target.path().join("page"), b"after").unwrap();
        assert_eq!(
            verify_target(
                [3; 32],
                expected,
                target.path(),
                ReplicaTargetVerificationBudget::bounded(64).unwrap(),
            )
            .unwrap_err(),
            ReplicaTargetVerificationDenial::TargetIdentityMismatch,
        );
    }
}
