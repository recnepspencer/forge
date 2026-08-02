use super::*;

const DIGEST: [u8; 32] = [0xA5; 32];

#[test]
fn v6_round_trips_every_recovery_target_family() {
    let range = RecordFrameCoordinate::new(RecordArtifactFile::BootstrapCatalog, 9, 17)
        .expect("nonempty coordinate");
    let targets = [
        (PhysicalWorkRecoveryTarget::Range(range), Some(DIGEST)),
        (
            PhysicalWorkRecoveryTarget::WalArtifactInterval {
                segment: 4,
                generation: 7,
                offset: 0x0102_0304_0506_0708,
                byte_count: 0x1112_1314_1516_1718,
            },
            Some(DIGEST),
        ),
        (
            PhysicalWorkRecoveryTarget::Checkpoint {
                sequence: 8,
                action: PhysicalCheckpointRecoveryAction::CreateCandidate { byte_count: 113 },
            },
            Some(DIGEST),
        ),
        (
            PhysicalWorkRecoveryTarget::Checkpoint {
                sequence: 8,
                action: PhysicalCheckpointRecoveryAction::AppendCandidate {
                    offset: 113,
                    byte_count: 29,
                },
            },
            Some(DIGEST),
        ),
        (
            PhysicalWorkRecoveryTarget::Checkpoint {
                sequence: 8,
                action: PhysicalCheckpointRecoveryAction::SynchronizeCandidate,
            },
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::Checkpoint {
                sequence: 8,
                action: PhysicalCheckpointRecoveryAction::RemoveCandidate,
            },
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::Checkpoint {
                sequence: 8,
                action: PhysicalCheckpointRecoveryAction::PublishCandidate,
            },
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::Checkpoint {
                sequence: 8,
                action: PhysicalCheckpointRecoveryAction::SynchronizeNamespace,
            },
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::ArtifactFileSynchronization(
                RecordArtifactFile::RootManifest { generation: 3 },
            ),
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::ArtifactParentSynchronization(
                RecordArtifactFile::RootManifest { generation: 3 },
            ),
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::CatalogReplacement(RecordArtifactFile::CatalogCandidate {
                publication: 11,
            }),
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::RecordNamespaceSynchronization,
            None,
        ),
        (
            PhysicalWorkRecoveryTarget::WalSegmentReclamation {
                segment: 17,
                generation: 3,
            },
            None,
        ),
    ];
    for (target, digest) in targets {
        let record = encoded(target, digest);
        assert_eq!(decode_target(&record), Some((target, digest)));
    }
}

#[test]
fn wal_reclamation_rejects_payload_and_interval_substitution() {
    let target = PhysicalWorkRecoveryTarget::WalSegmentReclamation {
        segment: 17,
        generation: 3,
    };
    let mut record = encoded(target, None);
    assert_eq!(decode_target(&record), Some((target, None)));
    record[64..72].copy_from_slice(&1_u64.to_le_bytes());
    assert_eq!(decode_target(&record), None);
    let record = encoded(target, Some(DIGEST));
    assert_eq!(decode_target(&record), None);
}

#[test]
fn wal_interval_cannot_overwrite_target_or_digest_fields() {
    let target = PhysicalWorkRecoveryTarget::WalArtifactInterval {
        segment: 9,
        generation: 12,
        offset: 0xFEED_FACE_CAFE_BEEF,
        byte_count: 0x0102_0304_0506_0708,
    };
    let record = encoded(target, Some(DIGEST));
    assert_eq!(record[104], TARGET_WAL_INTERVAL);
    assert_eq!(record[105], 1);
    assert_eq!(&record[72..104], &DIGEST);
    assert_eq!(decode_target(&record), Some((target, Some(DIGEST))));
}

#[test]
fn hostile_checkpoint_shape_is_rejected() {
    let target = PhysicalWorkRecoveryTarget::Checkpoint {
        sequence: 4,
        action: PhysicalCheckpointRecoveryAction::SynchronizeCandidate,
    };
    let mut record = encoded(target, None);
    record[105] = 1;
    record[72..104].copy_from_slice(&DIGEST);
    assert_eq!(decode_target(&record), None);
    record[105] = 0;
    record[104] = 0xFF;
    assert_eq!(decode_target(&record), None);
}

fn encoded(
    target: PhysicalWorkRecoveryTarget,
    digest: Option<[u8; 32]>,
) -> [u8; RECOVERY_RECORD_BYTES] {
    let mut record = [0; RECOVERY_RECORD_BYTES];
    encode_target(target, &mut record);
    if let Some(digest) = digest {
        record[105] = 1;
        record[72..104].copy_from_slice(&digest);
    }
    record
}
