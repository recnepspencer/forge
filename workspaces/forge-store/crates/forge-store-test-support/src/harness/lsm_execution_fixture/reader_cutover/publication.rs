use super::{ReaderCutoverWorld, *};

pub(super) fn execute_compaction_publication(
    mut world: ReaderCutoverWorld,
) -> (
    PublishedLsmCompaction,
    forge_store_physical_isolation::ReadDuringCompactionVerdict,
) {
    let output_digest = world.plan.output_frame_digest(&world.physical_intent);
    let output_scope = wal_scope(44, output_digest, 4096);
    let output_artifact = LsmMembershipArtifactDeclaration::compaction_output(&output_scope);
    let output = admit_durable_append(&wal_receipt(output_scope, output_artifact.bytes()))
        .expect("executed output durability");
    let demand = world
        .access
        .admit_compaction_demand(
            world.plan.clone(),
            output.clone(),
            world.physical_intent.clone(),
        )
        .expect("durable output and exact physical horizon admit compaction demand");
    let stale_demand = world
        .access
        .admit_compaction_demand(world.plan.clone(), output, world.physical_intent.clone())
        .expect("the same current persisted membership can be prepared concurrently");
    let prepared = lsm_compaction_runtime()
        .execute(demand)
        .into_result()
        .expect("compaction produces durable but unpublished output");
    let stale_prepared = lsm_compaction_runtime()
        .execute(stale_demand)
        .into_result()
        .expect("concurrent preparation does not retire membership");
    reject_unrelated_physical_publications(&world, &prepared);
    let interlocked = lsm_physical_compaction_runtime()
        .admit(prepared, world.physical_publication.clone())
        .into_result()
        .expect("physical isolation admits the exact prepared compaction");
    let stale_interlocked = lsm_physical_compaction_runtime()
        .admit(stale_prepared, world.physical_publication.clone())
        .into_result()
        .expect("concurrent semantic preparation binds the same executed physical rewrite");
    let activation = interlocked
        .prepare_membership_activation()
        .into_result()
        .expect("executed physical publication prepares durable membership activation");
    let activation_manifest = admit_checkpoint_publication(&manifest_receipt_for_artifact(
        activation.scope().clone(),
        activation.artifact().bytes(),
    ))
    .expect("post-physical membership activation is durably published");
    let manifest_path = activation_manifest.persisted_path().to_path_buf();
    let manifest_bytes = std::fs::read(&manifest_path).expect("persisted activation bytes");
    reject_wrong_selected_publication(
        &mut world.persisted,
        world.wrong_publication.clone(),
        interlocked.clone(),
        activation.clone(),
        activation_manifest.clone(),
    );
    let published = lsm_publication_runtime()
        .publish(
            &mut world.persisted,
            world.publication.clone(),
            interlocked,
            activation.clone(),
            activation_manifest.clone(),
        )
        .into_result()
        .expect("publication makes prepared compaction visible and retires old membership");
    assert_eq!(
        published.publication_execution().counters().publications(),
        2
    );
    assert_eq!(
        published
            .compaction_publication_receipt()
            .counters()
            .publications(),
        1,
    );
    let stale_denial = lsm_publication_runtime()
        .publish(
            &mut world.persisted,
            world.publication.clone(),
            stale_interlocked,
            activation,
            activation_manifest,
        )
        .into_result()
        .expect_err("retirement must stale every concurrent prepared compaction");
    assert_eq!(
        stale_denial,
        BaselineLsmExecutionAdmissionDenial::PersistedMembershipStale,
    );
    verify_durable_retirement_and_manifest_integrity(&world, manifest_path, manifest_bytes);
    let reader_cutover = published
        .observe_reader_cutover(
            world.physical_recovery,
            world.pre_cutover_read,
            world.post_cutover_read,
        )
        .expect("semantic LSM publication preserves physical reader cutover authority");
    (published, reader_cutover)
}

fn reject_unrelated_physical_publications(
    world: &ReaderCutoverWorld,
    prepared: &forge_store_layout_indexes::PreparedLsmCompaction,
) {
    let (wrong_publication, _, _, _) =
        crate::harness::physical_isolation::compaction::execute_compaction_cutover_for_manifest(
            &world.physical_plan,
            world.physical_manifest_epoch + 1,
        )
        .into_parts();
    assert_eq!(
        lsm_physical_compaction_runtime()
            .admit(prepared.clone(), wrong_publication)
            .into_result()
            .unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch,
    );
    for seed in [17, 18] {
        let plan =
            crate::harness::physical_isolation::compaction::admitted_compaction_plan_for_seed(seed);
        let manifest = plan.protected().root().manifest_epoch().get() + 1;
        let (publication, _, _, _) = crate::harness::physical_isolation::compaction::
            execute_compaction_cutover_for_manifest(&plan, manifest)
            .into_parts();
        assert_eq!(
            lsm_physical_compaction_runtime()
                .admit(prepared.clone(), publication)
                .into_result()
                .unwrap_err(),
            BaselineLsmExecutionAdmissionDenial::PhysicalPublicationBindingMismatch,
        );
    }
}

fn reject_wrong_selected_publication(
    persisted: &mut forge_store_lsm_authority::LsmMembershipSession,
    wrong: forge_store_layout_indexes::BaselineLsmRunPublicationAdmission,
    interlocked: forge_store_layout_indexes::InterlockedLsmCompaction,
    activation: forge_store_lsm_authority::LsmMembershipActivationDeclaration,
    manifest: forge_store_wal::AdmittedCheckpointPublicationReceipt,
) {
    let denial = lsm_publication_runtime()
        .publish(persisted, wrong, interlocked, activation, manifest)
        .into_result()
        .expect_err("wrong selected publication must fail before membership retirement");
    assert_eq!(
        denial,
        BaselineLsmExecutionAdmissionDenial::SelectedOperationKeyMismatch,
    );
}

fn verify_durable_retirement_and_manifest_integrity(
    world: &ReaderCutoverWorld,
    manifest_path: std::path::PathBuf,
    manifest_bytes: Vec<u8>,
) {
    let reopened = open_lsm_index(&world.first_durable)
        .expect("retired membership reopens through live retirement validation");
    assert_eq!(
        world
            .access
            .lower_compaction(&reopened, world.key, world.compaction.clone()),
        Err(BaselineLsmExecutionAdmissionDenial::ValueRecordRequired),
    );
    let mut substituted = manifest_bytes.clone();
    *substituted
        .last_mut()
        .expect("membership manifest is nonempty") ^= 0x01;
    std::fs::write(&manifest_path, &substituted)
        .expect("same-length hostile manifest substitution");
    assert_eq!(
        open_lsm_index(&world.first_durable).unwrap_err(),
        BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch,
    );
    std::fs::write(manifest_path, manifest_bytes).expect("restore persisted manifest fixture");
}
