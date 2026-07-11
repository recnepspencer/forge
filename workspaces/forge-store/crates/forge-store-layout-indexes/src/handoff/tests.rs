use super::*;
use crate::layout_closeout::layout_closeout;

#[test]
fn canonical_inventory_maps_each_required_hazard_to_a_real_proof_lane() {
    let inventory = S8LayoutHazardInventory::canonical();
    assert_eq!(inventory.rows().len(), 14);
    assert!(inventory.rows().iter().all(|row| matches!(
        row.proof_lane(),
        S8HazardProofLane::CompileFail
            | S8HazardProofLane::Runtime
            | S8HazardProofLane::Simulation
            | S8HazardProofLane::FormalModel
    )));
    assert!(inventory.rows().iter().all(|row| {
        S9LayoutMachineContract::for_machine(row.machine()).permits_edge(
            row.transition_from(),
            row.transition(),
            row.transition_to(),
        ) && matches!(
            (row.proof_target(), row.proof_lane()),
            (
                S8HazardProofTarget::CompileFail(_),
                S8HazardProofLane::CompileFail
            ) | (S8HazardProofTarget::Runtime(_), S8HazardProofLane::Runtime)
                | (
                    S8HazardProofTarget::FormalModel(_),
                    S8HazardProofLane::FormalModel
                )
        )
    }));
    for lane in [
        S8HazardProofLane::CompileFail,
        S8HazardProofLane::Runtime,
        S8HazardProofLane::FormalModel,
    ] {
        assert!(inventory.rows().iter().any(|row| row.proof_lane() == lane));
    }
}

#[test]
fn compile_fail_targets_name_sources_executed_by_the_real_ui_harness() {
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    for row in S8LayoutHazardInventory::canonical().rows() {
        let S8HazardProofTarget::CompileFail(target) = row.proof_target() else {
            continue;
        };
        assert!(
            manifest.join(target.fixture()).is_file(),
            "{}",
            target.fixture()
        );
    }
}

#[test]
fn runtime_targets_name_owner_operations_without_source_path_proxies() {
    for row in S8LayoutHazardInventory::canonical().rows() {
        let S8HazardProofTarget::Runtime(target) = row.proof_target() else {
            continue;
        };
        assert_eq!(
            target.owner(),
            if target == S8RuntimeProofTarget::HiddenScanDeniedWithOwnerCounters {
                S8RuntimeProofOwner::PhysicalFormat
            } else {
                S8RuntimeProofOwner::LayoutIndexes
            }
        );
    }
}

#[test]
fn handoff_preserves_machine_specific_hazard_transition_obligations() {
    let row = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.machine() == S9LayoutStateMachine::FullDeclaredScanAdmission)
        .unwrap();
    assert_eq!(row.hazard(), S8LayoutHazard::HiddenBroadScan);
    assert_eq!(row.transition(), S9LayoutMachineTransition::Deny);
    assert_eq!(
        row.evidence_requirement(),
        S8HazardEvidenceRequirement::OwnerBoundExactCounters
    );
}

#[test]
fn counter_bound_hazard_cannot_be_reduced_to_a_generic_simulation_requirement() {
    let hidden_scan = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.hazard() == S8LayoutHazard::HiddenBroadScan)
        .unwrap();
    assert_ne!(
        hidden_scan.evidence_requirement(),
        S8HazardEvidenceRequirement::SimulationOracle
    );
}

#[test]
fn hidden_scan_denial_is_an_owner_executed_runtime_lane() {
    let row = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.hazard() == S8LayoutHazard::HiddenBroadScan)
        .unwrap();
    assert_eq!(row.proof_lane(), S8HazardProofLane::Runtime);
    assert_eq!(row.detection(), S8HazardDetection::ExecutedCounterBoundary);
}

#[test]
fn cross_scope_index_uses_the_store_scope_admission_runtime_lane() {
    let row = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.hazard() == S8LayoutHazard::CrossScopeIndex)
        .unwrap();
    assert_eq!(row.proof_lane(), S8HazardProofLane::Runtime);
    assert_eq!(row.detection(), S8HazardDetection::RuntimeOutcome);
    assert_eq!(
        row.evidence_requirement(),
        S8HazardEvidenceRequirement::OwnerRuntimeOutcome
    );
    assert_eq!(
        row.proof_target(),
        S8HazardProofTarget::Runtime(S8RuntimeProofTarget::CrossScopeIndexDeniedByScopeAdmission)
    );
}

#[test]
fn cross_scope_runtime_target_executes_layout_admission_denial() {
    execute_cross_scope_runtime_denial();
}

