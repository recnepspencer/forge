use sha2::{Digest, Sha256};

use super::{
    prepare_wal_frame_append, verify_bounded_wal_segment, BoundedWalSegmentDenial,
    BoundedWalSegmentVerificationRequest,
};

#[test]
fn bounded_owner_decode_accepts_exact_contiguous_frame_stream() {
    let fixture = WalSegmentFixture::new();
    let observation = verify_bounded_wal_segment(&fixture.path, fixture.request(&fixture.bytes))
        .expect("canonical WAL segment");

    assert_eq!(observation.frame_count(), 2);
    assert_eq!(observation.bytes_read(), fixture.bytes.len() as u64);
    assert!(observation.peak_buffer_bytes() <= 181);
}

#[test]
fn owner_decode_rejects_rehashed_payload_corruption() {
    let fixture = WalSegmentFixture::new();
    let mut corrupted = fixture.bytes.clone();
    corrupted[116] ^= 0x40;
    std::fs::write(&fixture.path, &corrupted).expect("mutated WAL");

    let denial = verify_bounded_wal_segment(&fixture.path, fixture.request(&corrupted))
        .expect_err("outer digest cannot replace WAL frame integrity");

    assert!(matches!(
        denial,
        BoundedWalSegmentDenial::PayloadDigestMismatch
    ));
}

struct WalSegmentFixture {
    _directory: tempfile::TempDir,
    path: std::path::PathBuf,
    bytes: Vec<u8>,
}

impl WalSegmentFixture {
    fn new() -> Self {
        let directory = tempfile::Builder::new()
            .prefix("worth-store-wal-owner-decode-")
            .tempdir()
            .expect("fixture directory");
        let first = prepare_wal_frame_append(directory.path(), 7, 3, 10, 11, "frame-a", b"first")
            .expect("first frame");
        let second = prepare_wal_frame_append(directory.path(), 7, 3, 11, 12, "frame-b", b"second")
            .expect("second frame");
        let mut bytes = first.encoded_frame().to_vec();
        bytes.extend_from_slice(second.encoded_frame());
        let path = directory.path().join("segment.wal");
        std::fs::write(&path, &bytes).expect("WAL segment");
        Self {
            _directory: directory,
            path,
            bytes,
        }
    }

    fn request(&self, bytes: &[u8]) -> BoundedWalSegmentVerificationRequest {
        BoundedWalSegmentVerificationRequest::new(
            7,
            3,
            10,
            12,
            bytes.len() as u64,
            Sha256::digest(bytes).into(),
            181,
        )
        .expect("bounded request")
    }
}
