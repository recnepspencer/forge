mod material;
mod mutable_reopen;
mod root;
mod wal_data;

pub(crate) use material::certification_material;
pub(crate) use mutable_reopen::checkpoint_for_mutable_reopen;
pub(crate) use root::{publish_single, publish_single_with_manifest_capacity_transition};
pub(crate) use wal_data::{prepare_single, settle_single};

#[test]
fn canonical_single_member_driver_reaches_namespace_durable_current_root() {
    let parent = tempfile::tempdir().unwrap();
    let serving = crate::serving_from_initialization(&parent.path().join("store"));
    let (_, placement, _) = crate::configuration();
    let completed = publish_single(
        &serving,
        placement,
        worth_store::physical_runtime::PhysicalMutationIdempotencyMaterial::new([148; 32]),
        worth_store::physical_runtime::RecordAppendBatch::try_from_iter([
            b"canonical-publication-driver".as_slice(),
        ])
        .unwrap(),
    );

    assert_eq!(completed.current_root().generation(), 2);
    assert_eq!(completed.retained_root().manifest().generation(), 1);
    assert_eq!(completed.settled_members().len(), 1);
    assert!(completed.settled_members()[0].record_id(0).is_some());
    serving.close();
}
