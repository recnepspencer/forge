use crate::{
    CheckpointDurablePublicationScope, DurablePublicationDeclaration, DurablePublicationScope,
    StoreCheckpointRecordIdentity, WalFrameDurablePublicationScope,
};

#[test]
fn wal_publication_declaration_preserves_scope_without_durability_claim() {
    let scope = WalFrameDurablePublicationScope::new(7, 2, 11, 15, "sha256:frame", 4096)
        .expect("valid wal scope");
    let declaration = DurablePublicationDeclaration::wal_frame(scope.clone());

    let DurablePublicationScope::WalFrame(declared) = declaration.scope() else {
        panic!("expected wal scope");
    };
    assert_eq!(declared, &scope);
    assert_eq!(declared.segment_id(), 7);
    assert_eq!(declared.lsn_start(), 11);
    assert_eq!(declared.frame_digest(), "sha256:frame");
}

#[test]
fn checkpoint_publication_declaration_preserves_identity() {
    let scope = CheckpointDurablePublicationScope::new(
        StoreCheckpointRecordIdentity::new(9),
        "sha256:checkpoint",
        20,
        40,
    )
    .expect("valid checkpoint scope");
    let declaration = DurablePublicationDeclaration::checkpoint(scope.clone());

    let DurablePublicationScope::Checkpoint(declared) = declaration.scope() else {
        panic!("expected checkpoint scope");
    };
    assert_eq!(declared, &scope);
    assert_eq!(declared.checkpoint(), StoreCheckpointRecordIdentity::new(9));
}
