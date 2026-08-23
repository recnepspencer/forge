const EXACT_SOURCE_EXCLUSIONS: &[&str] = &[
    "crates/worth-ui-certification/src/topology/milestone_3101_inventory/phase8_closeout.rs",
    "crates/worth-ui-certification/src/topology/milestone_3101_inventory/phase8_closeout_tests.rs",
    "crates/worth-ui-certification/src/topology/milestone_3101_inventory/phase7_historical_source_scope.rs",
    "crates/worth-ui-certification/src/intent_execution_provider.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/scaled_canvas.rs",
    "crates/worth-ui-certification/src/scenario/phase5_locality_matrix.rs",
];

const SUCCESSOR_MILESTONE_SOURCE_PREFIXES: &[&str] = &[
    "crates/worth-ui-certification/src/scenario/application_authority_closure/platform_pulse_application.rs",
    "crates/worth-ui-certification/src/scenario/application_authority_closure/visual_identity_application.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/platform_pulse.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/authored_identity.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/post_classification_cost.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/rebind_profile.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/scaled_canvas.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/visual_identity.rs",
    "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle/visual_inspection.rs",
    "crates/worth-ui-certification/src/scenario/phase5_locality_matrix/",
    "crates/worth-ui-certification/src/topology/inspection_topology_audit/",
    "crates/worth-ui-certification/src/topology/milestone_3102_pulse_seed/",
    "crates/worth-ui-certification/src/topology/milestone_3103_executable_world/",
    "crates/worth-ui-certification/src/topology/milestone_3103_product_contract/",
    "crates/worth-ui-certification/src/topology/milestone_3103_external_world/",
    "crates/worth-ui-certification/src/topology/milestone_3103_watched_replacement/",
    "crates/worth-ui-certification/src/topology/milestone_3103_cost_closure/",
];

pub(super) fn belongs_to_phase7_inventory(path: &str) -> bool {
    !EXACT_SOURCE_EXCLUSIONS.contains(&path)
        && !SUCCESSOR_MILESTONE_SOURCE_PREFIXES
            .iter()
            .any(|prefix| path.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::belongs_to_phase7_inventory;

    #[test]
    fn successor_locality_sources_do_not_enlarge_frozen_phase7_history() {
        assert!(!belongs_to_phase7_inventory(
            "crates/worth-ui-certification/src/scenario/phase5_locality_matrix.rs"
        ));
        assert!(!belongs_to_phase7_inventory(
            "crates/worth-ui-certification/src/scenario/phase5_locality_matrix/oracle.rs"
        ));
        assert!(belongs_to_phase7_inventory(
            "crates/worth-ui-certification/src/scenario/filesystem_application_lifecycle.rs"
        ));
    }
}
