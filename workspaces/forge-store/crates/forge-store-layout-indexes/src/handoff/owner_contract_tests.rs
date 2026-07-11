use super::{
    S8HazardProofLane, S8HazardProofTarget, S8LayoutHazard, S8LayoutHazardInventory,
    S8RuntimeProofTarget, S9LayoutMachineContract, S9LayoutMachineState as State,
    S9LayoutMachineTransition as Transition, S9LayoutProductionOperation, S9LayoutStateMachine,
};

#[test]
fn owner_contracts_preserve_real_non_success_topology() {
    for machine in [
        S9LayoutStateMachine::ArtifactDeclaration,
        S9LayoutStateMachine::KeyDomainAdmission,
        S9LayoutStateMachine::StrategyInvariantAdmission,
        S9LayoutStateMachine::LegacyDisposition,
    ] {
        assert!(!S9LayoutMachineContract::for_machine(machine)
            .transitions()
            .is_empty());
    }

    let readiness = S9LayoutMachineContract::for_machine(S9LayoutStateMachine::ExecutionReadiness);
    assert!(readiness.permits_edge(State::Lowered, Transition::Ready, State::Stale));

    let readmission =
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::StaleRebindReadmission);
    for (from, transition, to) in [
        (
            State::Lowered,
            Transition::RequireRebind,
            State::RebindRequired,
        ),
        (State::RebindRequired, Transition::Rebind, State::Lowered),
        (State::Stale, Transition::Readmit, State::Readmitted),
        (State::Stale, Transition::Defer, State::Deferred),
        (State::Stale, Transition::Deny, State::Denied),
    ] {
        assert!(readmission.permits_edge(from, transition, to));
    }

    let maintenance = S9LayoutMachineContract::for_machine(
        S9LayoutStateMachine::LiveMaintenanceAdmissionAndLowering,
    );
    for state in [
        State::MaintenanceReady,
        State::MaintenanceLagged,
        State::MaintenanceDeferred,
        State::MaintenanceRebuildOnly,
        State::MaintenanceAdvisoryOnly,
        State::MaintenanceVerifierOnly,
        State::MaintenanceMigrationOnly,
    ] {
        assert!(maintenance
            .transitions()
            .iter()
            .any(|artifact| artifact.edge().to() == state));
    }

    let migration =
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::MigrationRollbackPlanning);
    assert!(migration
        .transitions()
        .iter()
        .any(|artifact| artifact.edge().to() == State::RebindRequired));

    let corruption =
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::CorruptionQuarantine);
    for state in [
        State::Clean,
        State::NotFound,
        State::Stale,
        State::RebuildRequired,
        State::MigrationRequired,
        State::Quarantined,
        State::QuarantineReadmissionRequired,
        State::OfflineEvidenceReadmissionRequired,
        State::TerminalImportReadmissionRequired,
        State::Unsupported,
        State::Readmitted,
    ] {
        assert!(corruption
            .transitions()
            .iter()
            .any(|artifact| artifact.edge().to() == state));
    }

    let bootstrap =
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::BootstrapCatalogDiscovery);
    crate::bootstrap::test_support::bootstrap_exact_materialization(
        crate::layout_declarations().seed_family().family(),
    );
    assert!(bootstrap.permits_edge(
        State::CatalogDiscovered,
        Transition::ValidateCurrentRoot,
        State::CurrentRootAdmitted,
    ));
    let denied_bootstrap = crate::bootstrap::test_support::mismatched_current_root_outcome();
    assert!(bootstrap.contains(denied_bootstrap.production_transition()));
    assert!(denied_bootstrap.is_err());
}

