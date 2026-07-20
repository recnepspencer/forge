use sha2::{Digest, Sha256};

pub(in crate::workflow::repair) fn physical_target_identity(
    path: &std::path::Path,
) -> Option<[u8; 32]> {
    let canonical = std::fs::canonicalize(path).ok()?;
    let value = canonical.as_os_str().to_string_lossy();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-repair-physical-target-v1");
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value.as_bytes());
    Some(digest.finalize().into())
}
