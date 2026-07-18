use std::collections::BTreeMap;

use super::ConsolidatedSuiteInventory;

pub fn validate_suite_semantic_authority(
    authority: &ConsolidatedSuiteInventory,
    observed: &ConsolidatedSuiteInventory,
) -> Result<(), Vec<String>> {
    let declared = scenario_map(authority);
    let current = scenario_map(observed);
    let mut violations = Vec::new();
    if authority.schema_version != 2 || observed.schema_version != 2 {
        violations.push(format!(
            "unsupported scenario authority schema: declared={}, observed={}",
            authority.schema_version, observed.schema_version
        ));
    }
    if authority.pre_cleanup_scenario_executables != observed.pre_cleanup_scenario_executables
        || authority.consolidated_suite_executables != observed.consolidated_suite_executables
    {
        violations.push("scenario executable cardinality drifted from sealed authority".to_owned());
    }
    for suite in &observed.suites {
        let Some(expected) = authority
            .suites
            .iter()
            .find(|expected| expected.suite_identity == suite.suite_identity)
        else {
            violations.push(format!(
                "suite has no sealed semantic authority: {}",
                suite.suite_identity
            ));
            continue;
        };
        if suite.target_identity != expected.target_identity
            || suite.responsibility_boundary != expected.responsibility_boundary
            || suite.admitted_process_topologies != expected.admitted_process_topologies
            || suite.shared_support_sources != expected.shared_support_sources
            || suite.suite_source_fingerprints != expected.suite_source_fingerprints
        {
            violations.push(format!(
                "suite boundary drifted from sealed authority: {}",
                suite.suite_identity
            ));
        }
    }
    for missing in authority.suites.iter().filter(|expected| {
        !observed
            .suites
            .iter()
            .any(|suite| suite.suite_identity == expected.suite_identity)
    }) {
        violations.push(format!(
            "sealed suite is no longer reachable: {}",
            missing.suite_identity
        ));
    }
    for (identity, scenario) in &current {
        let Some(expected) = declared.get(identity) else {
            violations.push(format!(
                "scenario has no sealed semantic authority: {}::{}",
                scenario.identity.owner_package, scenario.identity.responsibility
            ));
            continue;
        };
        if scenario != expected {
            violations.push(format!(
                "scenario contract drifted from sealed authority: {}::{}",
                scenario.identity.owner_package, scenario.identity.responsibility
            ));
        }
        if scenario
            .proof_contract
            .production_subject_packages
            .is_empty()
            || scenario.proof_contract.oracle_owner_packages.is_empty()
            || scenario.proof_contract.setup_authority_sources.is_empty()
        {
            violations.push(format!(
                "scenario contract omits subject, setup, or oracle authority: {}::{}",
                scenario.identity.owner_package, scenario.identity.responsibility
            ));
        }
    }
    for (_, missing) in declared
        .iter()
        .filter(|(identity, _)| !current.contains_key(identity))
    {
        violations.push(format!(
            "sealed scenario is no longer reachable: {}::{}",
            missing.identity.owner_package, missing.identity.responsibility
        ));
    }
    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

pub fn validate_suite_semantic_authority_for_source_edit(
    authority: &ConsolidatedSuiteInventory,
    observed: &ConsolidatedSuiteInventory,
    source_path: &str,
) -> Result<(), Vec<String>> {
    let mut projected = observed.clone();
    let mut admitted_fingerprints = 0;
    for suite in &mut projected.suites {
        let Some(expected) = authority
            .suites
            .iter()
            .find(|candidate| candidate.suite_identity == suite.suite_identity)
        else {
            continue;
        };
        for (path, fingerprint) in &mut suite.suite_source_fingerprints {
            if !source_matches(path, source_path) {
                continue;
            }
            let Some((_, sealed)) = expected
                .suite_source_fingerprints
                .iter()
                .find(|(candidate, _)| source_matches(candidate, source_path))
            else {
                continue;
            };
            *fingerprint = sealed.clone();
            admitted_fingerprints += 1;
        }
    }
    if admitted_fingerprints == 0 {
        return Err(vec![format!(
            "declared certification source edit has no suite fingerprint: {source_path}"
        )]);
    }
    validate_suite_semantic_authority(authority, &projected)
}

fn scenario_map(
    inventory: &ConsolidatedSuiteInventory,
) -> BTreeMap<(&str, &super::ScenarioIdentity), &super::CertificationScenarioDeclaration> {
    inventory
        .suites
        .iter()
        .flat_map(|suite| {
            suite.scenarios.iter().map(move |scenario| {
                (
                    (suite.suite_identity.as_str(), &scenario.identity),
                    scenario,
                )
            })
        })
        .collect()
}

fn source_matches(observed: &str, declared: &str) -> bool {
    observed.replace('\\', "/").ends_with(declared)
}
