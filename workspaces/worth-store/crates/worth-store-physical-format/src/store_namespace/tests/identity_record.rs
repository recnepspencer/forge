use super::super::*;
use sha2::{Digest, Sha256};

fn proposed(identity: [u8; 16]) -> ProposedStoreIdentity {
    ProposedStoreIdentity::from_nonzero_bytes(identity).expect("nonzero identity")
}

fn encoded_identity(identity: [u8; 16]) -> Vec<u8> {
    StoreNamespaceIdentityRecord::new(StoreNamespaceVersion::CURRENT, proposed(identity))
        .encode()
        .to_vec()
}

fn resign(mut bytes: Vec<u8>) -> Vec<u8> {
    let digest = Sha256::digest(&bytes[..40]);
    bytes[40..].copy_from_slice(&digest);
    bytes
}

#[test]
fn identity_record_round_trips_exact_golden_bytes_at_identity_extremes() {
    const CASES: [([u8; 16], [u8; 72]); 2] = [
        (
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [
                0x57, 0x53, 0x54, 0x4e, 0x53, 0x49, 0x44, 0x00, 0x01, 0x00, 0x01, 0x00, 0x48, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x5e, 0xb1,
                0x4a, 0x0f, 0x01, 0x23, 0xcf, 0xdc, 0xb3, 0xf5, 0x3a, 0x56, 0x13, 0x37, 0xa9, 0x45,
                0x42, 0x5a, 0x3f, 0x97, 0x28, 0x90, 0x11, 0x4d, 0x9c, 0xfd, 0x38, 0x9e, 0x42, 0x8b,
                0x62, 0x23,
            ],
        ),
        (
            [u8::MAX; 16],
            [
                0x57, 0x53, 0x54, 0x4e, 0x53, 0x49, 0x44, 0x00, 0x01, 0x00, 0x01, 0x00, 0x48, 0x00,
                0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x10, 0x00, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xea, 0x4b,
                0xa5, 0x5c, 0xe0, 0x4d, 0x42, 0xf5, 0x34, 0x31, 0x74, 0xf7, 0x85, 0xf0, 0x74, 0x3b,
                0x23, 0x03, 0x1c, 0x33, 0x86, 0x31, 0x74, 0x11, 0x54, 0xcf, 0xbb, 0x67, 0x11, 0xbe,
                0x12, 0x08,
            ],
        ),
    ];

    for (identity, golden) in CASES {
        let record =
            StoreNamespaceIdentityRecord::new(StoreNamespaceVersion::CURRENT, proposed(identity));
        let encoded = record.encode();
        assert_eq!(encoded, golden);
        assert_eq!(StoreNamespaceIdentityRecord::decode(&encoded), Ok(record));
    }
}

#[test]
fn identity_is_unavailable_after_every_structural_or_integrity_violation() {
    let valid = encoded_identity([7; 16]);
    let cases: Vec<(Vec<u8>, StoreNamespaceIdentityDecodeError)> = vec![
        (
            {
                let mut bytes = valid.clone();
                bytes[0] ^= 1;
                bytes
            },
            StoreNamespaceIdentityDecodeError::BadMagic,
        ),
        (
            valid[..valid.len() - 1].to_vec(),
            StoreNamespaceIdentityDecodeError::IncorrectLength,
        ),
        (
            {
                let mut bytes = valid.clone();
                bytes.push(0);
                bytes
            },
            StoreNamespaceIdentityDecodeError::TrailingBytes,
        ),
        (
            resign({
                let mut bytes = valid.clone();
                bytes[8..10].copy_from_slice(&2_u16.to_le_bytes());
                bytes
            }),
            StoreNamespaceIdentityDecodeError::UnsupportedEncodingVersion(2),
        ),
        (
            resign({
                let mut bytes = valid.clone();
                bytes[10..12].copy_from_slice(&2_u16.to_le_bytes());
                bytes
            }),
            StoreNamespaceIdentityDecodeError::UnsupportedNamespaceVersion(2),
        ),
        (
            {
                let mut bytes = valid.clone();
                bytes[12..16].copy_from_slice(&71_u32.to_le_bytes());
                bytes
            },
            StoreNamespaceIdentityDecodeError::IncorrectLength,
        ),
        (
            resign({
                let mut bytes = valid.clone();
                bytes[16..18].copy_from_slice(&0_u16.to_le_bytes());
                bytes
            }),
            StoreNamespaceIdentityDecodeError::MissingIdentityField,
        ),
        (
            resign({
                let mut bytes = valid.clone();
                bytes[16..18].copy_from_slice(&2_u16.to_le_bytes());
                bytes
            }),
            StoreNamespaceIdentityDecodeError::DuplicateIdentityField { declared_count: 2 },
        ),
        (
            resign({
                let mut bytes = valid.clone();
                bytes[18] = 1;
                bytes
            }),
            StoreNamespaceIdentityDecodeError::ReservedBytesNonzero,
        ),
        (
            resign({
                let mut bytes = valid.clone();
                bytes[20..22].copy_from_slice(&2_u16.to_le_bytes());
                bytes
            }),
            StoreNamespaceIdentityDecodeError::UnexpectedIdentityFieldTag(2),
        ),
        (
            resign({
                let mut bytes = valid.clone();
                bytes[22..24].copy_from_slice(&15_u16.to_le_bytes());
                bytes
            }),
            StoreNamespaceIdentityDecodeError::IncorrectIdentityFieldLength(15),
        ),
        (
            {
                let mut bytes = valid.clone();
                bytes[24] ^= 1;
                bytes
            },
            StoreNamespaceIdentityDecodeError::ChecksumMismatch,
        ),
    ];

    for (bytes, expected) in cases {
        assert_eq!(StoreNamespaceIdentityRecord::decode(&bytes), Err(expected));
    }

    let mut zero_identity = valid;
    zero_identity[24..40].fill(0);
    assert_eq!(
        StoreNamespaceIdentityRecord::decode(&resign(zero_identity)),
        Err(StoreNamespaceIdentityDecodeError::ZeroIdentity)
    );
}
