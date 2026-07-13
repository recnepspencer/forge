mod durability;
mod hostile_replay;
mod repeated_compaction;
#[cfg(test)]
mod source_binding;

pub use hostile_replay::{execute_lsm_replay_hostile_matrix, LsmReplayHostileMatrix};

pub use repeated_compaction::{
    execute_repeated_lsm_membership_fixture, substituted_lsm_base_is_rejected_before_compaction,
    RepeatedLsmMembershipFixture,
};

#[cfg(test)]
use durability::durable_record_binding_for_store;
use durability::{
    begin_durability_fixture, durable_record, durable_record_binding, manifest_receipt,
    manifest_receipt_for_artifact, wal_receipt, wal_scope,
};
use forge_store_security::{
    admitted_store_wal_checkpoint_security_scope_for_layout_partition_test, StoreKeyVersionPosture,
    StoreLegacySecurityPosture,
};

use forge_store_budgets::PreExecutionBudgetEnvelope;
use forge_store_contracts::WalRecordFamily;
use forge_store_layout_indexes::{
    layout_lsm_maintenance, lsm_compaction_runtime, lsm_physical_compaction_runtime,
    lsm_publication_runtime, lsm_replay_runtime, lsm_strategy, BaselineLsmExecutionAdmissionDenial,
    LsmCompactionAdmissionRequest, LsmPhysicalCompactionIntent, LsmReplayAdmissionRequest,
    LsmRunPublicationAdmissionRequest, PublishedLsmCompaction,
};
use forge_store_lsm_authority::{
    LsmMembershipArtifactDeclaration, LsmMembershipKey, LsmReplaySourceDenial, LsmReplaySourceKind,
};
use forge_store_recovery_physics::{
    PartialPublicationClassification, PartialPublicationCrashEdge, PartialPublicationEvidence,
    TornPublicationDenial,
};
use forge_store_wal::{
    admit_checkpoint_publication, admit_durable_append, AdmittedWalAppendReceipt,
    BlobWalRecordIdentity, BlobWalRecordKind, CheckpointDurablePublicationScope,
    StoreCheckpointRecordIdentity, StoreWalRecordIdentity,
};

#[cfg(test)]
use forge_store_layout_indexes::access_planning;
#[cfg(test)]
use forge_store_layout_indexes::declarations::{layout_declarations, ArtifactFamilyAccessLane};
#[cfg(test)]
use forge_store_layout_indexes::maintenance::{
    layout_maintenance, ExactPublicationAuthoritySource, IndexMaintenanceMode,
    IndexPublicationProtocol, LiveMaintenanceRequest, PhysicalMutationShape,
};
#[cfg(test)]
use forge_store_layout_indexes::strategy_declarations::LayoutStrategyFamily;
#[cfg(test)]
use forge_store_layout_indexes::LsmStrategy;
#[cfg(test)]
use forge_store_lsm_authority::LsmMembershipRecord;
#[cfg(test)]
use forge_store_wal::{BlobWalRecordEnvelope, DurablePublicationDeclaration};

#[derive(Debug, Clone)]
pub struct LsmMembershipReplacementCrashFixture {
    anchor: AdmittedWalAppendReceipt,
    key: LsmMembershipKey,
    activation_path: std::path::PathBuf,
    activation_bytes: Vec<u8>,
    replacement_output: BlobWalRecordIdentity,
    replacement_path: std::path::PathBuf,
    wrong_physical_denial: forge_store_lsm_authority::LsmMembershipDenial,
}

#[derive(Debug, Clone)]
pub struct ExecutedLsmCompactionFixture {
    published: PublishedLsmCompaction,
    reader_cutover: forge_store_physical_isolation::ReadDuringCompactionVerdict,
}

pub(super) fn open_lsm_index(
    anchor: &AdmittedWalAppendReceipt,
) -> Result<forge_store_lsm_authority::LsmMembershipSession, BaselineLsmExecutionAdmissionDenial> {
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    lsm_strategy().open_index(anchor, security.witnesses())
}

