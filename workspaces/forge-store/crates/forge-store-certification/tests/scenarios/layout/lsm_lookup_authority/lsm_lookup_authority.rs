mod activation_mutation;

use activation_mutation::{mutate_activation_field, ALL_ACTIVATION_FIELDS};
use forge_store_budgets::PreExecutionBudgetEnvelope;
use forge_store_contracts::WalRecordFamily;
use forge_store_layout_indexes::{
    baseline_lsm_lookup_cases, layout_read_runtime, lsm_strategy,
    BaselineLsmExecutionAdmissionDenial, BaselineLsmLookupDisposition, BaselineLsmLookupView,
    BootstrapCatalogReadAdmission, LayoutReadAdmissionDenied, PlannedCounterObservation,
    WalLookupRequest,
};
use forge_store_lsm_authority::{LsmMembershipDenial, LsmMembershipReplayPosture};
use forge_store_security::admitted_store_wal_checkpoint_security_scope_for_layout_partition_test;
use forge_store_security::admitted_tenant_wal_checkpoint_security_scope_for_layout_partition_test;
use forge_store_test_support::{
    admitted_layout_bootstrap_catalog, advanced_admitted_layout_bootstrap_catalog,
    execute_baseline_lsm_persisted_fixture, execute_lsm_compaction_reader_cutover_fixture,
    execute_lsm_replay_hostile_matrix, execute_repeated_lsm_membership_fixture,
    lsm_membership_replacement_crash_fixture, substituted_lsm_base_is_rejected_before_compaction,
};
use forge_store_wal::{AdmittedWalAppendReceipt, AdmittedWalArtifactStore, StoreWalRecordIdentity};

fn reopen(
    anchor: &AdmittedWalAppendReceipt,
) -> Result<forge_store_lsm_authority::LsmMembershipSession, BaselineLsmExecutionAdmissionDenial> {
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    lsm_strategy().open_index(anchor, security.witnesses())
}

#[test]
fn ordinary_runtime_rejects_lsm_membership_displaced_before_execution() {
    let catalog = admitted_layout_bootstrap_catalog();
    let advanced = advanced_admitted_layout_bootstrap_catalog();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let source = execute_baseline_lsm_persisted_fixture().admit_lookup_source();

    let denial = layout_read_runtime()
        .execute_wal_lookup(
            WalLookupRequest::new(
                &catalog,
                security.witnesses(),
                WalRecordFamily::DurableMutationIntent,
                StoreWalRecordIdentity::new(43),
                43,
                PreExecutionBudgetEnvelope::foreground_default(),
                source,
            )
            .against_current_catalog(&advanced),
        )
        .expect_err("an advanced catalog must stale the admitted LSM membership");

    assert!(matches!(
        denial,
        LayoutReadAdmissionDenied::StaleMaterialization(_)
    ));
}

#[test]
fn ordinary_runtime_selects_and_executes_persisted_lsm_membership() {
    let catalog = admitted_layout_bootstrap_catalog();
    let security = admitted_store_wal_checkpoint_security_scope_for_layout_partition_test();
    let durable = execute_baseline_lsm_persisted_fixture();
    let source = durable.admit_lookup_source();

    let newest = execute(&catalog, security.witnesses(), source.clone(), 43);
    let older = execute(&catalog, security.witnesses(), source.clone(), 42);
    let blocked = execute(&catalog, security.witnesses(), source.clone(), 41);

    assert_eq!(newest.disposition(), BaselineLsmLookupDisposition::Memtable);
    assert_eq!(older.disposition(), BaselineLsmLookupDisposition::SortedRun);
    assert_eq!(
        blocked.disposition(),
        BaselineLsmLookupDisposition::NotFound
    );
    assert!(blocked.tombstone_blocks_older());
    let BaselineLsmLookupView::Absent(absence) = blocked.view() else {
        panic!("not-found LSM execution must issue the absent case")
    };
    assert_eq!(absence.probe_sequence(), 41);
    assert!(absence.tombstone_blocks_older());
    assert_eq!(
        blocked
            .plan_binding()
            .materialization()
            .expect("executed LSM plan retains materialization")
            .source(),
        blocked.current_materialization().materialization().source(),
    );
    assert!(matches!(newest.view(), BaselineLsmLookupView::Memtable(_)));
    assert!(matches!(older.view(), BaselineLsmLookupView::SortedRun(_)));
    let declared = baseline_lsm_lookup_cases()
        .map(|case| case.name())
        .collect::<std::collections::BTreeSet<_>>();
    let observed = [newest.case_id(), older.case_id(), blocked.case_id()]
        .into_iter()
        .map(|case| case.name())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(declared, observed);
    for (counters, comparisons) in [
        (newest.counters(), 1),
        (older.counters(), 2),
        (blocked.counters(), 3),
    ] {
        assert_eq!(counters.point_lookups(), 1);
        assert_eq!(counters.range_lookups(), 0);
        assert_eq!(counters.wal_replays(), 0);
        assert_eq!(counters.publications(), 0);
        assert_eq!(counters.maintenance_reads(), 0);
        assert_eq!(counters.index_probes(), comparisons);
        assert_eq!(counters.key_comparisons(), comparisons);
    }
    assert_eq!(
        newest.counter_receipt().observation(),
        PlannedCounterObservation::WithinEnvelope
    );
    assert_eq!(newest.counter_receipt().observed().allocation_events(), 1);
    assert_eq!(
        older.counter_receipt().observation(),
        PlannedCounterObservation::WithinEnvelope
    );
    assert_eq!(
        blocked.counter_receipt().observation(),
        PlannedCounterObservation::Exact
    );
    assert_eq!(
        newest
            .current_materialization()
            .materialization()
            .source()
            .kind(),
        forge_store_layout_indexes::LayoutMaterializationSourceKind::LsmReplacement(
            source.replacement_output(),
        ),
    );
}

