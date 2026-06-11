use crate::source::{WorthUiIdentityReplacementClass, WorthUiIdentityReplacementClassifier};

use super::identity_fixture_support::{
    assert_authored_seed_kind, assert_durable_eligible_count, assert_ineligible_reason,
    authored_component_module, binding_node, component_node, identity_seeded_from_modules,
    import_node, imported_identity_modules, reordered_imported_identity_modules, surface_node,
    token_node,
};

#[test]
fn same_authored_identity_and_same_structure_produce_same_identity_seed() {
    let modules = [authored_component_module("dashboard.root")];

    let (left, left_metrics) = identity_seeded_from_modules(modules.clone());
    let (right, right_metrics) = identity_seeded_from_modules(modules);

    let left_component = component_node(&left, "workspace.component.dashboard");
    let right_component = component_node(&right, "workspace.component.dashboard");

    assert_eq!(
        left_component.identity_seed(),
        right_component.identity_seed()
    );
    assert_authored_seed_kind(left_component);
    assert_durable_eligible_count(left_component.durable_state_eligibility(), 3);
    assert_eq!(left_metrics.node_count_seeded(), 1);
    assert_eq!(left_metrics.authored_seed_count(), 1);
    assert_eq!(left_metrics.structural_fallback_count(), 0);
    assert_eq!(left_metrics.durable_state_eligible_count(), 1);
    assert_eq!(left_metrics, right_metrics);
}

#[test]
fn identity_seed_is_not_file_order_folklore() {
    let (left, left_metrics) = identity_seeded_from_modules(imported_identity_modules());
    let (right, right_metrics) =
        identity_seeded_from_modules(reordered_imported_identity_modules());

    assert!(left.equivalent_shape(&right));
    assert_eq!(
        import_node(&left, "app/panels/inspector.wui").identity_seed(),
        import_node(&right, "app/panels/inspector.wui").identity_seed()
    );
    assert_eq!(
        surface_node(&left, "workspace.surface.inspector").identity_seed(),
        surface_node(&right, "workspace.surface.inspector").identity_seed()
    );
    assert_eq!(
        binding_node(&left, "workspace.view_binding.selection").identity_seed(),
        binding_node(&right, "workspace.view_binding.selection").identity_seed()
    );
    assert_eq!(
        token_node(&left, "theme.text.default").identity_seed(),
        token_node(&right, "theme.text.default").identity_seed()
    );
    assert_eq!(left_metrics, right_metrics);
}

#[test]
fn durable_state_eligibility_is_explicit_and_conservative() {
    let (identity_seeded, _) = identity_seeded_from_modules(imported_identity_modules());

    assert_ineligible_reason(
        surface_node(&identity_seeded, "workspace.surface.inspector").durable_state_eligibility(),
        crate::source::WorthUiDurableStateIneligibilityReason::NoRestorableStateSlots,
    );
    assert_ineligible_reason(
        binding_node(&identity_seeded, "workspace.view_binding.selection")
            .durable_state_eligibility(),
        crate::source::WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
    );
    assert_ineligible_reason(
        token_node(&identity_seeded, "theme.text.default").durable_state_eligibility(),
        crate::source::WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
    );
}

#[test]
fn structural_identity_change_is_classified_as_replacement() {
    let (baseline, _) = identity_seeded_from_modules([authored_component_module("dashboard.root")]);
    let (replayed, _) = identity_seeded_from_modules([authored_component_module("dashboard.root")]);

    let baseline_seed = component_node(&baseline, "workspace.component.dashboard").identity_seed();
    let replayed_seed = component_node(&replayed, "workspace.component.dashboard").identity_seed();

    assert_eq!(
        WorthUiIdentityReplacementClassifier::classify(baseline_seed, replayed_seed),
        WorthUiIdentityReplacementClass::CarryForward
    );
}
