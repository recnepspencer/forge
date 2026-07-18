use std::collections::BTreeMap;

use crate::classification::{ConsolidatedSuiteInventory, ScenarioProcessTopology};

#[derive(Clone)]
pub(super) struct DeclaredScenarioExecution {
    pub(super) filter: String,
    pub(super) process_model: String,
}

pub(super) fn declared_suite_filters(
    inventory: &ConsolidatedSuiteInventory,
) -> BTreeMap<String, BTreeMap<String, DeclaredScenarioExecution>> {
    inventory
        .suites
        .iter()
        .map(|suite| {
            let scenarios = suite
                .scenarios
                .iter()
                .map(|scenario| {
                    (
                        scenario.identity.responsibility.clone(),
                        DeclaredScenarioExecution {
                            filter: scenario.libtest_filter_prefix.clone(),
                            process_model: scenario_process_model(scenario.process_topology)
                                .to_owned(),
                        },
                    )
                })
                .collect();
            (suite.target_identity.clone(), scenarios)
        })
        .collect()
}

pub(super) fn suite_case_responsibilities(
    suites: &ConsolidatedSuiteInventory,
) -> BTreeMap<String, String> {
    suites
        .suites
        .iter()
        .flat_map(|suite| &suite.scenarios)
        .flat_map(|scenario| {
            scenario.case_identities.iter().map(|identity| {
                (
                    identity.stable_id.clone(),
                    scenario.identity.responsibility.clone(),
                )
            })
        })
        .collect()
}

fn scenario_process_model(topology: ScenarioProcessTopology) -> &'static str {
    match topology {
        ScenarioProcessTopology::InProcessLibtest => "libtest-process",
        ScenarioProcessTopology::FreshChildProcess => "libtest-with-fresh-child-process",
        ScenarioProcessTopology::NestedCargoProcess => "libtest-with-nested-cargo-process",
        ScenarioProcessTopology::AllocatorGlobalProcess => "allocator-global-process",
    }
}
