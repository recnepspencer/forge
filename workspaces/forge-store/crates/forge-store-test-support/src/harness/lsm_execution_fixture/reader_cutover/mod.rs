mod publication;
mod replay;

use super::*;

struct ReaderCutoverWorld {
    access: forge_store_layout_indexes::LsmStrategy,
    physical_plan: forge_store_physical_isolation::CompactionReadInterlockPlan,
    physical_manifest_epoch: u64,
    physical_publication: forge_store_physical_isolation::CompactionRewritePublication,
    physical_recovery: forge_store_recovery_physics::CompactionCutoverRecoveryPosture,
    pre_cutover_read: forge_store_physical_isolation::StablePhysicalReadReceipt,
    post_cutover_read: forge_store_physical_isolation::StablePhysicalReadReceipt,
    physical_intent: LsmPhysicalCompactionIntent,
    catalog: forge_store_layout_indexes::BootstrapCatalogReadAdmission,
    security: forge_store_security::StoreAdmittedSecurityScope,
    compaction: forge_store_layout_indexes::BaselineLsmCompactionAdmission,
    publication: forge_store_layout_indexes::BaselineLsmRunPublicationAdmission,
    wrong_publication: forge_store_layout_indexes::BaselineLsmRunPublicationAdmission,
    key: LsmMembershipKey,
    first_durable: AdmittedWalAppendReceipt,
    persisted: forge_store_lsm_authority::LsmMembershipSession,
    plan: forge_store_layout_indexes::BaselineLsmCompactionPlan,
}

pub fn execute_lsm_compaction_reader_cutover_fixture() -> ExecutedLsmCompactionFixture {
    let world = build_reader_cutover_world();
    let replay_source = replay::adjudicate_replay_sources(&world);
    let (published, reader_cutover) = publication::execute_compaction_publication(world);
    ExecutedLsmCompactionFixture {
        published,
        reader_cutover,
        replay_source,
    }
}

fn build_reader_cutover_world() -> ReaderCutoverWorld {
    begin_durability_fixture();
    let access = lsm_strategy();
    let physical_plan = crate::harness::physical_isolation::compaction::admitted_compaction_plan();
    let physical_manifest_epoch = physical_plan.protected().root().manifest_epoch().get() + 1;
    let (physical_publication, physical_recovery, pre_cutover_read, post_cutover_read) =
        crate::harness::physical_isolation::compaction::execute_compaction_cutover_for_manifest(
            &physical_plan,
            physical_manifest_epoch,
        )
        .into_parts();
    let physical_intent = LsmPhysicalCompactionIntent::from_interlock_plan(
        physical_plan.clone(),
        physical_manifest_epoch,
    )
    .expect("physical compaction intent retains the admitted interlock plan");
    let catalog = super::super::layout::admitted_layout_bootstrap_catalog();
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
        .expect("ordinary layout planning admits exact LSM compaction");
    let publication = admit_publication(&security, 43);
    let wrong_publication = admit_publication(&security, 99);
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
    let persisted = open_lsm_index(&first_durable)
        .expect("reopen re-admits membership from persisted WAL artifacts");
    let plan = access
        .lower_compaction(&persisted, key, compaction.clone())
        .expect("persisted WAL membership lowers to one compaction plan");
    ReaderCutoverWorld {
        access,
        physical_plan,
        physical_manifest_epoch,
        physical_publication,
        physical_recovery,
        pre_cutover_read,
        post_cutover_read,
        physical_intent,
        catalog,
        security,
        compaction,
        publication,
        wrong_publication,
        key,
        first_durable,
        persisted,
        plan,
    }
}

fn admit_publication(
    security: &forge_store_security::StoreAdmittedSecurityScope,
    record_identity: u64,
) -> forge_store_layout_indexes::BaselineLsmRunPublicationAdmission {
    layout_lsm_maintenance()
        .admit_run_publication(LsmRunPublicationAdmissionRequest::new(
            security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(record_identity),
            PreExecutionBudgetEnvelope::maintenance_default(),
        ))
        .into_result()
        .expect("ordinary layout planning admits exact LSM publication")
}
