use super::scenario::{all_s8_layout_scenarios, S8LayoutScenarioKind};
use super::transcripts::S8LayoutTranscriptKind;

const EXPECTED_S8_LAYOUT_SCENARIOS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8LayoutScenarioInventoryDenial {
    MissingScenario,
    DuplicateScenario,
    MissingProductionApis,
    MissingActors,
    MissingFaults,
    MissingObservers,
    MissingOracles,
    MissingCoverageRows,
    MissingShortcutDenials,
    MissingTransitions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8LayoutScenarioInventoryReceipt {
    scenario_count: usize,
    production_api_bindings: usize,
    actor_bindings: usize,
    fault_bindings: usize,
    observer_bindings: usize,
    oracle_bindings: usize,
    coverage_rows: usize,
    shortcut_denials: usize,
    transition_bindings: usize,
    replay_bundle_scenarios: usize,
    shortcut_trace_scenarios: usize,
}

pub fn verify_canonical_s8_layout_scenario_inventory(
) -> Result<S8LayoutScenarioInventoryReceipt, S8LayoutScenarioInventoryDenial> {
    let scenarios = all_s8_layout_scenarios();
    if scenarios.len() != EXPECTED_S8_LAYOUT_SCENARIOS {
        return Err(S8LayoutScenarioInventoryDenial::MissingScenario);
    }

    let mut seen = Vec::with_capacity(EXPECTED_S8_LAYOUT_SCENARIOS);
    let mut receipt = S8LayoutScenarioInventoryReceipt {
        scenario_count: 0,
        production_api_bindings: 0,
        actor_bindings: 0,
        fault_bindings: 0,
        observer_bindings: 0,
        oracle_bindings: 0,
        coverage_rows: 0,
        shortcut_denials: 0,
        transition_bindings: 0,
        replay_bundle_scenarios: 0,
        shortcut_trace_scenarios: 0,
    };

    for scenario in scenarios {
        if seen.contains(&scenario.kind()) {
            return Err(S8LayoutScenarioInventoryDenial::DuplicateScenario);
        }
        seen.push(scenario.kind());
        require_non_empty(
            scenario.production_apis(),
            S8LayoutScenarioInventoryDenial::MissingProductionApis,
        )?;
        require_non_empty(
            scenario.actors(),
            S8LayoutScenarioInventoryDenial::MissingActors,
        )?;
        require_non_empty(
            scenario.faults(),
            S8LayoutScenarioInventoryDenial::MissingFaults,
        )?;
        require_non_empty(
            scenario.observers(),
            S8LayoutScenarioInventoryDenial::MissingObservers,
        )?;
        require_non_empty(
            scenario.oracles(),
            S8LayoutScenarioInventoryDenial::MissingOracles,
        )?;
        require_non_empty(
            scenario.coverage(),
            S8LayoutScenarioInventoryDenial::MissingCoverageRows,
        )?;
        require_non_empty(
            scenario.shortcut_denials(),
            S8LayoutScenarioInventoryDenial::MissingShortcutDenials,
        )?;
        require_non_empty(
            scenario.transitions(),
            S8LayoutScenarioInventoryDenial::MissingTransitions,
        )?;

        receipt.scenario_count += 1;
        receipt.production_api_bindings += scenario.production_apis().len();
        receipt.actor_bindings += scenario.actors().len();
        receipt.fault_bindings += scenario.faults().len();
        receipt.observer_bindings += scenario.observers().len();
        receipt.oracle_bindings += scenario.oracles().len();
        receipt.coverage_rows += scenario.coverage().len();
        receipt.shortcut_denials += scenario.shortcut_denials().len();
        receipt.transition_bindings += scenario.transitions().len();
        if scenario.transcript() == S8LayoutTranscriptKind::ReplayBundle {
            receipt.replay_bundle_scenarios += 1;
        }
        if scenario.transcript() == S8LayoutTranscriptKind::ShortcutDenialTrace {
            receipt.shortcut_trace_scenarios += 1;
        }
    }

    require_known_scenario_set(&seen)?;
    Ok(receipt)
}

impl S8LayoutScenarioInventoryReceipt {
    pub const fn scenario_count(&self) -> usize {
        self.scenario_count
    }
    pub const fn production_api_bindings(&self) -> usize {
        self.production_api_bindings
    }
    pub const fn actor_bindings(&self) -> usize {
        self.actor_bindings
    }
    pub const fn fault_bindings(&self) -> usize {
        self.fault_bindings
    }
    pub const fn observer_bindings(&self) -> usize {
        self.observer_bindings
    }
    pub const fn oracle_bindings(&self) -> usize {
        self.oracle_bindings
    }
    pub const fn coverage_rows(&self) -> usize {
        self.coverage_rows
    }
    pub const fn shortcut_denials(&self) -> usize {
        self.shortcut_denials
    }
    pub const fn transition_bindings(&self) -> usize {
        self.transition_bindings
    }
    pub const fn replay_bundle_scenarios(&self) -> usize {
        self.replay_bundle_scenarios
    }
    pub const fn shortcut_trace_scenarios(&self) -> usize {
        self.shortcut_trace_scenarios
    }
}

fn require_non_empty<T>(
    values: &[T],
    denial: S8LayoutScenarioInventoryDenial,
) -> Result<(), S8LayoutScenarioInventoryDenial> {
    if values.is_empty() {
        return Err(denial);
    }
    Ok(())
}

fn require_known_scenario_set(
    seen: &[S8LayoutScenarioKind],
) -> Result<(), S8LayoutScenarioInventoryDenial> {
    for required in [
        S8LayoutScenarioKind::LayoutDeclarationInventory,
        S8LayoutScenarioKind::AccessShapeDenial,
        S8LayoutScenarioKind::BroadScanRejection,
        S8LayoutScenarioKind::ExactCounter,
        S8LayoutScenarioKind::CorruptionRebuildParity,
        S8LayoutScenarioKind::MigrationRollbackInterruption,
        S8LayoutScenarioKind::TrustBoundaryReadmission,
        S8LayoutScenarioKind::MultiArtifactIntegration,
    ] {
        if !seen.contains(&required) {
            return Err(S8LayoutScenarioInventoryDenial::MissingScenario);
        }
    }
    Ok(())
}
