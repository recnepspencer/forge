use super::closeout::LayoutCloseoutEvidenceLane;
use super::scenario::{
    all_layout_index_layout_scenarios, canonical_layout_index_layout_production_apis,
    canonical_layout_index_layout_required_transitions,
    canonical_layout_index_layout_shortcut_denials,
    canonical_layout_index_layout_supported_scenarios, layout_scenario, LayoutProductionApi,
    LayoutScenarioKind, LayoutTransitionState,
};
use super::scenario_inventory::verify_canonical_layout_index_layout_scenario_inventory;
use super::shortcut_denials::LayoutShortcutDenialKind;
use super::transcripts::LayoutTranscriptKind;

#[test]
fn canonical_layout_index_layout_aggregate_vocabulary_matches_scenario_inventory() {
    assert_eq!(
        canonical_layout_index_layout_supported_scenarios(),
        &[
            LayoutScenarioKind::LayoutDeclarationInventory,
            LayoutScenarioKind::AccessShapeDenial,
            LayoutScenarioKind::BroadScanRejection,
            LayoutScenarioKind::ExactCounter,
            LayoutScenarioKind::CorruptionRebuildParity,
            LayoutScenarioKind::MigrationRollbackInterruption,
            LayoutScenarioKind::TrustBoundaryReadmission,
            LayoutScenarioKind::MultiArtifactIntegration,
        ]
    );
    assert_eq!(
        canonical_layout_index_layout_production_apis(),
        &[
            LayoutProductionApi::LayoutFamilies,
            LayoutProductionApi::LayoutStrategyAdmission,
            LayoutProductionApi::AccessPlanning,
            LayoutProductionApi::AccessLowering,
            LayoutProductionApi::AccessExecution,
            LayoutProductionApi::LayoutRebuild,
            LayoutProductionApi::LayoutReadmission,
            LayoutProductionApi::LayoutMigration,
        ]
    );
    assert_eq!(
        canonical_layout_index_layout_required_transitions(),
        &[
            LayoutTransitionState::Declared,
            LayoutTransitionState::Admitted,
            LayoutTransitionState::Planned,
            LayoutTransitionState::Lowered,
            LayoutTransitionState::ExecutionReady,
            LayoutTransitionState::Executed,
            LayoutTransitionState::Rebuilt,
            LayoutTransitionState::Readmitted,
            LayoutTransitionState::Rebound,
        ]
    );
    assert_eq!(
        canonical_layout_index_layout_shortcut_denials(),
        &[
            LayoutShortcutDenialKind::SyntheticFixtureAuthority,
            LayoutShortcutDenialKind::BroadScanMasqueradingAsPointLookup,
            LayoutShortcutDenialKind::CopiedCounterRows,
            LayoutShortcutDenialKind::TerminalProjectionAuthority,
            LayoutShortcutDenialKind::FoundationalMaterializationAuthority,
            LayoutShortcutDenialKind::LooseLogEvidence,
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
        LayoutScenarioKind::LayoutDeclarationInventory,
        LayoutTranscriptKind::ScenarioTranscript,
        LayoutCloseoutEvidenceLane::ScenarioDefinition,
        false,
    );
    assert_scenario_contract(
        LayoutScenarioKind::AccessShapeDenial,
        LayoutTranscriptKind::ShortcutDenialTrace,
        LayoutCloseoutEvidenceLane::ScenarioDefinition,
        false,
    );
    assert_scenario_contract(
        LayoutScenarioKind::BroadScanRejection,
        LayoutTranscriptKind::ShortcutDenialTrace,
        LayoutCloseoutEvidenceLane::PerformanceEvidence,
        false,
    );
    assert_scenario_contract(
        LayoutScenarioKind::ExactCounter,
        LayoutTranscriptKind::ScenarioTranscript,
        LayoutCloseoutEvidenceLane::PerformanceEvidence,
        true,
    );
    assert_scenario_contract(
        LayoutScenarioKind::CorruptionRebuildParity,
        LayoutTranscriptKind::ReplayBundle,
        LayoutCloseoutEvidenceLane::CertificationCloseout,
        true,
    );
    assert_scenario_contract(
        LayoutScenarioKind::MigrationRollbackInterruption,
        LayoutTranscriptKind::ReplayBundle,
        LayoutCloseoutEvidenceLane::CertificationCloseout,
        false,
    );
    assert_scenario_contract(
        LayoutScenarioKind::TrustBoundaryReadmission,
        LayoutTranscriptKind::ShortcutDenialTrace,
        LayoutCloseoutEvidenceLane::CertificationCloseout,
        true,
    );
    assert_scenario_contract(
        LayoutScenarioKind::MultiArtifactIntegration,
        LayoutTranscriptKind::ReplayBundle,
        LayoutCloseoutEvidenceLane::CertificationCloseout,
        true,
    );
}

fn assert_scenario_contract(
    kind: LayoutScenarioKind,
    transcript: LayoutTranscriptKind,
    closeout: LayoutCloseoutEvidenceLane,
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
            .contains(&LayoutTransitionState::Executed),
        reaches_executed,
        "{kind:?} executed transition posture drifted"
    );
}