#[test]
fn semantic_compaction_preserves_pre_and_post_cutover_reader_authority() {
    let fixture = execute_lsm_compaction_reader_cutover_fixture();

    assert!(fixture
        .reader_cutover()
        .pre_cutover_reader_retained_old_structure());
    assert!(fixture
        .reader_cutover()
        .post_cutover_reader_observed_new_epoch());
    assert_eq!(
        fixture
            .reader_cutover()
            .proof()
            .publication()
            .publication()
            .new_root(),
        fixture
            .published()
            .physical_compaction()
            .publication()
            .new_root(),
    );
}

#[test]
fn tenant_scoped_wal_claim_cannot_enter_store_internal_lsm_lookup() {
    let catalog = admitted_layout_bootstrap_catalog();
    let wrong_security = admitted_tenant_wal_checkpoint_security_scope_for_layout_partition_test();
    let durable = execute_baseline_lsm_persisted_fixture();

    assert_eq!(
        layout_read_runtime().execute_wal_lookup(WalLookupRequest::new(
            &catalog,
            wrong_security.witnesses(),
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(43),
            43,
            PreExecutionBudgetEnvelope::foreground_default(),
            durable.admit_lookup_source(),
        )),
        Err(LayoutReadAdmissionDenied::SecurityScope),
    );
}

#[test]
fn missing_activation_cannot_retire_durable_membership() {
    let fixture = lsm_membership_replacement_crash_fixture();
    let hidden = fixture.activation_path().with_extension("not-published");
    std::fs::rename(fixture.activation_path(), &hidden).unwrap();

    let reopened = reopen(fixture.anchor()).unwrap();

    assert!(
        forge_store_lsm_authority::select_lsm_compaction_membership(&reopened, fixture.key())
            .into_result()
            .is_ok()
    );
    assert_eq!(
        forge_store_lsm_authority::lookup_published_lsm_membership(&reopened, fixture.key())
            .into_result(),
        Err(LsmMembershipDenial::MembershipIncomplete)
    );
    std::fs::rename(hidden, fixture.activation_path()).unwrap();
}

#[test]
fn every_torn_activation_prefix_fails_closed_or_leaves_old_membership_active() {
    let fixture = lsm_membership_replacement_crash_fixture();
    assert_eq!(
        fixture.wrong_physical_denial(),
        LsmMembershipDenial::PhysicalPublicationBindingMismatch
    );
    for cut in 1..fixture.activation_bytes().len() {
        std::fs::write(
            fixture.activation_path(),
            &fixture.activation_bytes()[..cut],
        )
        .unwrap();
        match reopen(fixture.anchor()) {
            Ok(reopened) => {
                assert!(forge_store_lsm_authority::select_lsm_compaction_membership(
                    &reopened,
                    fixture.key()
                )
                .into_result()
                .is_ok());
                assert_eq!(
                    forge_store_lsm_authority::lookup_published_lsm_membership(
                        &reopened,
                        fixture.key(),
                    )
                    .into_result(),
                    Err(LsmMembershipDenial::MembershipIncomplete)
                );
            }
            Err(BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch) => {}
            Err(other) => panic!("torn activation escaped fail-closed handling: {other:?}"),
        }
    }
    std::fs::write(fixture.activation_path(), fixture.activation_bytes()).unwrap();
    let reopened = reopen(fixture.anchor()).unwrap();

    assert_eq!(
        reopened.replay_posture(),
        LsmMembershipReplayPosture::DurableArtifactsReadmitted
    );
    assert_eq!(
        forge_store_lsm_authority::lookup_published_lsm_membership(&reopened, fixture.key())
            .into_result()
            .unwrap()
            .output(),
        fixture.replacement_output()
    );
    assert_eq!(
        forge_store_lsm_authority::select_lsm_compaction_membership(&reopened, fixture.key())
            .into_result(),
        Err(LsmMembershipDenial::ValueRecordRequired)
    );
}

#[test]
fn complete_corrupt_activation_fails_closed_on_ordinary_reopen() {
    let fixture = lsm_membership_replacement_crash_fixture();
    let mut corrupt = fixture.activation_bytes().to_vec();
    corrupt[32] ^= 0x01;
    std::fs::write(fixture.activation_path(), corrupt).unwrap();

    assert!(matches!(
        reopen(fixture.anchor()),
        Err(BaselineLsmExecutionAdmissionDenial::ManifestMembershipMismatch)
    ));
}