impl ExecutedLsmCompactionFixture {
    pub const fn published(&self) -> &PublishedLsmCompaction {
        &self.published
    }

    pub const fn reader_cutover(
        &self,
    ) -> &forge_store_physical_isolation::ReadDuringCompactionVerdict {
        &self.reader_cutover
    }

    pub fn into_published(self) -> PublishedLsmCompaction {
        self.published
    }
}

impl LsmMembershipReplacementCrashFixture {
    pub const fn anchor(&self) -> &AdmittedWalAppendReceipt {
        &self.anchor
    }

    pub const fn key(&self) -> LsmMembershipKey {
        self.key
    }

    pub fn activation_path(&self) -> &std::path::Path {
        &self.activation_path
    }

    pub fn activation_bytes(&self) -> &[u8] {
        &self.activation_bytes
    }

    pub const fn replacement_output(&self) -> BlobWalRecordIdentity {
        self.replacement_output
    }

    pub fn replacement_path(&self) -> &std::path::Path {
        &self.replacement_path
    }

    pub const fn wrong_physical_denial(&self) -> forge_store_lsm_authority::LsmMembershipDenial {
        self.wrong_physical_denial
    }
}

pub fn lsm_membership_replacement_crash_fixture() -> LsmMembershipReplacementCrashFixture {
    begin_durability_fixture();
    let access = lsm_strategy();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let metadata = forge_store_wal::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let compaction = layout_lsm_maintenance()
        .admit_compaction(LsmCompactionAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .expect("ordinary retire crash fixture planning");
    let key = access.admit_key(metadata, compaction).unwrap();
    let (first_envelope, anchor) = durable_record_binding(key, 41, BlobWalRecordKind::LsmValue);
    let mut session = open_lsm_index(&anchor).unwrap();
    access
        .persist_record(&mut session, first_envelope, &anchor, key)
        .unwrap();
    durable_record(
        &access,
        &mut session,
        key,
        42,
        BlobWalRecordKind::GenerationPublication,
    );
    durable_record(
        &access,
        &mut session,
        key,
        43,
        BlobWalRecordKind::LsmTombstone,
    );
    let selected = session.select_compaction(key).unwrap();
    let (physical_intent, physical_publication) = physical_compaction_fixture();
    let output_scope = wal_scope(
        selected.expected_output_identity().unwrap().sequence(),
        selected.compaction_output_digest(
            physical_intent.root_scope(),
            physical_intent.target_epoch(),
            physical_intent.manifest_epoch(),
        ),
        4096,
    );
    let output_artifact = LsmMembershipArtifactDeclaration::compaction_output(&output_scope);
    let output_durable =
        admit_durable_append(&wal_receipt(output_scope, output_artifact.bytes())).unwrap();
    let wrong_physical =
        forge_store_physical_isolation::compaction_cutover_evidence_for_certification_rewrite_manifest(
            physical_intent.plan(),
            physical_intent.manifest_epoch() + 1,
        )
        .into_parts()
        .0;
    let output = forge_store_lsm_authority::admit_lsm_replacement_output(
        &selected,
        output_durable,
        physical_intent,
    )
    .unwrap();
    let replacement_output = output.identity();
    let replacement_path = output.persisted_path().to_path_buf();
    let wrong_physical_denial = forge_store_lsm_authority::prepare_lsm_membership_activation(
        &selected,
        output.clone(),
        &wrong_physical,
    )
    .unwrap_err();
    let activation = forge_store_lsm_authority::prepare_lsm_membership_activation(
        &selected,
        output,
        &physical_publication,
    )
    .unwrap();
    let artifact = activation.artifact();
    let checkpoint = admit_checkpoint_publication(&manifest_receipt_for_artifact(
        activation.scope().clone(),
        artifact.bytes(),
    ))
    .unwrap();
    let activation_path = checkpoint.persisted_path().to_path_buf();
    let activation_bytes = std::fs::read(&activation_path).unwrap();
    let publication = forge_store_lsm_authority::admit_lsm_membership_replacement(
        &selected, activation, checkpoint,
    )
    .unwrap();
    session.replace(&selected, &publication).unwrap();
    LsmMembershipReplacementCrashFixture {
        anchor,
        key,
        activation_path,
        activation_bytes,
        replacement_output,
        replacement_path,
        wrong_physical_denial,
    }
}

pub(super) fn physical_compaction_fixture() -> (
    LsmPhysicalCompactionIntent,
    forge_store_physical_isolation::CompactionRewritePublication,
) {
    let plan =
        forge_store_physical_isolation::compaction_read_interlock_plan_for_certification_test();
    let manifest_epoch = plan.protected().root().manifest_epoch().get() + 1;
    let (publication, _, _, _) =
        forge_store_physical_isolation::compaction_cutover_evidence_for_certification_rewrite_manifest(
            &plan,
            manifest_epoch,
        )
        .into_parts();
    let intent = LsmPhysicalCompactionIntent::from_interlock_plan(plan, manifest_epoch).unwrap();
    (intent, publication)
}

/// Certification drives the same durability and WAL facades as production.
/// It supplies observations to the backend boundary but cannot construct WAL receipts.
pub fn execute_baseline_lsm_persisted_fixture() -> PublishedLsmCompaction {
    execute_lsm_compaction_reader_cutover_fixture().into_published()
}

pub fn execute_baseline_lsm_membership_replacement_fixture(
) -> forge_store_lsm_authority::PublishedLsmMembershipReplacement {
    execute_baseline_lsm_persisted_fixture()
        .membership_replacement()
        .clone()
}

#[cfg(test)]
#[test]
fn published_lsm_manifest_admits_exact_layout_maintenance_end_to_end() {
    let published = execute_baseline_lsm_persisted_fixture();
    let execution = published.publication_execution();
    let authority = ExactPublicationAuthoritySource::installed_lsm_manifest(&execution);
    let catalog = super::layout::admitted_layout_bootstrap_catalog();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let declarations = layout_declarations();
    let declaration = declarations
        .declaration(forge_store_contracts::DurableArtifactFamilyId::PublicationWalIntent)
        .expect("publication WAL intent is a declared layout family");
    let family = declarations
        .admit_physical_artifact_family(declaration, security.witnesses())
        .expect("current Store security admits the publication family");
    let key_domain = declarations
        .admit_physical_key_domain(family, security.witnesses())
        .expect("publication family admits its physical key domain");
    let materialization = access_planning()
        .admit_lsm_publication_materialization(family, &catalog, &execution)
        .expect("owner-issued manifest publication admits exact LSM materialization");
    let request = LiveMaintenanceRequest::new(
        family,
        key_domain,
        LayoutStrategyFamily::BaselineLsmWriteOptimized,
        ArtifactFamilyAccessLane::HotPath,
        IndexMaintenanceMode::SynchronousExact,
        PhysicalMutationShape::ObservationOnly,
        IndexPublicationProtocol::StableManifestInstall,
    )
    .with_exact_publication_authority(authority)
    .with_exact_coverage(materialization.coverage().clone());
    let plan = layout_maintenance()
        .admit_mutation(request)
        .into_exact()
        .expect("matching manifest publication must admit exact maintenance");
    let lowered = layout_maintenance().lower_exact(plan);

    assert!(layout_maintenance().certify_live_exact(&lowered).is_some());
}

pub fn execute_lsm_compaction_reader_cutover_fixture() -> ExecutedLsmCompactionFixture {
    begin_durability_fixture();
    let access = lsm_strategy();
    let physical_plan =
        forge_store_physical_isolation::compaction_read_interlock_plan_for_certification_test();
    let physical_manifest_epoch = physical_plan.protected().root().manifest_epoch().get() + 1;
    let (physical_publication, physical_recovery, pre_cutover_read, post_cutover_read) =
        forge_store_physical_isolation::compaction_cutover_evidence_for_certification_rewrite_manifest(
            &physical_plan,
            physical_manifest_epoch,
        )
        .into_parts();
    let physical_intent = LsmPhysicalCompactionIntent::from_interlock_plan(
        physical_plan.clone(),
        physical_manifest_epoch,
    )
    .expect("physical compaction intent retains the admitted interlock plan");
    let catalog = super::layout::admitted_layout_bootstrap_catalog();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let metadata = forge_store_wal::WalSecurityMetadataCarrier::for_wal_record(
        security.witnesses(),
        StoreKeyVersionPosture::Current,
        StoreLegacySecurityPosture::NativeScoped,
    );
    let compaction = layout_lsm_maintenance()
        .admit_compaction(LsmCompactionAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .expect("ordinary layout planning admits exact LSM compaction");
    let publication = layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .expect("ordinary layout planning admits exact LSM publication");
    let wrong_publication = layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(99),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .expect("hostile publication remains a well-formed different operation");
    let key = access
        .admit_key(metadata, compaction.clone())
        .expect("security-scoped canonical key");
    let (first_envelope, first_durable) =
        durable_record_binding(key, 41, BlobWalRecordKind::LsmValue);
    let mut persisted =
        open_lsm_index(&first_durable).expect("WAL-owned persistent membership index");
    let first_record = access
        .persist_record(&mut persisted, first_envelope, &first_durable, key)
        .expect("first record binds the index to its WAL store");
    let _persisted_records = [
        first_record,
        durable_record(
            &access,
            &mut persisted,
            key,
            42,
            BlobWalRecordKind::GenerationPublication,
        ),
        durable_record(
            &access,
            &mut persisted,
            key,
            43,
            BlobWalRecordKind::LsmTombstone,
        ),
    ];
    drop(persisted);
    let mut persisted = open_lsm_index(&first_durable)
        .expect("reopen re-admits membership from persisted WAL artifacts");
    let plan = access
        .lower_compaction(&persisted, key, compaction.clone())
        .expect("persisted WAL membership lowers to one compaction plan");
    let wal_replay_source = access
        .admit_replay_source(&plan, None, None)
        .expect("recovery owner admits the persisted WAL replay source");
    assert_eq!(
        wal_replay_source.selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
    let replay = layout_lsm_maintenance()
        .admit_replay(LsmReplayAdmissionRequest::new(
            &catalog,
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            &wal_replay_source,
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .expect("recovery-owned WAL source admits exact LSM replay");
    assert_eq!(
        layout_lsm_maintenance().admit_replay(LsmReplayAdmissionRequest::new(
            &catalog,
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(99),
            &wal_replay_source,
            PreExecutionBudgetEnvelope::maintenance_default(),
        )),
        Err(forge_store_layout_indexes::LsmMaintenanceAdmissionDenied::UnexpectedSelectedOperation),
        "a replay source cannot authorize a different canonical WAL operation",
    );
    let partial_checkpoint_scope = plan
        .manifest_scope(StoreCheckpointRecordIdentity::new(2), 42, 45)
        .expect("hostile partial checkpoint scope");
    let partial_checkpoint =
        admit_checkpoint_publication(&manifest_receipt(partial_checkpoint_scope))
            .expect("hostile checkpoint remains durably published");
    assert_eq!(
        access.admit_replay_source(&plan, Some(&partial_checkpoint), None),
        Err(LsmReplaySourceDenial::CheckpointDoesNotCoverMembership),
        "a newer checkpoint that omits the first membership record cannot become replay authority",
    );
    let stale_checkpoint_scope = plan
        .manifest_scope(StoreCheckpointRecordIdentity::new(4), 40, 42)
        .expect("stale checkpoint scope remains well formed");
    let stale_checkpoint =
        admit_checkpoint_publication(&manifest_receipt(stale_checkpoint_scope)).unwrap();
    assert_eq!(
        access
            .admit_replay_source(&plan, Some(&stale_checkpoint), None)
            .unwrap()
            .selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
    let output_digest = plan.output_frame_digest(&physical_intent);
    let output_scope = wal_scope(44, output_digest, 4096);
    let output_artifact = LsmMembershipArtifactDeclaration::compaction_output(&output_scope);
    let output = admit_durable_append(&wal_receipt(output_scope, output_artifact.bytes()))
        .expect("executed output durability");
    let manifest_scope = plan
        .manifest_scope(StoreCheckpointRecordIdentity::new(1), 41, 45)
        .expect("manifest coverage");
    let manifest = admit_checkpoint_publication(&manifest_receipt(manifest_scope))
        .expect("executed manifest durability");
    let checkpoint_replay_source = access
        .admit_replay_source(&plan, Some(&manifest), None)
        .expect("recovery owner admits the fully covering checkpoint source");
    assert_eq!(
        checkpoint_replay_source.selected_source(),
        LsmReplaySourceKind::Checkpoint,
    );
    let checkpoint_replay = layout_lsm_maintenance()
        .admit_replay(LsmReplayAdmissionRequest::new(
            &catalog,
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            &checkpoint_replay_source,
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .expect("recovery-owned checkpoint source admits exact LSM replay");
    let wrong_digest_scope = CheckpointDurablePublicationScope::new(
        StoreCheckpointRecordIdentity::new(3),
        "copied-unrelated-lsm-manifest",
        41,
        45,
    )
    .unwrap();
    let wrong_digest_checkpoint =
        admit_checkpoint_publication(&manifest_receipt(wrong_digest_scope)).unwrap();
    assert_eq!(
        access.admit_replay_source(&plan, Some(&wrong_digest_checkpoint), None),
        Err(LsmReplaySourceDenial::CheckpointDoesNotBindMembership),
    );
    let ambiguous = PartialPublicationClassification::classify(
        PartialPublicationEvidence::insufficient_persisted_evidence(
            "lsm-checkpoint-cutover-ambiguous",
        ),
    );
    assert_eq!(
        access.admit_replay_source(&plan, Some(&manifest), Some(&ambiguous)),
        Err(LsmReplaySourceDenial::PartialPublicationAmbiguous),
    );
    let before_wal = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_persisted_crash_edge(
            PartialPublicationCrashEdge::before_wal_append("lsm-replacement"),
        ),
    );
    assert_eq!(
        access
            .admit_replay_source(&plan, Some(&manifest), Some(&before_wal))
            .unwrap()
            .selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
    let log_only = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_log_only("copied-lsm-log-claim"),
    );
    assert_eq!(
        access
            .admit_replay_source(&plan, Some(&manifest), Some(&log_only))
            .unwrap()
            .selected_source(),
        LsmReplaySourceKind::WalFrame,
    );
    let torn = PartialPublicationClassification::classify(
        PartialPublicationEvidence::from_torn_publication(TornPublicationDenial::new(
            None,
            "torn LSM replacement publication",
        )),
    );
    assert_eq!(
        access.admit_replay_source(&plan, Some(&manifest), Some(&torn)),
        Err(LsmReplaySourceDenial::TornPublication),
    );
    let wal_replay_execution = lsm_replay_runtime()
        .execute(replay)
        .expect("WAL-selected replay executes the durable tail");
    assert_eq!(wal_replay_execution.replayable_count(), 3);
    assert_eq!(wal_replay_execution.stale_run_count(), 0);
    assert_eq!(wal_replay_execution.cleanup_batch_count(), 0);
    assert_eq!(wal_replay_execution.remaining_run_count(), 3);
    assert_eq!(wal_replay_execution.counters().wal_replays(), 3);
    assert_eq!(wal_replay_execution.counters().maintenance_reads(), 0);
    let checkpoint_replay_execution = lsm_replay_runtime()
        .execute(checkpoint_replay)
        .expect("selected replay executes only from recovery-owned source");
    assert_eq!(checkpoint_replay_execution.replayable_count(), 0);
    assert_eq!(checkpoint_replay_execution.stale_run_count(), 3);
    assert_eq!(checkpoint_replay_execution.cleanup_batch_count(), 1);
    assert_eq!(checkpoint_replay_execution.remaining_run_count(), 1);
    assert_eq!(checkpoint_replay_execution.counters().wal_replays(), 0);
    assert_eq!(
        checkpoint_replay_execution.counters().maintenance_reads(),
        1
    );
    let demand = access
        .admit_compaction_demand(plan.clone(), output.clone(), physical_intent.clone())
        .expect("durable output and exact physical horizon admit compaction demand");
    let stale_demand = access
        .admit_compaction_demand(plan, output, physical_intent)
        .expect("the same current persisted membership can be prepared concurrently");
    let prepared = lsm_compaction_runtime()
        .execute(demand)
        .expect("compaction produces durable but unpublished output");
    let stale_prepared = lsm_compaction_runtime()
        .execute(stale_demand)
        .expect("concurrent preparation does not retire membership");
    let (wrong_physical_publication, _, _, _) =
        forge_store_physical_isolation::compaction_cutover_evidence_for_certification_rewrite_manifest(
            &physical_plan,
            physical_manifest_epoch + 1,
        )
        .into_parts();
    let physical_denial = lsm_physical_compaction_runtime()
        .admit(prepared.clone(), wrong_physical_publication)
        .expect_err("a different physical rewrite cannot authorize semantic publication");
    assert_eq!(
        physical_denial,
        BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch,
    );
    let same_looking_plan =
        forge_store_physical_isolation::compaction_read_interlock_plan_for_certification_root_seed(
            17,
        );
    let (same_looking_publication, _, _, _) =
        forge_store_physical_isolation::compaction_cutover_evidence_for_certification_rewrite_manifest(
            &same_looking_plan,
            physical_manifest_epoch,
        )
        .into_parts();
    assert_eq!(
        lsm_physical_compaction_runtime()
            .admit(prepared.clone(), same_looking_publication)
            .unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch,
        "an independently admitted equal-looking reader horizon cannot publish this compaction",
    );
    let wrong_footprint_plan =
        forge_store_physical_isolation::compaction_read_interlock_plan_for_certification_root_seed(
            18,
        );
    let wrong_footprint_manifest = wrong_footprint_plan
        .protected()
        .root()
        .manifest_epoch()
        .get()
        + 1;
    let (wrong_footprint_publication, _, _, _) =
        forge_store_physical_isolation::compaction_cutover_evidence_for_certification_rewrite_manifest(
            &wrong_footprint_plan,
            wrong_footprint_manifest,
        )
        .into_parts();
    assert!(
        matches!(
            lsm_physical_compaction_runtime().admit(prepared.clone(), wrong_footprint_publication),
            Err(BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch)
        ),
        "a valid rewrite for a different protected footprint cannot publish this compaction"
    );
    let interlocked = lsm_physical_compaction_runtime()
        .admit(prepared, physical_publication.clone())
        .expect("physical isolation admits the exact prepared compaction");
    let stale_interlocked = lsm_physical_compaction_runtime()
        .admit(stale_prepared, physical_publication)
        .expect("concurrent semantic preparation binds the same executed physical rewrite");
    let activation = interlocked
        .prepare_membership_activation()
        .expect("executed physical publication prepares durable membership activation");
    let activation_artifact = activation.artifact();
    let activation_manifest = admit_checkpoint_publication(&manifest_receipt_for_artifact(
        activation.scope().clone(),
        activation_artifact.bytes(),
    ))
    .expect("post-physical membership activation is durably published");
    let manifest_path = activation_manifest.persisted_path().to_path_buf();
    let manifest_bytes = std::fs::read(&manifest_path).expect("persisted activation bytes");
    let wrong_publication_denial = lsm_publication_runtime()
        .publish(
            &mut persisted,
            wrong_publication,
            interlocked.clone(),
            activation.clone(),
            activation_manifest.clone(),
        )
        .expect_err("wrong selected publication must fail before membership retirement");
    assert_eq!(
        wrong_publication_denial,
        BaselineLsmExecutionAdmissionDenial::SelectedOperationKeyMismatch,
    );
    let published = lsm_publication_runtime()
        .publish(
            &mut persisted,
            publication.clone(),
            interlocked,
            activation.clone(),
            activation_manifest.clone(),
        )
        .expect("publication makes prepared compaction visible and retires old membership");
    let publication_counters = published.publication_execution().counters();
    assert_eq!(publication_counters.publications(), 2);
    assert_eq!(publication_counters.maintenance_reads(), 2);
    let compaction_counters = published.compaction_publication_receipt().counters();
    assert_eq!(compaction_counters.publications(), 1);
    assert_eq!(compaction_counters.maintenance_reads(), 3);
    let stale_denial = lsm_publication_runtime()
        .publish(
            &mut persisted,
            publication,
            stale_interlocked,
            activation,
            activation_manifest,
        )
        .expect_err("retirement must stale every concurrent prepared compaction");
    assert_eq!(
        stale_denial,
        BaselineLsmExecutionAdmissionDenial::PersistedMembershipStale,
    );
    drop(persisted);
    let reopened = open_lsm_index(&first_durable)
        .expect("retired membership reopens through the same validation as live retirement");
    assert_eq!(
        access.lower_compaction(&reopened, key, compaction.clone()),
        Err(BaselineLsmExecutionAdmissionDenial::PersistedMembershipIncomplete),
        "durably retired inputs must remain retired after reopen",
    );
    let mut substituted_manifest = manifest_bytes.clone();
    let last = substituted_manifest
        .last_mut()
        .expect("membership manifest is nonempty");
    *last ^= 0x01;
    std::fs::write(&manifest_path, &substituted_manifest)
        .expect("same-length hostile manifest substitution");
    let reopen_denial = open_lsm_index(&first_durable)
        .expect_err("reopen must reject a different same-length manifest");
    assert_eq!(
        reopen_denial,
        BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch,
    );
    std::fs::write(manifest_path, manifest_bytes).expect("restore persisted manifest fixture");
    let reader_cutover = published
        .observe_reader_cutover(physical_recovery, pre_cutover_read, post_cutover_read)
        .expect("semantic LSM publication preserves physical reader cutover authority");
    ExecutedLsmCompactionFixture {
        published,
        reader_cutover,
    }
}

#[cfg(test)]
mod artifact_binding_tests {
    use super::*;

    #[test]
    fn durable_scope_cannot_authorize_different_wal_bytes() {
        let access = lsm_strategy();
        let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
        let metadata = forge_store_wal::WalSecurityMetadataCarrier::for_wal_record(
            security.witnesses(),
            StoreKeyVersionPosture::Current,
            StoreLegacySecurityPosture::NativeScoped,
        );
        let compaction = layout_lsm_maintenance()
            .admit_compaction(LsmCompactionAdmissionRequest::new(
                security.witnesses(),
                WalRecordFamily::DurableMutationIntent,
                StoreWalRecordIdentity::new(99),
                PreExecutionBudgetEnvelope::maintenance_default(),
            ))
            .unwrap();
        let key = access.admit_key(metadata, compaction).unwrap();
        let scope = wal_scope(91, "claimed-frame".into(), 11);
        let receipt = admit_durable_append(&wal_receipt(scope.clone(), b"wrong-bytes")).unwrap();
        let envelope = BlobWalRecordEnvelope::new(
            BlobWalRecordIdentity::new(91, BlobWalRecordKind::LsmValue).unwrap(),
            DurablePublicationDeclaration::wal_frame(scope),
            "claimed-frame",
        )
        .unwrap();
        let mut index = open_lsm_index(&receipt).unwrap();
        assert_eq!(
            access.persist_record(&mut index, envelope, &receipt, key),
            Err(BaselineLsmExecutionAdmissionDenial::DurableRecordBindingMismatch)
        );
    }

    #[test]
    fn duplicate_active_component_is_denied_before_membership_admission() {
        let (access, key) = admitted_test_index(99);
        let (first_envelope, first) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
        let mut index = open_lsm_index(&first).unwrap();
        access
            .persist_record(&mut index, first_envelope, &first, key)
            .unwrap();
        let (duplicate_envelope, duplicate) =
            durable_record_binding(key, 92, BlobWalRecordKind::LsmValue);

        assert_eq!(
            access.persist_record(&mut index, duplicate_envelope, &duplicate, key),
            Err(BaselineLsmExecutionAdmissionDenial::PersistedMembershipAmbiguous)
        );
    }

    #[test]
    fn segment_or_generation_substitution_is_denied_before_membership_admission() {
        let (access, key) = admitted_test_index(99);
        let (_, anchor) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
        let mut index = open_lsm_index(&anchor).unwrap();
        let (foreign_envelope, foreign) = durable_record_binding_for_store(
            key,
            92,
            BlobWalRecordKind::GenerationPublication,
            2,
            7,
        );

        assert_eq!(
            access.persist_record(&mut index, foreign_envelope, &foreign, key),
            Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch)
        );
    }

    #[test]
    fn equal_scope_from_a_different_store_directory_is_denied() {
        begin_durability_fixture();
        let (access, key) = admitted_test_index(99);
        let (first_envelope, anchor) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
        let mut index = open_lsm_index(&anchor).unwrap();
        access
            .persist_record(&mut index, first_envelope, &anchor, key)
            .unwrap();

        begin_durability_fixture();
        let (foreign_envelope, foreign) =
            durable_record_binding(key, 92, BlobWalRecordKind::GenerationPublication);

        assert_eq!(
            access.persist_record(&mut index, foreign_envelope, &foreign, key),
            Err(BaselineLsmExecutionAdmissionDenial::RecordKeyScopeMismatch)
        );
    }

    #[test]
    fn artifact_swap_after_record_admission_is_denied_before_membership_admission() {
        let (_access, key) = admitted_test_index(99);
        let (envelope, durable) = durable_record_binding(key, 91, BlobWalRecordKind::LsmValue);
        let record = LsmMembershipRecord::admit(envelope, &durable, key).unwrap();
        let mut index = open_lsm_index(&durable).unwrap();
        std::fs::write(durable.persisted_path(), b"substituted-after-admission").unwrap();

        assert_eq!(
            index.persist(record),
            Err(forge_store_lsm_authority::LsmMembershipDenial::DurableRecordBindingMismatch)
        );
    }

    fn admitted_test_index(sequence: u64) -> (LsmStrategy, LsmMembershipKey) {
        let access = lsm_strategy();
        let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
        let metadata = forge_store_wal::WalSecurityMetadataCarrier::for_wal_record(
            security.witnesses(),
            StoreKeyVersionPosture::Current,
            StoreLegacySecurityPosture::NativeScoped,
        );
        let compaction = layout_lsm_maintenance()
            .admit_compaction(LsmCompactionAdmissionRequest::new(
                security.witnesses(),
                WalRecordFamily::DurableMutationIntent,
                StoreWalRecordIdentity::new(sequence),
                PreExecutionBudgetEnvelope::maintenance_default(),
            ))
            .unwrap();
        let key = access.admit_key(metadata, compaction).unwrap();
        (access, key)
    }
}