#[test]
fn ordinary_execution_and_corruption_outcomes_are_the_handoff_source() {
    let declaration = crate::layout_declarations().seed_family();
    assert!(
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::ArtifactDeclaration)
            .contains(declaration.production_transition())
    );

    let (lifecycle, key_domain) = crate::execution::tests_support::admit_page_scope();

    let strategy_outcome = crate::strategy_registry::layout_admission_registry().admit(
        crate::strategy_registry::S8LayoutAdmissionRequest::new(
            lifecycle,
            key_domain,
            crate::S8LayoutStrategyFamily::BaselineBTreeRange,
            crate::strategy_registry::S8LayoutRequestedCapability::point_lookup(),
            crate::ArtifactFamilyAccessLane::HotPath,
        ),
    );
    assert!(
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::LayoutAdmission)
            .contains(strategy_outcome.production_transition())
    );
    let strategy = strategy_outcome.unwrap().admitted_strategy();
    let invariant_suite = strategy.invariant_suite();
    assert!(
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::StrategyInvariantAdmission)
            .contains(strategy.invariant_production_transition())
    );
    let btree_suite = invariant_suite.require_btree_suite().unwrap();
    let btree_lookup = btree_suite.verify_baseline_lookup();
    assert!(
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::BTreeSearchPathInvariant)
            .contains(btree_lookup.production_transition())
    );
    btree_lookup.unwrap();

    let coverage = crate::facade::access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(declaration.family()),
            forge_store_physical_format::PhysicalEpoch::from_raw(340).unwrap(),
        )
        .unwrap();
    let absence_outcome = crate::facade::access_planning().prove_exact_index_absence(coverage);
    assert!(S9LayoutMachineContract::for_machine(
        S9LayoutStateMachine::MaterializationCoverageAbsence
    )
    .contains(absence_outcome.production_transition()));
    absence_outcome.unwrap();
    super::runtime_proof_tests::assert_partial_absence_transition_equivalence();

    let legacy = crate::LegacySurfaceDispositionAndDedicatedWorkspaceBoundary::current()
        .inventory()
        .disposition_for("ForgeStore");
    assert!(
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::LegacyDisposition)
            .contains(legacy.production_transition())
    );

    crate::access_shape::tests::assert_hidden_scan_owner_transition_equivalence();
    crate::planning::tests::assert_owner_transition_handoff_equivalence();
    crate::execution::assert_owner_transition_handoff_equivalence();
    crate::corruption::assert_owner_transition_handoff_equivalence();
    crate::migration::tests::assert_owner_transition_handoff_equivalence();
    crate::maintenance::tests::assert_rebuild_owner_transition_handoff_equivalence();
    crate::maintenance::live_tests::assert_live_owner_transition_handoff_equivalence();
}

#[test]
fn stale_exactness_is_bound_to_the_runtime_derivation_lane() {
    let row = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.hazard() == S8LayoutHazard::StaleIndexExactness)
        .unwrap();
    assert_eq!(row.proof_lane(), S8HazardProofLane::Runtime);
    assert_eq!(
        row.proof_target(),
        S8HazardProofTarget::Runtime(S8RuntimeProofTarget::StaleIndexCannotReachExactAccess)
    );
}

#[test]
fn compaction_handoff_preserves_the_physical_isolation_owner_contract() {
    let handoff = crate::layout_closeout::layout_closeout()
        .admit_s9_layout_handoff()
        .unwrap();
    let compaction_contract =
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::CompactionCutover);
    assert!(compaction_contract.owner_families().len() > 1);
    let owner_count = handoff.compaction_cutover_owner_transitions().count();
    let projected = handoff
        .machine_contract(S9LayoutStateMachine::CompactionCutover)
        .unwrap();
    assert!(owner_count > 0);
    assert!(super::compaction_cutover::owner_contract_is_preserved(
        &projected
    ));
    assert!(projected.permits_edge(
        State::CompactionRewriteLowered,
        Transition::AdmitTombstoneRetention,
        State::CompactionTombstoneRetentionAdmitted,
    ));
    assert!(projected.permits_edge(
        State::CompactionTombstoneRetentionAdmitted,
        Transition::Publish,
        State::CompactionPublicationCommitted,
    ));
    assert!(projected.permits_edge(
        State::CompactionReclaimDeferred,
        Transition::Reclaim,
        State::CompactionReclaimed,
    ));
    assert!(projected.permits_edge(
        State::CompactionReclaimDeferred,
        Transition::Deny,
        State::Denied,
    ));
}
