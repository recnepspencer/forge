use forge_store_physical_backend::{
    BackendCapabilityAdmissionRequest, BackendCapabilityEvidenceBasis, BackendCapabilitySupportSet,
    BackendMediaAssumptionSet, BackendRebindTriggers, BackendTargetProfile,
    BlobPhysicalManifestObservation, BlobPhysicalManifestValidation, CapabilityEvidenceClass,
    PhysicalBackendCapabilityAdmissionAuthority, PhysicalStoreDurabilityExecutor,
    StoreDurabilityAdmission, StoreDurabilityExecutionObservation, StoreDurabilityExecutionRequest,
    StoreDurabilityExecutionSession, StoreDurabilityFileSyncKind, StoreDurabilityPublicationKind,
    StoreDurabilityRequirement, StoreOwnedDurabilityExecution, WalDurabilityBarrier,
    WalDurabilityBarrierSet,
};
use forge_store_recovery_physics::{
    BlobReplaySourceAdmission, DurableCheckpointPublication, DurableManifestPublication,
};
use forge_store_wal::{CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity};

use crate::blob_publication_commit_test_support::{
    durable_wal_publication, publication_inputs, replayable_wal_classification,
};
use crate::BlobAdmittedRecoveryRecords;
use crate::{
    BlobCheckpointFrontierRecord, BlobChunkAppendRecord, BlobGenerationPublicationRecord,
    BlobManifestAgreement, BlobPlacementManifestRow, BlobPublicationWalCommit,
    BlobPublicationWalPayload, BlobPublicationWalRecord, BlobReachabilityManifestRow,
    BlobReachabilityStaging, BlobRecoveryOutcome, BlobRecoveryRecordDenialKind,
    BlobRecoveryRecordSet, BlobRecoveryReplay, BlobResumeSessionCheckpointRecord,
    BlobRootCandidateRecord,
};

#[test]
fn admitted_wal_checkpoint_manifest_replay_reconstructs_blob_facts() {
    let fixture = replay_fixture("phase7-replay");
    let record_counters = fixture.records.counters();
    assert_eq!(
        (
            record_counters.wal_records(),
            record_counters.checkpoint_records(),
            record_counters.manifest_rows(),
            record_counters.replayed_outcomes(),
        ),
        (3, 2, 2, 0)
    );

    let replay = BlobRecoveryReplay::reconstruct(fixture.records);

    assert_eq!(
        replay.outcome(),
        BlobRecoveryOutcome::ClosedResumeSessionPublishedGeneration
    );
    assert_eq!(
        replay.published_generation().object_id(),
        &fixture.object_id
    );
    assert_eq!(replay.resume_session().object_id(), &fixture.object_id);
    assert_eq!(
        replay.reachability_staging().object_id(),
        &fixture.object_id
    );
    assert_eq!(
        replay.placement_observation().object_id(),
        &fixture.object_id
    );
    let replay_counters = replay.counters();
    assert_eq!(
        (
            replay_counters.wal_records(),
            replay_counters.checkpoint_records(),
            replay_counters.manifest_rows(),
            replay_counters.replayed_outcomes(),
        ),
        (3, 2, 2, 1)
    );
}