pub(super) fn execute_cross_scope_runtime_denial() {
    use crate::strategy::tests_support::{admit_phase_five_scope, root_manifest_scope};
    use crate::strategy::S8LayoutStrategyFamily;
    use crate::strategy_registry::{
        layout_admission_registry, S8LayoutAdmissionDenial, S8LayoutAdmissionRequest,
        S8LayoutRequestedCapability,
    };
    use crate::ArtifactFamilyAccessLane;
    use forge_proof::TransitionOutcome;
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
    let (_, root_domain) = root_manifest_scope();
    let request = S8LayoutAdmissionRequest::new(
        page_lifecycle,
        page_domain,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutRequestedCapability::point_lookup(),
        ArtifactFamilyAccessLane::HotPath,
    )
    .within_scope_partition(root_domain.scope());
    let outcome = layout_admission_registry().admit(request);
    assert!(
        S9LayoutMachineContract::for_machine(S9LayoutStateMachine::LayoutAdmission)
            .contains(outcome.production_transition())
    );
    assert_eq!(
        outcome.unwrap_err(),
        S8LayoutAdmissionDenial::RequestedScopeDoesNotMatchKeyDomain {
            requested_scope: root_domain.scope(),
            key_domain_scope: page_domain.scope(),
        }
    );
}

#[test]
fn hidden_scan_runtime_target_executes_the_owner_denial_operation() {
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
    assert_eq!(
        receipt.counters().full_store_materialization_rejections(),
        1
    );
    assert_eq!(receipt.counters().scans(), 0);
    assert_eq!(receipt.fact().counters().planned_units(), 0);
    assert_eq!(receipt.fact().counters().observed_units(), 0);
}

#[test]
fn lsm_tombstone_hazard_hands_off_to_the_s9_compaction_model() {
    let row = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.hazard() == S8LayoutHazard::LsmTombstoneLoss)
        .unwrap();
    assert_eq!(row.proof_lane(), S8HazardProofLane::FormalModel);
    assert_eq!(
        row.proof_target(),
        S8HazardProofTarget::FormalModel(S9FormalModelTarget::CompactionCutover)
    );
    assert!(layout_closeout()
        .admit_s9_layout_handoff()
        .unwrap()
        .declares_pending_protocol_target(S9FormalModelTarget::CompactionCutover));
    let handoff = layout_closeout().admit_s9_layout_handoff().unwrap();
    let owner_transitions: Vec<_> = handoff.compaction_cutover_owner_transitions().collect();
    assert!(owner_transitions.iter().any(|edge| {
        edge.kind()
            == forge_store_physical_isolation::CompactionCutoverTransitionKind::AdmitLsmTombstoneRetention
    }));
    assert!(owner_transitions.iter().any(|edge| {
        edge.kind()
            == forge_store_physical_isolation::CompactionCutoverTransitionKind::DenyEarlyReclaim
    }));
}

#[test]
fn s9_handoff_carries_every_required_state_machine() {
    let handoff = layout_closeout().admit_s9_layout_handoff().unwrap();
    assert!(S9_REQUIRED_LAYOUT_MACHINES
        .into_iter()
        .all(|machine| handoff.requires(machine)));
}

#[test]
fn every_s9_machine_has_named_transition_obligations() {
    for (index, machine) in S9_REQUIRED_LAYOUT_MACHINES.into_iter().enumerate() {
        let contract = S9LayoutMachineContract::for_machine(machine);
        assert_eq!(contract.machine(), machine);
        assert!(!contract.transitions().is_empty());
        assert!(contract.transitions().iter().all(|transition| {
            let edge = transition.edge();
            edge.from() != edge.to()
                || (edge.from() == S9LayoutMachineState::Stale
                    && edge.transition() == S9LayoutMachineTransition::Readmit)
        }));
        for other in S9_REQUIRED_LAYOUT_MACHINES.into_iter().skip(index + 1) {
            assert_ne!(
                contract.production_operation(),
                S9LayoutMachineContract::for_machine(other).production_operation()
            );
        }
    }
}

#[test]
fn s9_handoff_aggregates_complete_owner_transition_contracts() {
    let handoff = layout_closeout().admit_s9_layout_handoff().unwrap();
    let machines = handoff.machine_inventory();
    assert!(machines.is_complete());
    assert_eq!(
        machines.contracts().len(),
        S9_REQUIRED_LAYOUT_MACHINES.len()
    );
    for target in S9_DOWNSTREAM_PROTOCOL_DESTINATIONS {
        assert!(handoff.declares_pending_protocol_target(target));
    }
    assert!(handoff
        .downstream_protocol_targets()
        .declares_all_destinations());
    assert!(handoff
        .downstream_protocol_targets()
        .rows()
        .iter()
        .all(|row| !row.expected_owners().is_empty()));
}

