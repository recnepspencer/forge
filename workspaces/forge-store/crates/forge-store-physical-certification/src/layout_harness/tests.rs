use super::closeout::S8LayoutCloseoutEvidenceLane;
use super::scenario::{
    all_layout_index_layout_scenarios, canonical_layout_index_layout_production_apis,
    canonical_layout_index_layout_required_transitions, canonical_layout_index_layout_shortcut_denials,
    canonical_layout_index_layout_supported_scenarios, layout_scenario, S8LayoutProductionApi,
    S8LayoutScenarioKind, S8LayoutTransitionState,
};
use super::scenario_inventory::verify_canonical_layout_index_layout_scenario_inventory;
use super::shortcut_denials::S8LayoutShortcutDenialKind;
use super::transcripts::S8LayoutTranscriptKind;

#[test]
fn canonical_layout_index_layout_aggregate_vocabulary_matches_scenario_inventory() {
    assert_eq!(
        canonical_layout_index_layout_supported_scenarios(),
        &[
            S8LayoutScenarioKind::LayoutDeclarationInventory,
            S8LayoutScenarioKind::AccessShapeDenial,
            S8LayoutScenarioKind::BroadScanRejection,
            S8LayoutScenarioKind::ExactCounter,
            S8LayoutScenarioKind::CorruptionRebuildParity,
            S8LayoutScenarioKind::MigrationRollbackInterruption,
            S8LayoutScenarioKind::TrustBoundaryReadmission,
            S8LayoutScenarioKind::MultiArtifactIntegration,
        ]
    );
    assert_eq!(
        canonical_layout_index_layout_production_apis(),
        &[
            S8LayoutProductionApi::LayoutFamilies,
            S8LayoutProductionApi::LayoutStrategyAdmission,
            S8LayoutProductionApi::AccessPlanning,
            S8LayoutProductionApi::AccessLowering,
            S8LayoutProductionApi::AccessExecution,
            S8LayoutProductionApi::LayoutRebuild,
            S8LayoutProductionApi::LayoutReadmission,
            S8LayoutProductionApi::LayoutMigration,
        ]
    );
    assert_eq!(
        canonical_layout_index_layout_required_transitions(),
        &[
            S8LayoutTransitionState::Declared,
            S8LayoutTransitionState::Admitted,
            S8LayoutTransitionState::Planned,
            S8LayoutTransitionState::Lowered,
            S8LayoutTransitionState::ExecutionReady,
            S8LayoutTransitionState::Executed,
            S8LayoutTransitionState::Rebuilt,
            S8LayoutTransitionState::Readmitted,
            S8LayoutTransitionState::Rebound,
        ]
    );
    assert_eq!(
        canonical_layout_index_layout_shortcut_denials(),
        &[
            S8LayoutShortcutDenialKind::SyntheticFixtureAuthority,
            S8LayoutShortcutDenialKind::BroadScanMasqueradingAsPointLookup,
            S8LayoutShortcutDenialKind::CopiedCounterRows,
            S8LayoutShortcutDenialKind::TerminalProjectionAuthority,
            S8LayoutShortcutDenialKind::FoundationalMaterializationAuthority,
            S8LayoutShortcutDenialKind::LooseLogEvidence,
        ]
    );
}

#[test]
fn every_layout_index_layout_scenario_binds_a_complete_canonical_contract() {
    for scenario in all_layout_index_layout_scenarios() {
        assert!(!scenario.production_apis().is_empty());
        assert!(!scenario.actors().is_empty());
        assert!(!scenario.faults().is_empty());
        assert!(!scenario.observers().is_empty());
        assert!(!scenario.oracles().is_empty());
        assert!(!scenario.coverage().is_empty());
        assert!(!scenario.shortcut_denials().is_empty());
        assert!(!scenario.transitions().is_empty());
    }
}

#[test]
fn canonical_layout_index_layout_inventory_receipt_covers_the_whole_phase_contract() {
    let receipt = verify_canonical_layout_index_layout_scenario_inventory()
        .expect("canonical S.8 layout inventory should verify");
    assert_eq!(receipt.scenario_count(), 8);
    assert_eq!(receipt.coverage_rows(), 8);
    assert!(receipt.production_api_bindings() >= 8);
    assert!(receipt.actor_bindings() >= 8);
    assert!(receipt.fault_bindings() >= 8);
    assert!(receipt.observer_bindings() >= 8);
    assert!(receipt.oracle_bindings() >= 8);
    assert!(receipt.shortcut_denials() >= 8);
    assert!(receipt.transition_bindings() >= 8);
    assert!(receipt.replay_bundle_scenarios() > 0);
    assert!(receipt.shortcut_trace_scenarios() > 0);
}

#[test]
fn layout_index_layout_scenarios_bind_expected_transcripts_and_closeout_lanes() {
    assert_scenario_contract(
        S8LayoutScenarioKind::LayoutDeclarationInventory,
        S8LayoutTranscriptKind::ScenarioTranscript,
        S8LayoutCloseoutEvidenceLane::ScenarioDefinition,
        false,
    );
    assert_scenario_contract(
        S8LayoutScenarioKind::AccessShapeDenial,
        S8LayoutTranscriptKind::ShortcutDenialTrace,
        S8LayoutCloseoutEvidenceLane::ScenarioDefinition,
        false,
    );
    assert_scenario_contract(
        S8LayoutScenarioKind::BroadScanRejection,
        S8LayoutTranscriptKind::ShortcutDenialTrace,
        S8LayoutCloseoutEvidenceLane::PerformanceEvidence,
        false,
    );
    assert_scenario_contract(
        S8LayoutScenarioKind::ExactCounter,
        S8LayoutTranscriptKind::ScenarioTranscript,
        S8LayoutCloseoutEvidenceLane::PerformanceEvidence,
        true,
    );
    assert_scenario_contract(
        S8LayoutScenarioKind::CorruptionRebuildParity,
        S8LayoutTranscriptKind::ReplayBundle,
        S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        true,
    );
    assert_scenario_contract(
        S8LayoutScenarioKind::MigrationRollbackInterruption,
        S8LayoutTranscriptKind::ReplayBundle,
        S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        false,
    );
    assert_scenario_contract(
        S8LayoutScenarioKind::TrustBoundaryReadmission,
        S8LayoutTranscriptKind::ShortcutDenialTrace,
        S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        true,
    );
    assert_scenario_contract(
        S8LayoutScenarioKind::MultiArtifactIntegration,
        S8LayoutTranscriptKind::ReplayBundle,
        S8LayoutCloseoutEvidenceLane::CertificationCloseout,
        true,
    );
}

fn assert_scenario_contract(
    kind: S8LayoutScenarioKind,
    transcript: S8LayoutTranscriptKind,
    closeout: S8LayoutCloseoutEvidenceLane,
    reaches_executed: bool,
) {
    let scenario = layout_scenario(kind);
    assert_eq!(
        scenario.transcript(),
        transcript,
        "{kind:?} transcript drifted"
    );
    assert_eq!(scenario.closeout(), closeout, "{kind:?} closeout drifted");
    assert_eq!(
        scenario
            .transitions()
            .contains(&S8LayoutTransitionState::Executed),
        reaches_executed,
        "{kind:?} executed transition posture drifted"
    );
}