#[test]
fn missing_prerequisites_produce_distinct_typed_denials() {
    let fixture = replay_fixture("phase7-denial");

    assert_eq!(
        BlobRecoveryRecordSet::from_admitted_replay_records(BlobAdmittedRecoveryRecords::new())
            .expect_err("chunk bytes alone must deny before chunk append admission")
            .kind(),
        BlobRecoveryRecordDenialKind::ChunkBytesWithoutIntegrityAdmission
    );
    assert_eq!(
        BlobRecoveryRecordSet::from_admitted_replay_records(
            BlobAdmittedRecoveryRecords::new().with_chunk_append(fixture.chunk.clone()),
        )
        .expect_err("integrity without frontier must deny")
        .kind(),
        BlobRecoveryRecordDenialKind::IntegrityWithoutCheckpointFrontier
    );
    assert_eq!(
        BlobRecoveryRecordSet::from_admitted_replay_records(
            BlobAdmittedRecoveryRecords::new()
                .with_chunk_append(fixture.chunk.clone())
                .with_checkpoint_frontier(fixture.frontier.clone()),
        )
        .expect_err("frontier without root candidate must deny")
        .kind(),
        BlobRecoveryRecordDenialKind::CheckpointFrontierWithoutRootCandidate
    );
    assert_eq!(
        BlobRecoveryRecordSet::from_admitted_replay_records(
            BlobAdmittedRecoveryRecords::new()
                .with_chunk_append(fixture.chunk.clone())
                .with_checkpoint_frontier(fixture.frontier.clone())
                .with_root_candidate(fixture.root.clone()),
        )
        .expect_err("root candidate without publication must deny")
        .kind(),
        BlobRecoveryRecordDenialKind::RootCandidateWithoutPublication
    );
    assert_eq!(
        BlobRecoveryRecordSet::from_admitted_replay_records(
            BlobAdmittedRecoveryRecords::new()
                .with_chunk_append(fixture.chunk.clone())
                .with_checkpoint_frontier(fixture.frontier.clone())
                .with_root_candidate(fixture.root.clone())
                .with_publication(fixture.publication.clone()),
        )
        .expect_err("publication without closed resume session must deny")
        .kind(),
        BlobRecoveryRecordDenialKind::PublicationWithoutClosedResumeSession
    );
    assert_eq!(
        BlobRecoveryRecordSet::from_admitted_replay_records(
            BlobAdmittedRecoveryRecords::new()
                .with_chunk_append(fixture.chunk.clone())
                .with_checkpoint_frontier(fixture.frontier.clone())
                .with_root_candidate(fixture.root.clone())
                .with_publication(fixture.publication.clone())
                .with_resume_session(fixture.resume.clone()),
        )
        .expect_err("publication without manifest agreement must deny")
        .kind(),
        BlobRecoveryRecordDenialKind::PublicationWithoutManifestAgreement
    );
}

#[test]
fn manifest_agreement_denies_wrong_physical_validation_digest() {
    let fixture = replay_fixture("phase7-wrong-manifest-validation");
    let manifest_source = manifest_source("phase7-wrong-manifest-validation");
    let reachability_row =
        BlobReachabilityManifestRow::from_root_candidate(&fixture.root, manifest_source.clone())
            .expect("reachability row should admit");
    let placement_row =
        BlobPlacementManifestRow::from_replayed_publication(&fixture.publication, manifest_source)
            .expect("placement row should admit");
    let generation = fixture.publication.published().generation().sequence();
    let wrong_validation = BlobPhysicalManifestValidation::validate_observation(
        BlobPhysicalManifestObservation::for_certification_test_authority(
            "phase7-other-manifest",
            generation,
            "phase7-other-manifest",
            generation,
            fixture
                .publication
                .published()
                .security_metadata()
                .identity(),
            true,
        )
        .expect("wrong manifest observation should still be physically valid"),
    )
    .expect("physical validation is valid for the wrong manifest");

    let denial = BlobManifestAgreement::validate(reachability_row, placement_row, wrong_validation)
        .expect_err("physical validation from another manifest must not agree");
    assert_eq!(
        denial.kind(),
        BlobRecoveryRecordDenialKind::PublicationWithoutManifestAgreement
    );
}

struct ReplayFixture {
    records: BlobRecoveryRecordSet,
    object_id: crate::BlobObjectId,
    chunk: BlobChunkAppendRecord,
    frontier: BlobCheckpointFrontierRecord,
    root: BlobRootCandidateRecord,
    publication: BlobGenerationPublicationRecord,
    resume: BlobResumeSessionCheckpointRecord,
}