#[test]
fn canonical_inventory_names_the_pending_compaction_formal_risk_only() {
    for row in S8LayoutHazardInventory::canonical().rows() {
        if row.hazard() == S8LayoutHazard::LsmTombstoneLoss {
            assert!(matches!(
                row.residual_risk(),
                S8HazardResidualRisk::ExplicitlyContained { .. }
            ));
        } else {
            assert_eq!(row.residual_risk(), S8HazardResidualRisk::None);
        }
    }
}

#[test]
fn degraded_exact_scan_has_explicit_budget_and_exact_counter_obligations() {
    let contract = S9LayoutMachineContract::for_machine(S9LayoutStateMachine::DegradedExactScan);
    let transitions = contract.transitions();
    assert!(transitions
        .iter()
        .any(|artifact| artifact.edge().transition() == S9LayoutMachineTransition::Budget));
    assert!(transitions.iter().any(|artifact| {
        let edge = artifact.edge();
        edge.from() == S9LayoutMachineState::Ready
            && edge.transition() == S9LayoutMachineTransition::AdmitExactCounters
            && edge.to() == S9LayoutMachineState::ExactCountersObserved
    }));
    assert!(transitions.iter().any(|artifact| {
        let edge = artifact.edge();
        edge.from() == S9LayoutMachineState::ExactCountersObserved
            && edge.transition() == S9LayoutMachineTransition::Execute
            && edge.to() == S9LayoutMachineState::Executed
    }));
    let row = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.hazard() == S8LayoutHazard::DegradedExactScanWithoutBudget)
        .expect("degraded exact scans must have a named S.9 hazard");
    assert_eq!(row.machine(), S9LayoutStateMachine::DegradedExactScan);
    assert_eq!(row.proof_lane(), S8HazardProofLane::Runtime);
    assert_eq!(
        row.evidence_requirement(),
        S8HazardEvidenceRequirement::OwnerRuntimeOutcome
    );
}

#[test]
fn degraded_exact_scan_is_not_allowed_to_claim_execution_without_a_family_executor() {
    let row = S8LayoutHazardInventory::canonical()
        .rows()
        .iter()
        .find(|row| row.hazard() == S8LayoutHazard::DegradedExactScanWithoutBudget)
        .unwrap();
    assert_eq!(
        row.proof_target(),
        S8HazardProofTarget::Runtime(S8RuntimeProofTarget::UnbudgetedDegradedScanDenied)
    );
}

#[test]
fn degraded_machine_reaches_execution_only_through_the_real_budgeted_counter_path() {
    let executed = crate::execution::tests_support::execute_budgeted_degraded_exact_scan();
    assert_eq!(
        executed.selected().selected_family(),
        crate::S8LayoutStrategyFamily::ExactScan
    );
    assert_eq!(
        executed.observed(),
        executed.selected().planned_counter_envelope().lookup()
    );
}

#[test]
fn degraded_scan_runtime_target_executes_the_unbudgeted_denial() {
    execute_unbudgeted_degraded_denial();
}

pub(super) fn execute_unbudgeted_degraded_denial() {
    use crate::access_shape::{
        access_shapes, S8AccessShapeUnsupportedDenial, S8DegradedExactScanRequest,
    };
    use crate::facade::{access_planning, layout_declarations};
    use forge_store_physical_format::PhysicalEpoch;

    let family = layout_declarations().seed_family();
    let coverage = access_planning()
        .exact_root_epoch_coverage(
            crate::bootstrap::test_support::bootstrap_exact_materialization(family.family()),
            PhysicalEpoch::from_raw(34).unwrap(),
        )
        .unwrap();
    assert_eq!(
        access_shapes().explicit_degraded_exact_scan(S8DegradedExactScanRequest::new(coverage)),
        Err(S8AccessShapeUnsupportedDenial::DegradedExactScanBudgetRequired)
    );
}

#[test]
fn s9_handoff_admits_complete_store_grammar_not_a_generic_summary() {
    let handoff = layout_closeout()
        .admit_s9_layout_handoff()
        .expect("complete grammar should admit S.9 handoff");

    assert!(handoff.requires(S9LayoutStateMachine::FullDeclaredScanAdmission));
    assert!(handoff.requires(S9LayoutStateMachine::DegradedExactScan));
}
