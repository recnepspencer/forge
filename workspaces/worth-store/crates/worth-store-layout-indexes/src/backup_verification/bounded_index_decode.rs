use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::{decode_baseline_btree_leaf_record, decode_baseline_btree_root_record};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutIndexBackupFormat {
    BaselineBTreeLeafV1,
    BaselineBTreeRootV1,
}

impl LayoutIndexBackupFormat {
    const fn encoded_bytes(self) -> usize {
        match self {
            Self::BaselineBTreeLeafV1 => 6,
            Self::BaselineBTreeRootV1 => 56,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundedLayoutIndexVerificationRequest {
    format: LayoutIndexBackupFormat,
    expected_bytes: u64,
    expected_digest: [u8; 32],
    max_buffer_bytes: usize,
}

impl BoundedLayoutIndexVerificationRequest {
    pub const fn new(
        format: LayoutIndexBackupFormat,
        expected_bytes: u64,
        expected_digest: [u8; 32],
        max_buffer_bytes: usize,
    ) -> Self {
        Self {
            format,
            expected_bytes,
            expected_digest,
            max_buffer_bytes,
        }
    }
}

#[derive(Debug)]
pub enum BoundedLayoutIndexDenial {
    Io(std::io::Error),
    BufferTooSmall { required: usize, actual: usize },
    LengthMismatch { expected: u64, actual: u64 },
    DigestMismatch,
    MalformedIndex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundedLayoutIndexObservation {
    bytes_read: u64,
    peak_buffer_bytes: u64,
}

impl BoundedLayoutIndexObservation {
    pub const fn bytes_read(self) -> u64 {
        self.bytes_read
    }

    pub const fn peak_buffer_bytes(self) -> u64 {
        self.peak_buffer_bytes
    }
}

pub fn verify_bounded_layout_index_artifact(
    path: &Path,
    request: BoundedLayoutIndexVerificationRequest,
) -> Result<BoundedLayoutIndexObservation, BoundedLayoutIndexDenial> {
    let mut file = std::fs::File::open(path).map_err(BoundedLayoutIndexDenial::Io)?;
    let actual = file.metadata().map_err(BoundedLayoutIndexDenial::Io)?.len();
    verify_bounded_layout_index_artifact_from_reader(&mut file, actual, request)
}

pub fn verify_bounded_layout_index_artifact_from_reader(
    reader: &mut impl Read,
    actual: u64,
    request: BoundedLayoutIndexVerificationRequest,
) -> Result<BoundedLayoutIndexObservation, BoundedLayoutIndexDenial> {
    let required = request.format.encoded_bytes();
    if request.max_buffer_bytes < required {
        return Err(BoundedLayoutIndexDenial::BufferTooSmall {
            required,
            actual: request.max_buffer_bytes,
        });
    }
    if request.expected_bytes != required as u64 {
        return Err(BoundedLayoutIndexDenial::LengthMismatch {
            expected: required as u64,
            actual: request.expected_bytes,
        });
    }
    if actual != request.expected_bytes {
        return Err(BoundedLayoutIndexDenial::LengthMismatch {
            expected: request.expected_bytes,
            actual,
        });
    }
    let mut bytes = [0_u8; 56];
    reader
        .read_exact(&mut bytes[..required])
        .map_err(BoundedLayoutIndexDenial::Io)?;
    if <[u8; 32]>::from(Sha256::digest(&bytes[..required])) != request.expected_digest {
        return Err(BoundedLayoutIndexDenial::DigestMismatch);
    }
    let valid = match request.format {
        LayoutIndexBackupFormat::BaselineBTreeLeafV1 => {
            decode_baseline_btree_leaf_record(&bytes[..required]).is_some()
        }
        LayoutIndexBackupFormat::BaselineBTreeRootV1 => {
            decode_baseline_btree_root_record(&bytes[..required]).is_some()
        }
    };
    if !valid {
        return Err(BoundedLayoutIndexDenial::MalformedIndex);
    }
    Ok(BoundedLayoutIndexObservation {
        bytes_read: required as u64,
        peak_buffer_bytes: required as u64,
    })
}

#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};
    use worth_store_physical_format::PhysicalRecordSlot;

    use super::*;

    #[test]
    fn bounded_layout_owner_decode_rejects_reserved_leaf_flags() {
        let slots = [
            PhysicalRecordSlot::from_raw(1).expect("slot"),
            PhysicalRecordSlot::from_raw(2).expect("slot"),
        ];
        let mut bytes = crate::encode_baseline_btree_leaf_record(slots, true, false);
        bytes[1] |= 0b1000_0000;
        let path = temporary_index_path();
        std::fs::write(&path, bytes).expect("index bytes");
        let denial = verify_bounded_layout_index_artifact(
            &path,
            BoundedLayoutIndexVerificationRequest::new(
                LayoutIndexBackupFormat::BaselineBTreeLeafV1,
                bytes.len() as u64,
                Sha256::digest(bytes).into(),
                64,
            ),
        )
        .expect_err("outer digest cannot legalize reserved owner-format bits");

        assert!(matches!(denial, BoundedLayoutIndexDenial::MalformedIndex));
        let _ = std::fs::remove_file(path);
    }

    fn temporary_index_path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "worth-store-layout-index-{}-{:?}.bin",
            std::process::id(),
            std::thread::current().id()
        ))
    }
}