fn replay_fixture(case: &str) -> ReplayFixture {
    let (candidate, reachability, resumability) = publication_inputs(case);
    let logical = candidate.intent().logical_content_digest().clone();
    let staged = BlobReachabilityStaging::stage(candidate.clone(), reachability)
        .expect("stage reachability");
    let payload = BlobPublicationWalPayload::from_staged_reachability(&staged);
    let durable = durable_wal_publication(payload.frame_digest());
    let classification = replayable_wal_classification(payload.frame_digest());
    let wal_commit = BlobPublicationWalCommit::from_replayable_wal_record(
        staged.clone(),
        payload.clone(),
        durable.clone(),
        &classification,
    )
    .expect("publication wal commit should admit");
    let wal_record = BlobPublicationWalRecord::append(wal_commit);
    let object_id = candidate.intent().object_id().clone();
    let wal_source = BlobReplaySourceAdmission::from_replayable_wal_classification(&classification)
        .expect("wal replay source should admit");
    let checkpoint_source = checkpoint_source(case);
    let manifest_source = manifest_source(case);
    let manifest_digest = manifest_source.source_digest().to_owned();
    let chunk = BlobChunkAppendRecord::from_integrity_admission(logical, wal_source)
        .expect("integrity-admitted chunk should produce append record");
    let frontier =
        BlobCheckpointFrontierRecord::from_chunk_append(chunk.clone(), checkpoint_source.clone())
            .expect("checkpoint frontier should admit chunk append");
    let root =
        BlobRootCandidateRecord::from_checkpoint_frontier(frontier.clone(), candidate.clone())
            .expect("root candidate should bind frontier");
    let publication = BlobGenerationPublicationRecord::from_replayed_wal_record(
        root.clone(),
        wal_record,
        BlobReplaySourceAdmission::from_replayable_wal_classification(&classification)
            .expect("publication wal source should admit"),
    )
    .expect("replayed wal publication should admit");
    let resume = BlobResumeSessionCheckpointRecord::from_replayed_publication(
        &publication,
        resumability,
        checkpoint_source,
    )
    .expect("resume checkpoint should admit");
    let reachability_row =
        BlobReachabilityManifestRow::from_root_candidate(&root, manifest_source.clone())
            .expect("reachability row should admit");
    let placement_row =
        BlobPlacementManifestRow::from_replayed_publication(&publication, manifest_source)
            .expect("placement row should admit");
    let validation = BlobPhysicalManifestValidation::validate_observation(
        BlobPhysicalManifestObservation::for_certification_test_authority(
            manifest_digest.clone(),
            publication.published().generation().sequence(),
            manifest_digest,
            publication.published().generation().sequence(),
            publication.published().security_metadata().identity(),
            true,
        )
        .expect("backend manifest observation should admit"),
    )
    .expect("physical manifest should validate");
    let manifest = BlobManifestAgreement::validate(reachability_row, placement_row, validation)
        .expect("manifest agrees");
    let records = BlobRecoveryRecordSet::from_admitted_replay_records(
        BlobAdmittedRecoveryRecords::new()
            .with_chunk_append(chunk.clone())
            .with_checkpoint_frontier(frontier.clone())
            .with_root_candidate(root.clone())
            .with_publication(publication.clone())
            .with_resume_session(resume.clone())
            .with_manifest(manifest.clone()),
    )
    .unwrap();
    ReplayFixture {
        records,
        object_id,
        chunk,
        frontier,
        root,
        publication,
        resume,
    }
}

fn checkpoint_source(case: &str) -> BlobReplaySourceAdmission {
    let publication = DurableCheckpointPublication::publish(durable_publication_receipt(
        StoreDurabilityPublicationKind::Checkpoint,
        format!("{case}.checkpoint"),
    ))
    .expect("durable checkpoint publication should admit");
    BlobReplaySourceAdmission::from_durable_checkpoint_publication(&publication)
        .expect("checkpoint replay source should admit")
}

