pub(super) use super::durability::durable_record_binding_for_store;
pub(super) use super::durability::{
    begin_durability_fixture, durable_record, durable_record_binding, manifest_receipt,
    manifest_receipt_for_artifact, wal_receipt, wal_scope,
};
pub(super) use forge_store_security::{
    admitted_store_wal_checkpoint_security_scope_for_layout_partition_test, StoreKeyVersionPosture,
    StoreLegacySecurityPosture,
};

pub(super) use forge_store_budgets::PreExecutionBudgetEnvelope;
pub(super) use forge_store_contracts::WalRecordFamily;
pub(super) use forge_store_layout_indexes::{
    layout_lsm_maintenance, lsm_compaction_runtime, lsm_physical_compaction_runtime,
    lsm_publication_runtime, lsm_replay_runtime, lsm_strategy, BaselineLsmExecutionAdmissionDenial,
    LsmCompactionAdmissionRequest, LsmPhysicalCompactionIntent, LsmReplayAdmissionRequest,
    LsmRunPublicationAdmissionRequest, PublishedLsmCompaction,
};
pub(super) use forge_store_lsm_authority::{
    LsmMembershipArtifactDeclaration, LsmMembershipKey, LsmReplaySourceDenial, LsmReplaySourceKind,
};
pub(super) use forge_store_recovery_physics::{
    PartialPublicationClassification, PartialPublicationCrashEdge, PartialPublicationEvidence,
    TornPublicationDenial,
};
pub(super) use forge_store_wal::{
    admit_checkpoint_publication, admit_durable_append, AdmittedWalAppendReceipt,
    BlobWalRecordIdentity, BlobWalRecordKind, CheckpointDurablePublicationScope,
    StoreCheckpointRecordIdentity, StoreWalRecordIdentity,
};

#[cfg(test)]
pub(super) use forge_store_layout_indexes::access_planning;
#[cfg(test)]
pub(super) use forge_store_layout_indexes::declarations::layout_declarations;
#[cfg(test)]
pub(super) use forge_store_layout_indexes::LsmStrategy;
#[cfg(test)]
pub(super) use forge_store_lsm_authority::LsmMembershipRecord;
#[cfg(test)]
pub(super) use forge_store_wal::{BlobWalRecordEnvelope, DurablePublicationDeclaration};

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
    pub(super) published: PublishedLsmCompaction,
    pub(super) reader_cutover: forge_store_physical_isolation::ReadDuringCompactionVerdict,
    pub(super) replay_source: forge_store_lsm_authority::AdmittedLsmReplaySource,
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

    pub const fn replay_source(&self) -> &forge_store_lsm_authority::AdmittedLsmReplaySource {
        &self.replay_source
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
        .into_result()
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
    let selected = forge_store_lsm_authority::select_lsm_compaction_membership(&session, key)
        .into_result()
        .unwrap();
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
        crate::harness::physical_isolation::compaction::execute_compaction_cutover_for_manifest(
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
    forge_store_lsm_authority::replace_lsm_membership(&mut session, &selected, &publication)
        .into_result()
        .unwrap();
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
    let plan = crate::harness::physical_isolation::compaction::admitted_compaction_plan();
    let manifest_epoch = plan.protected().root().manifest_epoch().get() + 1;
    let (publication, _, _, _) =
        crate::harness::physical_isolation::compaction::execute_compaction_cutover_for_manifest(
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

pub fn execute_baseline_lsm_replay_source_fixture(
) -> forge_store_lsm_authority::AdmittedLsmReplaySource {
    execute_lsm_compaction_reader_cutover_fixture()
        .replay_source()
        .clone()
}

#[cfg(test)]
#[test]
fn published_lsm_manifest_materializes_its_frontier_without_claiming_live_exactness() {
    let published = execute_baseline_lsm_persisted_fixture();
    let execution = published.publication_execution();
    let catalog = super::super::layout::admitted_layout_bootstrap_catalog();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let declarations = layout_declarations();
    let declaration = declarations
        .declaration(forge_store_contracts::DurableArtifactFamilyId::PublicationWalIntent)
        .expect("publication WAL intent is a declared layout family");
    let family = declarations
        .admit_physical_artifact_family(declaration, security.witnesses())
        .into_result()
        .expect("current Store security admits the publication family");
    let materialization = access_planning()
        .admit_lsm_publication_materialization(family, &catalog, &execution)
        .expect("owner-issued manifest publication admits exact LSM materialization");
    assert!(materialization.coverage().is_exact());
    assert_eq!(
        execution.maintenance_mode(),
        forge_store_layout_indexes::IndexMaintenanceMode::AsynchronousLagged
    );
    assert_eq!(execution.counters().publications(), 2);
}
pub(super) use super::reader_cutover::execute_lsm_compaction_reader_cutover_fixture;
