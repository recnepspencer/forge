use forge_store_certification::S6FlushDurabilityEvidenceRow;
use forge_store_physical_backend::{
    PhysicalStoreDurabilityExecutor, StoreDurabilityAdmission, StoreDurabilityExecutionObservation,
    StoreDurabilityExecutionRequest, StoreDurabilityExecutionSession, StoreDurabilityFileSyncKind,
    StoreDurabilityRequirement, StoreOwnedDurabilityExecution, WalDurabilityBarrier,
    WalDurabilityBarrierSet,
};
use forge_store_recovery_physics::DurableCheckpointPublication;
use forge_store_wal::{CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity};

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
    let mut backend = DurabilityBackend {
        expected_scope: accepted.scope().clone(),
        requirement,
    };
    let proof = StoreDurabilityExecutionSession::for_store_backend(
        &mut backend,
        StoreOwnedDurabilityExecution::for_certification_test_authority(),
    )
    .execute(&accepted)
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

struct DurabilityBackend<S> {
    expected_scope: S,
    requirement: StoreDurabilityRequirement,
}

impl<S: Eq + core::fmt::Debug> PhysicalStoreDurabilityExecutor<S> for DurabilityBackend<S> {
    type Error = ();

    fn execute_durability(
        &mut self,
        request: StoreDurabilityExecutionRequest<S>,
    ) -> Result<StoreDurabilityExecutionObservation, Self::Error> {
        assert_eq!(request.scope(), &self.expected_scope);
        assert_eq!(request.requirement(), self.requirement);
        Ok(StoreDurabilityExecutionObservation::new(
            self.requirement.required_barriers(),
            StoreDurabilityFileSyncKind::Fsync,
        )
        .with_directory_sync_completed()
        .with_rename_completed()
        .with_ordering_barrier_completed())
    }
}
