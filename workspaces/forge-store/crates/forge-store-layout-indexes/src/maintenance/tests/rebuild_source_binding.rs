use crate::maintenance::test_support::{
    root_manifest_source_witness, root_manifest_source_witness_for_store,
    wal_replay_source_witness_for_identity, wal_replay_source_witness_with_security,
};
use crate::maintenance::{layout_rebuild_admission, DerivedIndexRebuildAdmissionView};
use crate::strategy::tests_support::{
    admit_btree_page_strategy, admit_persisted_lsm_strategy,
    strategy_test_wal_security_scope_for_store,
};
use crate::{
    access_shapes, AccessLaneClassification, DerivedIndexRebuildDenied, DerivedIndexRebuildRequest,
    DerivedIndexRebuildSourceInput, LayoutMaterializationSourceKind,
};
use forge_store_wal::{BlobWalRecordIdentity, BlobWalRecordKind};

#[test]
fn root_manifest_rebuild_rejects_an_equal_root_from_another_store() {
    let strategy = admit_btree_page_strategy();
    let shape = access_shapes()
        .rebuild_read(AccessLaneClassification::Maintenance)
        .unwrap();
    let ordinary_source = root_manifest_source_witness(7, 11);
    let materialization = crate::maintenance::test_support::root_materialization(
        strategy.admitted_family(),
        &ordinary_source,
    );
    let key = forge_foundational::aspects()
        .vocabulary()
        .key("store.foreign.root-rebuild")
        .expect("foreign Store key");
    let foreign_store = forge_store_physical_format::PhysicalStoreIdentity::from_aspect_identity(
        forge_store_aspect_native::StoreAspectIdentity::from_aspect_key(key),
    );
    let foreign_source = root_manifest_source_witness_for_store(7, 11, foreign_store);

    let outcome = layout_rebuild_admission().admit_plan(DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        shape,
        materialization,
        DerivedIndexRebuildSourceInput::PhysicalRootManifest {
            source: foreign_source,
        },
    ));

    assert!(matches!(
        outcome.view(),
        DerivedIndexRebuildAdmissionView::Denied(
            DerivedIndexRebuildDenied::SourceStoreAuthorityMismatch { .. }
        )
    ));
}

#[test]
fn wal_rebuild_rejects_a_different_record_under_the_same_security_authority() {
    let (strategy, shape, materialization) = setup();
    let LayoutMaterializationSourceKind::LsmReplacement(expected) = materialization.source().kind()
    else {
        panic!("persisted LSM materialization must retain replacement identity");
    };
    let security =
        forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let source = wal_replay_source_witness_for_identity(
        BlobWalRecordIdentity::new(
            expected.sequence().checked_add(1).unwrap(),
            BlobWalRecordKind::GenerationPublication,
        )
        .unwrap(),
        security.witnesses(),
    );

    let outcome =
        layout_rebuild_admission().admit_plan(request(strategy, shape, materialization, source));
    assert!(matches!(
        outcome.view(),
        DerivedIndexRebuildAdmissionView::Denied(
            DerivedIndexRebuildDenied::SourceMaterializationIdentityMismatch { .. }
        )
    ));
}

#[test]
fn wal_rebuild_rejects_cross_tenant_security_scope() {
    let (strategy, shape, materialization) = setup();
    let foreign_scope =
        forge_store_security::admitted_tenant_wal_checkpoint_security_scope_for_layout_partition_test();
    let source = wal_replay_source_witness_with_security(
        &materialization,
        BlobWalRecordKind::GenerationPublication,
        foreign_scope.witnesses(),
    );

    let outcome =
        layout_rebuild_admission().admit_plan(request(strategy, shape, materialization, source));
    assert!(matches!(
        outcome.view(),
        DerivedIndexRebuildAdmissionView::Denied(
            DerivedIndexRebuildDenied::SourceSecurityScopeMismatch { .. }
        )
    ));
}

#[test]
fn wal_rebuild_rejects_equal_scope_from_another_store_authority() {
    let (strategy, shape, materialization) = setup();
    let foreign_scope = strategy_test_wal_security_scope_for_store("store.foreign.rebuild");
    let source = wal_replay_source_witness_with_security(
        &materialization,
        BlobWalRecordKind::GenerationPublication,
        foreign_scope.witnesses(),
    );

    let outcome =
        layout_rebuild_admission().admit_plan(request(strategy, shape, materialization, source));
    assert!(matches!(
        outcome.view(),
        DerivedIndexRebuildAdmissionView::Denied(
            DerivedIndexRebuildDenied::SourceStoreAuthorityMismatch { .. }
        )
    ));
}

fn setup() -> (
    crate::strategy::AdmittedLayoutStrategy,
    crate::AccessShapeContract,
    crate::AdmittedLayoutMaterialization,
) {
    let strategy = admit_persisted_lsm_strategy();
    let shape = access_shapes()
        .rebuild_read(AccessLaneClassification::Maintenance)
        .unwrap();
    let materialization =
        crate::maintenance::test_support::wal_materialization(strategy.admitted_family());
    (strategy, shape, materialization)
}

fn request(
    strategy: crate::strategy::AdmittedLayoutStrategy,
    shape: crate::AccessShapeContract,
    materialization: crate::AdmittedLayoutMaterialization,
    source_witness: forge_store_wal::BlobWalReplayRebuildWitness,
) -> DerivedIndexRebuildRequest {
    DerivedIndexRebuildRequest::new(
        strategy.admitted_family(),
        strategy.admitted_key_domain(),
        strategy.family(),
        shape,
        materialization,
        DerivedIndexRebuildSourceInput::WalReplayRecord { source_witness },
    )
}