#[test]
fn checksum_valid_activation_field_substitutions_all_fail_closed() {
    let fixture = lsm_membership_replacement_crash_fixture();

    for field in ALL_ACTIVATION_FIELDS {
        let mutated = mutate_activation_field(fixture.activation_bytes(), field);
        std::fs::write(fixture.activation_path(), mutated).unwrap();
        assert!(
            reopen(fixture.anchor()).is_err(),
            "checksum-valid {field:?} substitution was admitted",
        );
    }

    std::fs::write(fixture.activation_path(), fixture.activation_bytes()).unwrap();
    assert!(reopen(fixture.anchor()).is_ok());
}

#[test]
fn wal_store_scan_and_membership_reopen_counters_are_exact() {
    let fixture = lsm_membership_replacement_crash_fixture();
    let store = AdmittedWalArtifactStore::open(fixture.anchor()).unwrap();
    let artifacts = store.scan().unwrap();
    let expected_bytes = artifacts
        .artifacts()
        .iter()
        .map(|artifact| artifact.bytes().len() as u64)
        .sum::<u64>();

    assert_eq!(artifacts.artifacts().len(), 5);
    assert_eq!(artifacts.counters().directories_examined(), 5);
    assert_eq!(artifacts.counters().artifacts_read(), 5);
    assert_eq!(artifacts.counters().bytes_read(), expected_bytes);

    let reopened = reopen(fixture.anchor()).unwrap();
    let counters = reopened.reopen_counters();
    assert_eq!(counters.artifacts_examined(), 5);
    assert_eq!(counters.artifacts_readmitted(), 4);
    assert_eq!(counters.bytes_examined(), expected_bytes);
}

#[test]
fn wal_store_handle_rejects_an_equal_scope_append_from_another_store() {
    let first = lsm_membership_replacement_crash_fixture();
    let second = lsm_membership_replacement_crash_fixture();
    let store = AdmittedWalArtifactStore::open(first.anchor()).unwrap();

    assert!(store.admits_append(first.anchor()));
    assert!(!store.admits_append(second.anchor()));
    assert!(!store.admits_persisted_path(second.activation_path()));
}

#[test]
fn same_length_replacement_artifact_substitution_fails_closed_on_reopen() {
    let fixture = lsm_membership_replacement_crash_fixture();
    let mut substituted = std::fs::read(fixture.replacement_path()).unwrap();
    let byte = substituted
        .last_mut()
        .expect("replacement artifact is nonempty");
    *byte ^= 0x01;
    std::fs::write(fixture.replacement_path(), substituted).unwrap();

    assert!(matches!(
        reopen(fixture.anchor()),
        Err(BaselineLsmExecutionAdmissionDenial::OutputPublicationMismatch)
    ));
}

#[test]
fn repeated_compaction_selects_and_replays_the_published_base_frontier() {
    let fixture = execute_repeated_lsm_membership_fixture();

    assert_eq!(fixture.selected_base(), fixture.first_output());
    assert_ne!(fixture.second_output(), fixture.first_output());
    assert_eq!(fixture.reopened_output(), fixture.second_output());
    assert_eq!(fixture.reopened_identity(), fixture.published_identity());
}

#[test]
fn substituted_published_base_is_rejected_before_repeated_compaction() {
    assert_eq!(
        substituted_lsm_base_is_rejected_before_compaction(),
        LsmMembershipDenial::DurableRecordBindingMismatch
    );
}

#[test]
fn malformed_replay_membership_is_rejected_by_its_owning_boundary() {
    let matrix = execute_lsm_replay_hostile_matrix();
    assert!(matrix
        .permutation_denials()
        .iter()
        .all(|denial| *denial == forge_store_lsm_authority::LsmReplaySourceDenial::MembershipSequenceNotStrictlyIncreasing));
    assert_eq!(
        matrix.duplicate_sequence_denial(),
        forge_store_lsm_authority::LsmReplaySourceDenial::MembershipSequenceNotStrictlyIncreasing
    );
    assert!(matrix
        .unsupported_kind_denials()
        .iter()
        .all(|denial| *denial == LsmMembershipDenial::UnsupportedRecordKind));
    assert_eq!(
        matrix.retired_membership_denial(),
        LsmMembershipDenial::ValueRecordRequired
    );
}

fn execute(
    catalog: &BootstrapCatalogReadAdmission,
    security: &forge_store_security::StoreCurrentSecurityScopeWitnessSet,
    source: forge_store_layout_indexes::BaselineLsmLookupSource,
    probe_sequence: u64,
) -> forge_store_layout_indexes::BaselineLsmLookupExecution {
    layout_read_runtime()
        .execute_wal_lookup(WalLookupRequest::new(
            catalog,
            security,
            WalRecordFamily::DurableMutationIntent,
            StoreWalRecordIdentity::new(probe_sequence),
            probe_sequence,
            PreExecutionBudgetEnvelope::foreground_default(),
            source,
        ))
        .unwrap()
}
