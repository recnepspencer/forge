use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::CertificationSuiteDeclaration;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
pub enum ScenarioProcessTopology {
    InProcessLibtest,
    FreshChildProcess,
    NestedCargoProcess,
    AllocatorGlobalProcess,
}

pub(super) fn admitted_process_topologies(suite_name: &str) -> BTreeSet<ScenarioProcessTopology> {
    let mut admitted = BTreeSet::from([ScenarioProcessTopology::InProcessLibtest]);
    if suite_name == "durability_recovery" {
        admitted.extend([
            ScenarioProcessTopology::FreshChildProcess,
            ScenarioProcessTopology::NestedCargoProcess,
        ]);
    }
    admitted
}

pub fn validate_suite_process_cohesion(
    suites: &[CertificationSuiteDeclaration],
) -> Result<(), Vec<String>> {
    let mut violations = Vec::new();
    for suite in suites {
        for scenario in &suite.scenarios {
            if !suite
                .admitted_process_topologies
                .contains(&scenario.process_topology)
            {
                violations.push(format!(
                    "suite {} does not admit {:?} for scenario {}",
                    suite.suite_identity,
                    scenario.process_topology,
                    scenario.identity.responsibility
                ));
            }
        }
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}
