use worth_store_certification::S6FlushDurabilityEvidenceRow;
use worth_store_physical_backend::{
    StoreDurabilityAdmission, StoreDurabilityRequirement, StoreDurabilityRuntime,
    WalDurabilityBarrier, WalDurabilityBarrierSet,
};
use worth_store_recovery_physics::DurableCheckpointPublication;
use worth_store_wal::{CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity};

pub(super) fn flush_row() -> S6FlushDurabilityEvidenceRow {
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = StoreDurabilityAdmission::admit(requirement, &super::backend_witness())
        .unwrap()
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(13),
                "sha256:s6-materialized",
                100,
                200,
            )
            .unwrap(),
        )
        .backend_accepted();
    let directory = tempfile::tempdir().unwrap();
    let proof = StoreDurabilityRuntime::new()
        .persist_and_execute(directory.path(), b"evidence-durable-write", &accepted)
        .unwrap();
    let publication = DurableCheckpointPublication::publish(
        accepted
            .reach_durability_boundary(proof)
            .unwrap()
            .parent_namespace_durable()
            .unwrap()
            .rename_durable()
            .unwrap()
            .ordering_barrier_durable()
            .unwrap(),
    )
    .unwrap();
    S6FlushDurabilityEvidenceRow::from_checkpoint_publication(&publication)
}
