use crate::{
    CheckpointPublicationScope, LogSequenceNumber, PublicationDeclaration, PublicationScope,
    StoreCheckpointRecordIdentity, WalFramePublicationScope, WalLsnRange, WalSegmentGeneration,
    WalSegmentId,
};

#[test]
fn wal_publication_declaration_preserves_scope_without_durability_claim() {
    let scope = WalFramePublicationScope::new(
        WalSegmentId::new(7).unwrap(),
        WalSegmentGeneration::new(2).unwrap(),
        WalLsnRange::new(LogSequenceNumber::new(11), LogSequenceNumber::new(15)).unwrap(),
        "sha256:frame",
        4096,
    )
    .expect("valid wal scope");
    let declaration = PublicationDeclaration::wal_frame(scope.clone());

    let PublicationScope::WalFrame(declared) = declaration.scope() else {
        panic!("expected wal scope");
    };
    assert_eq!(declared, &scope);
    assert_eq!(declared.segment_id(), 7);
    assert_eq!(declared.lsn_start(), 11);
    assert_eq!(declared.frame_digest(), "sha256:frame");
}

#[test]
fn checkpoint_publication_declaration_preserves_identity() {
    let scope = CheckpointPublicationScope::new(
        StoreCheckpointRecordIdentity::new(9),
        "sha256:checkpoint",
        20,
        40,
    )
    .expect("valid checkpoint scope");
    let declaration = PublicationDeclaration::checkpoint(scope.clone());

    let PublicationScope::Checkpoint(declared) = declaration.scope() else {
        panic!("expected checkpoint scope");
    };
    assert_eq!(declared, &scope);
    assert_eq!(declared.checkpoint(), StoreCheckpointRecordIdentity::new(9));
}
