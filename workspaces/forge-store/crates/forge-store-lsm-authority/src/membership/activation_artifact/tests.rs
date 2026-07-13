use super::super::artifact::{decode_key_scope, decode_tenant};
use super::super::model::LsmMembershipKey;
use super::super::model::LsmMembershipReadmissionAuthority;
use super::super::session::LsmMembershipDenial;
use super::event::PersistedMembershipActivation;
use super::frame::{decode_activation, encode_activation, HEADER_BYTES};
use crate::{
    BlobWalRecordIdentity, BlobWalRecordKind, CheckpointDurablePublicationScope,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope,
};
use std::path::PathBuf;

#[test]
fn native_activation_round_trips_exactly() {
    let activation = activation();
    let bytes = encode_activation(&activation).unwrap();

    assert_eq!(decode_activation(&bytes, authority()).unwrap(), activation);
}

#[test]
fn every_incomplete_activation_prefix_is_rejected() {
    let bytes = encode_activation(&activation()).unwrap();

    for cut in 0..bytes.len() {
        assert_eq!(
            decode_activation(&bytes[..cut], authority()).unwrap_err(),
            LsmMembershipDenial::PersistedMembershipArtifactInvalid
        );
    }
}

#[test]
fn payload_corruption_is_rejected() {
    let mut bytes = encode_activation(&activation()).unwrap();
    bytes[HEADER_BYTES] ^= 0x01;

    assert_eq!(
        decode_activation(&bytes, authority()).unwrap_err(),
        LsmMembershipDenial::PersistedMembershipArtifactInvalid
    );
}

#[test]
fn header_length_magic_and_version_corruption_are_rejected() {
    for offset in [0, 8, 12] {
        let mut bytes = encode_activation(&activation()).unwrap();
        bytes[offset] ^= 0x01;

        assert_eq!(
            decode_activation(&bytes, authority()).unwrap_err(),
            LsmMembershipDenial::PersistedMembershipArtifactInvalid
        );
    }
}

fn activation() -> PersistedMembershipActivation {
    let key = LsmMembershipKey::readmit(
        authority(),
        decode_tenant(0).unwrap(),
        decode_key_scope(4).unwrap(),
        b"tenant-key",
    )
    .unwrap();
    PersistedMembershipActivation {
        key,
        selected_version: 3,
        selected_identities: [
            BlobWalRecordIdentity::new(10, BlobWalRecordKind::LsmValue).unwrap(),
            BlobWalRecordIdentity::new(11, BlobWalRecordKind::GenerationPublication).unwrap(),
            BlobWalRecordIdentity::new(12, BlobWalRecordKind::LsmTombstone).unwrap(),
        ],
        selected_base: None,
        output_identity: BlobWalRecordIdentity::new(13, BlobWalRecordKind::GenerationPublication)
            .unwrap(),
        output_scope: WalFrameDurablePublicationScope::new(1, 2, 13, 14, "output-digest", 4096)
            .unwrap(),
        output_path: PathBuf::from("output.bin"),
        output_bytes: 4096,
        scope: CheckpointDurablePublicationScope::new(
            StoreCheckpointRecordIdentity::new(7),
            "manifest-digest".to_owned(),
            10,
            20,
        )
        .unwrap(),
    }
}

fn authority() -> LsmMembershipReadmissionAuthority {
    let scope =
        forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    LsmMembershipReadmissionAuthority::from_current_scope(scope.witnesses())
}
