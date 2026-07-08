#[test]
fn phase_five_denies_non_baseline_or_domain_mismatched_strategy_claims() {
    use super::tests_support::{admit_phase_five_scope, root_manifest_scope};
    use crate::{strategy_admission, S8LayoutStrategyFamily, S8StrategyDenial};
    use forge_store_contracts::DurableArtifactFamilyId;
    use forge_store_security::{
        StoreAuthenticityRequirement, StoreAuthenticityRequirementClass, StoreCustodyPosture,
        StoreKeyScope, StoreTenantScope,
    };

    let (page_lifecycle, page_domain) = admit_phase_five_scope(
        DurableArtifactFamilyId::PhysicalPage,
        StoreKeyScope::PageEnvelope,
        StoreTenantScope::TenantPhysicalBoundary,
        StoreAuthenticityRequirement::required(
            StoreAuthenticityRequirementClass::AuthenticatedFrame,
        ),
        StoreCustodyPosture::InternalStoreCustody,
    );
    let (root_lifecycle, root_domain) = root_manifest_scope();

    assert_eq!(
        strategy_admission().admit_baseline_strategy(
            page_lifecycle,
            page_domain,
            S8LayoutStrategyFamily::ExactScan,
        ),
        Err(S8StrategyDenial::UnsupportedFamily)
    );
    assert_eq!(
        strategy_admission().admit_baseline_strategy(
            root_lifecycle,
            root_domain,
            S8LayoutStrategyFamily::BTree,
        ),
        Err(S8StrategyDenial::PhysicalKeyDomainDoesNotSupportBaselineBTree)
    );
    assert_eq!(
        strategy_admission().admit_baseline_strategy(
            page_lifecycle,
            root_domain,
            S8LayoutStrategyFamily::BTree,
        ),
        Err(S8StrategyDenial::FamilyDoesNotMatchKeyDomain)
    );
}

#[test]
fn phase_five_admission_binds_counter_profiles_to_baseline_strategy_families() {
    use super::tests_support::{admit_btree_page_strategy, admit_lsm_wal_strategy};
    use crate::execution::S8AccessPathKind;
    use crate::{S8StrategyLookupInvariant, S8StrategyPublicationInvariant};
    use forge_store_physical_format::layout_access::baseline_btree_counter_observation::{
        execute_baseline_btree_lookup, BaselineBTreeLookupBranch,
    };
    use forge_store_wal::layout_access::baseline_lsm_counter_observation::{
        execute_baseline_lsm_lookup, execute_baseline_lsm_replay, BaselineLsmLookupDisposition,
    };

    let btree = admit_btree_page_strategy();
    let lsm = admit_lsm_wal_strategy();
    let btree_suite = btree.invariant_suite();
    let lsm_suite = lsm.invariant_suite();

    assert_eq!(
        btree_suite.lookup_invariant(),
        S8StrategyLookupInvariant::SeparatorDirectedLookup
    );
    assert_eq!(
        lsm_suite.publication_invariant(),
        S8StrategyPublicationInvariant::ManifestPublication
    );
    let btree_evidence = btree_suite.counter_evidence();
    let lsm_evidence = lsm_suite.counter_evidence();

    assert_eq!(
        btree_evidence.lookup().path_kind(),
        S8AccessPathKind::BaselineBTreePointLookup
    );
    assert_eq!(
        btree_evidence.publication().path_kind(),
        S8AccessPathKind::BaselineBTreeRootPublication
    );
    assert_eq!(
        btree_evidence.recovery().path_kind(),
        S8AccessPathKind::BaselineBTreeReplayRecovery
    );
    assert_eq!(
        lsm_evidence.lookup().path_kind(),
        S8AccessPathKind::BaselineLsmPointLookup
    );
    assert_eq!(
        lsm_evidence.publication().path_kind(),
        S8AccessPathKind::BaselineLsmManifestPublication
    );
    assert_eq!(
        lsm_evidence.recovery().path_kind(),
        S8AccessPathKind::BaselineLsmWalReplay
    );
    let btree_lookup_execution = execute_baseline_btree_lookup();
    let lsm_lookup_execution = execute_baseline_lsm_lookup();
    let lsm_replay_execution = execute_baseline_lsm_replay();

    assert_eq!(
        btree_lookup_execution.branch(),
        BaselineBTreeLookupBranch::Left
    );
    assert!(
        btree_lookup_execution.probe_slot().get() < btree_lookup_execution.separator_slot().get()
    );
    assert_eq!(
        btree_lookup_execution
            .selected_reference()
            .slot()
            .map(|slot| slot.get()),
        Some(1)
    );
    assert_eq!(
        lsm_lookup_execution.disposition(),
        BaselineLsmLookupDisposition::Memtable
    );
    assert_eq!(
        lsm_lookup_execution.memtable_record().sequence(),
        lsm_lookup_execution.probe_sequence()
    );
    assert!(
        lsm_lookup_execution.sorted_run_record().sequence() < lsm_lookup_execution.probe_sequence()
    );
    assert_eq!(lsm_replay_execution.replayable_count(), 1);
    assert!(btree_evidence.lookup().parity_holds());
    assert!(btree_evidence.publication().parity_holds());
    assert!(btree_evidence.recovery().parity_holds());
    assert!(lsm_evidence.lookup().parity_holds());
    assert!(lsm_evidence.publication().parity_holds());
    assert!(lsm_evidence.recovery().parity_holds());
    assert_eq!(
        btree_evidence.lookup().observed(),
        btree_evidence.lookup().planned()
    );
    assert_eq!(
        btree_evidence.publication().observed(),
        btree_evidence.publication().planned()
    );
    assert_eq!(
        btree_evidence.recovery().observed(),
        btree_evidence.recovery().planned()
    );
    assert_eq!(
        lsm_evidence.lookup().observed(),
        lsm_evidence.lookup().planned()
    );
    assert_eq!(
        lsm_evidence.publication().observed(),
        lsm_evidence.publication().planned()
    );
    assert_eq!(
        lsm_evidence.recovery().observed(),
        lsm_evidence.recovery().planned()
    );
    assert_eq!(
        btree_evidence.lookup().observed().point_lookups(),
        btree_lookup_execution.counters().point_lookups()
    );
    assert_eq!(
        lsm_evidence.lookup().observed().range_lookups(),
        lsm_lookup_execution.counters().range_lookups()
    );
    assert_eq!(
        lsm_evidence.recovery().observed().wal_replays(),
        lsm_replay_execution.counters().wal_replays()
    );
    assert_eq!(btree_evidence.lookup().planned().point_lookups(), 1);
    assert_eq!(btree_evidence.publication().planned().publications(), 1);
    assert_eq!(btree_evidence.recovery().planned().maintenance_reads(), 1);
    assert_eq!(lsm_evidence.lookup().planned().range_lookups(), 1);
    assert_eq!(lsm_evidence.publication().planned().publications(), 2);
    assert_eq!(lsm_evidence.recovery().planned().wal_replays(), 1);
    assert_eq!(btree_evidence.aggregate_profile().wal_replays(), 0);
    assert_eq!(lsm_evidence.aggregate_profile().wal_replays(), 1);
    assert!(
        lsm_evidence.aggregate_profile().publications()
            > btree_evidence.aggregate_profile().publications()
    );
    assert!(
        lsm_evidence.aggregate_profile().maintenance_reads()
            > btree_evidence.aggregate_profile().maintenance_reads()
    );
}
