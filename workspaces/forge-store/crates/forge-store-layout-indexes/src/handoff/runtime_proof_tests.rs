use super::*;

#[test]
fn every_runtime_target_executes_its_owner_production_operation() {
    let inventory = S8LayoutHazardInventory::canonical();
    let mut executed = 0;
    let mut targets = Vec::new();
    for row in inventory.rows() {
        let S8HazardProofTarget::Runtime(target) = row.proof_target() else {
            continue;
        };
        assert!(
            !targets.contains(&target),
            "runtime target reused: {target:?}"
        );
        targets.push(target);
        execute_runtime_target(target);
        executed += 1;
    }
    assert_eq!(executed, 8, "every runtime hazard must execute");
}

fn execute_runtime_target(target: S8RuntimeProofTarget) {
    match target {
        S8RuntimeProofTarget::HiddenScanDeniedWithOwnerCounters => hidden_scan_owner_denial(),
        S8RuntimeProofTarget::StaleIndexCannotReachExactAccess => stale_exactness_denial(),
        S8RuntimeProofTarget::CrossScopeIndexDeniedByScopeAdmission => cross_scope_denial(),
        S8RuntimeProofTarget::UnbudgetedDegradedScanDenied => unbudgeted_degraded_denial(),
        S8RuntimeProofTarget::PartialCoverageCannotProveAbsence => partial_absence_denial(),
        S8RuntimeProofTarget::CorruptionQuarantinesBeforeRebuild => corruption_quarantine(),
        S8RuntimeProofTarget::BTreeSeparatorCorruptionDenied => btree_separator_corruption_denial(),
        S8RuntimeProofTarget::BootstrapMisdiscoveryDenied => bootstrap_misdiscovery_denial(),
    }
}

fn stale_exactness_denial() {
    use crate::facade::{access_planning, layout_declarations};
    use crate::{S8AccessShapeUnsupportedDenial, S8MaterializationDenial};
    use forge_store_physical_format::PhysicalEpoch;

    let family = layout_declarations().seed_family();
    let stale = access_planning()
        .stale_root_epoch_coverage(family, PhysicalEpoch::from_raw(34).unwrap())
        .expect("stale coverage remains inspectable for rebind");
    assert!(matches!(
        access_planning().require_exact_point_access(stale),
        Err(S8AccessShapeUnsupportedDenial::MaterializationDenied(
            S8MaterializationDenial::LayoutCoverageIsStale { .. }
        ))
    ));
}

fn hidden_scan_owner_denial() {
    use forge_store_physical_certification::layout_harness::runtime::S8RuntimeEvidence;
    use forge_store_physical_certification::layout_harness::runtime_execution::execute_core_physical_case;
    use forge_store_physical_format::PlatformPhysicalRuntimeOperation;

    let evidence =
        execute_core_physical_case(forge_store_contracts::S8RuntimeCase::HiddenScanDenial)
            .expect("physical-format hidden-scan denial must execute");
    let S8RuntimeEvidence::PlatformPhysical(receipt) = evidence else {
        panic!("hidden scan must retain the physical-format receipt");
    };
    assert_eq!(
        receipt.operation(),
        PlatformPhysicalRuntimeOperation::DenyHiddenBroadScan
    );
    assert!(receipt.fact().counters().matches_plan());
    assert_eq!(receipt.fact().counters().planned_units(), 0);
    assert_eq!(receipt.fact().counters().observed_units(), 0);
    assert_eq!(
        receipt.counters().full_store_materialization_rejections(),
        1
    );
    assert_eq!(receipt.counters().scans(), 0);
    assert_eq!(receipt.counters().reads(), 0);
    assert_eq!(receipt.counters().locates(), 0);
}

fn cross_scope_denial() {
    super::tests::execute_cross_scope_runtime_denial();
}

fn unbudgeted_degraded_denial() {
    super::tests::execute_unbudgeted_degraded_denial();
}

fn partial_absence_denial() {
    use crate::facade::{access_planning, layout_declarations};
    use crate::S8MaterializationDenial;
    use forge_store_recovery_physics::{CheckpointCoveredLsnRange, LogSequenceNumber};

    let gap =
        CheckpointCoveredLsnRange::new(LogSequenceNumber::new(11), LogSequenceNumber::new(19))
            .unwrap();
    let partial = access_planning()
        .partial_wal_lsn_coverage(
            layout_declarations().seed_family(),
            LogSequenceNumber::new(10),
            LogSequenceNumber::new(20),
            gap,
        )
        .unwrap();
    let outcome = access_planning().prove_exact_index_absence(partial);
    assert!(
        crate::production_transition::S8LayoutMachineContract::for_machine(
            crate::production_transition::S8LayoutStateMachine::MaterializationCoverageAbsence,
        )
        .contains(outcome.production_transition())
    );
    assert!(matches!(
        outcome.into_result(),
        Err(S8MaterializationDenial::LayoutCoverageIsPartial { .. })
    ));
}

pub(crate) fn assert_partial_absence_transition_equivalence() {
    partial_absence_denial();
}

fn corruption_quarantine() {
    crate::maintenance::tests::execute_authoritative_wal_corruption_quarantine();
}

fn btree_separator_corruption_denial() {
    use crate::strategy::tests_support::{admit_btree_page_strategy, admitted_page_key_bytes};
    use crate::{S8BTreeLookupBranch, S8StrategyDenial};

    let suite = admit_btree_page_strategy()
        .invariant_suite()
        .require_btree_suite()
        .unwrap();
    let outcome = suite.search_path_law().verify_search_and_insertion_path(
        &admitted_page_key_bytes(1, 5),
        &admitted_page_key_bytes(1, 30),
        &admitted_page_key_bytes(1, 20),
        &admitted_page_key_bytes(1, 30),
        S8BTreeLookupBranch::Left,
    );
    assert!(
        crate::production_transition::S8LayoutMachineContract::for_machine(
            crate::production_transition::S8LayoutStateMachine::BTreeSearchPathInvariant,
        )
        .contains(outcome.production_transition())
    );
    assert_eq!(outcome, Err(S8StrategyDenial::ComparatorOrderViolation));
}

fn bootstrap_misdiscovery_denial() {
    use crate::S8BootstrapOnlyAccessDenied;

    assert!(matches!(
        crate::bootstrap::test_support::mismatched_current_root_denial(),
        S8BootstrapOnlyAccessDenied::CurrentRootReadmissionRequired { .. }
    ));
}