fn manifest_source(case: &str) -> BlobReplaySourceAdmission {
    let publication = DurableManifestPublication::publish(durable_publication_receipt(
        StoreDurabilityPublicationKind::Manifest,
        format!("{case}.manifest"),
    ))
    .expect("durable manifest publication should admit");
    BlobReplaySourceAdmission::from_durable_manifest_publication(&publication)
        .expect("manifest replay source should admit")
}

fn durable_publication_receipt(
    kind: StoreDurabilityPublicationKind,
    digest: String,
) -> forge_store_physical_backend::StoreDurabilityOrderingBarrierDurable<
    CheckpointDurablePublicationScope,
> {
    let requirement = match kind {
        StoreDurabilityPublicationKind::Checkpoint => {
            StoreDurabilityRequirement::checkpoint_publication(required_durability_barriers())
        }
        StoreDurabilityPublicationKind::Manifest => {
            StoreDurabilityRequirement::manifest_publication(required_durability_barriers())
        }
        StoreDurabilityPublicationKind::WalFrame => {
            unreachable!("phase7 uses checkpoint manifests")
        }
    };
    let admission = StoreDurabilityAdmission::admit(requirement, &durability_witness())
        .expect("durability admission should admit");
    let scope =
        CheckpointDurablePublicationScope::new(StoreCheckpointRecordIdentity::new(7), digest, 1, 2)
            .expect("checkpoint publication scope should admit");
    let accepted = admission.submit_write(scope.clone()).backend_accepted();
    let observation = StoreDurabilityExecutionObservation::new(
        accepted.requirement().required_barriers(),
        StoreDurabilityFileSyncKind::Fsync,
    )
    .with_directory_sync_completed()
    .with_rename_completed()
    .with_ordering_barrier_completed();
    let mut backend = ManifestDurabilityBackend {
        expected_scope: scope,
        expected_requirement: requirement,
        expected_publication: kind,
        observation,
    };
    let proof = StoreDurabilityExecutionSession::for_store_backend(
        &mut backend,
        StoreOwnedDurabilityExecution::for_certification_test_authority(),
    )
    .execute(&accepted)
    .expect("durability execution should succeed");
    accepted
        .reach_durability_boundary(proof)
        .expect("durability boundary should admit")
        .parent_namespace_durable()
        .expect("parent namespace should be durable")
        .rename_durable()
        .expect("rename should be durable")
        .ordering_barrier_durable()
        .expect("ordering barrier should be durable")
}

fn required_durability_barriers() -> WalDurabilityBarrierSet {
    WalDurabilityBarrierSet::of(WalDurabilityBarrier::WalFileFsync)
        .insert(WalDurabilityBarrier::WalDirectoryFsync)
}

fn durability_witness() -> forge_store_physical_backend::AdmittedBackendCapabilityWitness {
    PhysicalBackendCapabilityAdmissionAuthority::store_owned()
        .admit_backend_capability(BackendCapabilityAdmissionRequest::new(
            BackendTargetProfile::PosixFileFsyncDirSync,
            BackendCapabilityEvidenceBasis::certified_backend_profile(),
            BackendCapabilitySupportSet::all_supported(),
            BackendMediaAssumptionSet::platform_file_defaults()
                .with_direct_io_alignment()
                .with_sector_atomicity()
                .with_page_cache_policy()
                .with_flush_ordering()
                .with_fdatasync_durability(),
            BackendRebindTriggers::kernel_filesystem_mount_firmware_and_backend(),
        ))
        .expect("backend capability should admit")
}

struct ManifestDurabilityBackend {
    expected_scope: CheckpointDurablePublicationScope,
    expected_requirement: StoreDurabilityRequirement,
    expected_publication: StoreDurabilityPublicationKind,
    observation: StoreDurabilityExecutionObservation,
}

impl PhysicalStoreDurabilityExecutor<CheckpointDurablePublicationScope>
    for ManifestDurabilityBackend
{
    type Error = ();

    fn execute_durability(
        &mut self,
        request: StoreDurabilityExecutionRequest<CheckpointDurablePublicationScope>,
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
