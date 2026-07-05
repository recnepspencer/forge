use forge_store_certification::S6FlushDurabilityEvidenceRow;
use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilityKind,
    BackendCapabilitySupportPosture, BackendCapabilitySupportSet, BackendMediaAssumptionSet,
    BackendRebindTriggers, BackendTargetProfile, CapabilityEvidenceClass,
    PhysicalBackendCapabilityAdmissionAuthority, PhysicalStoreDurabilityExecutor,
    StoreDurabilityAdmission, StoreDurabilityDenialKind, StoreDurabilityExecutionObservation,
    StoreDurabilityExecutionRequest, StoreDurabilityExecutionSession, StoreDurabilityFileSyncKind,
    StoreDurabilityPublicationKind, StoreDurabilityRequirement, StoreDurabilityState,
    StoreOwnedDurabilityExecution, WalDurabilityBarrier, WalDurabilityBarrierSet,
};
use forge_store_recovery_physics::{
    DurabilityReplayKind, DurableCheckpointPublication, DurableManifestPublication,
};
use forge_store_wal::{CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity};

#[test]
fn certification_materializes_checkpoint_durability_without_minting_authority() {
    let requirement = StoreDurabilityRequirement::checkpoint_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = admitted(requirement)
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(13),
                "sha256:cert-checkpoint",
                100,
                200,
            )
            .unwrap(),
        )
        .backend_accepted();
    let mut backend = RequestAssertingDurabilityBackend {
        expected_scope: accepted.scope().clone(),
        expected_requirement: requirement,
        expected_publication: StoreDurabilityPublicationKind::Checkpoint,
        observation: StoreDurabilityExecutionObservation::new(
            requirement.required_barriers(),
            StoreDurabilityFileSyncKind::Fsync,
        )
        .with_directory_sync_completed()
        .with_rename_completed()
        .with_ordering_barrier_completed(),
    };
    let proof = StoreDurabilityExecutionSession::for_store_backend(
        &mut backend,
        StoreOwnedDurabilityExecution::for_certification_test_authority(),
    )
    .execute(&accepted)
    .unwrap();
    let receipt = accepted
        .reach_durability_boundary(proof)
        .unwrap()
        .parent_namespace_durable()
        .unwrap()
        .rename_durable()
        .unwrap()
        .ordering_barrier_durable()
        .unwrap();
    let publication = DurableCheckpointPublication::publish(receipt).unwrap();

    let row = S6FlushDurabilityEvidenceRow::from_checkpoint_publication(&publication);

    assert_eq!(
        row.required_state(),
        StoreDurabilityState::OrderingBarrierDurable
    );
    assert_eq!(
        row.replay_identity().kind(),
        DurabilityReplayKind::Checkpoint
    );
    assert_eq!(row.replay_identity().digest(), "sha256:cert-checkpoint");
    assert_eq!(row.counters().directory_syncs_completed(), 1);
    assert_eq!(row.counters().renames_completed(), 1);
}

#[test]
fn certification_materializes_manifest_durability_evidence() {
    let requirement = StoreDurabilityRequirement::manifest_publication(
        WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
            .insert(WalDurabilityBarrier::WalDirectoryFsync),
    );
    let accepted = admitted(requirement)
        .submit_write(
            CheckpointDurablePublicationScope::new(
                StoreCheckpointRecordIdentity::new(14),
                "sha256:cert-manifest",
                201,
                300,
            )
            .unwrap(),
        )
        .backend_accepted();
    let mut backend = RequestAssertingDurabilityBackend {
        expected_scope: accepted.scope().clone(),
        expected_requirement: requirement,
        expected_publication: StoreDurabilityPublicationKind::Manifest,
        observation: StoreDurabilityExecutionObservation::new(
            requirement.required_barriers(),
            StoreDurabilityFileSyncKind::Fsync,
        )
        .with_directory_sync_completed()
        .with_rename_completed()
        .with_ordering_barrier_completed(),
    };
    let proof = StoreDurabilityExecutionSession::for_store_backend(
        &mut backend,
        StoreOwnedDurabilityExecution::for_certification_test_authority(),
    )
    .execute(&accepted)
    .unwrap();
    let receipt = accepted
        .reach_durability_boundary(proof)
        .unwrap()
        .parent_namespace_durable()
        .unwrap()
        .rename_durable()
        .unwrap()
        .ordering_barrier_durable()
        .unwrap();
    let publication = DurableManifestPublication::publish(receipt).unwrap();

    let row = S6FlushDurabilityEvidenceRow::from_manifest_publication(&publication);

    assert_eq!(
        row.required_state(),
        StoreDurabilityState::OrderingBarrierDurable
    );
    assert_eq!(row.replay_identity().kind(), DurabilityReplayKind::Manifest);
    assert_eq!(row.replay_identity().digest(), "sha256:cert-manifest");
    assert_eq!(row.counters().fsyncs_completed(), 1);
    assert_eq!(row.counters().directory_syncs_completed(), 1);
    assert_eq!(row.counters().renames_completed(), 1);
    assert_eq!(row.counters().ordering_barriers_completed(), 1);
}

#[test]
fn certification_records_unknown_posture_as_denial_not_durable_evidence() {
    let support = BackendCapabilitySupportSet::buffered_durable_only().with_posture(
        BackendCapabilityKind::DirectorySync,
        BackendCapabilitySupportPosture::Unknown,
    );
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            support,
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap();

    let denial = StoreDurabilityAdmission::admit(
        StoreDurabilityRequirement::checkpoint_publication(WalDurabilityBarrierSet::of(
            WalDurabilityBarrier::WalFileFsync,
        )),
        &witness,
    )
    .unwrap_err();

    assert_eq!(denial.state(), StoreDurabilityState::DurabilityUnknown);
    assert_eq!(
        denial.kind(),
        StoreDurabilityDenialKind::UnknownDurabilityPosture
    );
    assert_eq!(denial.counters().unknown_claims(), 1);
}

fn admitted(requirement: StoreDurabilityRequirement) -> StoreDurabilityAdmission {
    let witness = PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::buffered_durable_only(),
            BackendMediaAssumptionSet::platform_file_defaults(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .unwrap();
    StoreDurabilityAdmission::admit(requirement, &witness).unwrap()
}

struct RequestAssertingDurabilityBackend<S> {
    expected_scope: S,
    expected_requirement: StoreDurabilityRequirement,
    expected_publication: StoreDurabilityPublicationKind,
    observation: StoreDurabilityExecutionObservation,
}

impl<S> PhysicalStoreDurabilityExecutor<S> for RequestAssertingDurabilityBackend<S>
where
    S: Eq + core::fmt::Debug,
{
    type Error = ();

    fn execute_durability(
        &mut self,
        request: StoreDurabilityExecutionRequest<S>,
    ) -> Result<StoreDurabilityExecutionObservation, Self::Error> {
        assert_eq!(request.scope(), &self.expected_scope);
        assert_eq!(
            request.profile(),
            BackendTargetProfile::PosixFileFsyncDirSync
        );
        assert_eq!(
            request.evidence_class(),
            CapabilityEvidenceClass::CertifiedBackendProfile
        );
        assert_eq!(request.requirement(), self.expected_requirement);
        assert_eq!(
            request.requirement().publication(),
            self.expected_publication
        );
        Ok(self.observation)
    }
}
