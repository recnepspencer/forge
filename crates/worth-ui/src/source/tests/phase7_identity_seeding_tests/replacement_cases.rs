use crate::source::{
    WorthUiArtifactIdentitySeedKind, WorthUiIdentityReplacementClass,
    WorthUiIdentityReplacementClassifier,
};

use super::identity_fixture_support::{
    assert_duplicate_authored_identity_report,
    assert_multi_family_duplicate_authored_identity_report, component_node,
    duplicate_component_authored_identity_modules,
    duplicate_surface_binding_and_token_identity_modules, identity_seeded_from_modules,
    identity_seeding_report_from_modules, primary_only_component_body_atoms,
    standard_component_body_atoms, structural_component_module,
};

#[test]
fn meaningful_identity_change_is_classified_as_replacement() {
    let (baseline, _) =
        identity_seeded_from_modules([
            structural_component_module(standard_component_body_atoms()),
        ]);
    let (changed, _) = identity_seeded_from_modules([structural_component_module(
        primary_only_component_body_atoms(),
    )]);

    let baseline_component = component_node(&baseline, "workspace.component.dashboard");
    let changed_component = component_node(&changed, "workspace.component.dashboard");

    assert_eq!(
        baseline_component.identity_seed().kind(),
        &WorthUiArtifactIdentitySeedKind::StructuralFallback
    );
    assert_eq!(
        changed_component.identity_seed().kind(),
        &WorthUiArtifactIdentitySeedKind::StructuralFallback
    );
    assert_ne!(
        baseline_component.identity_seed().basis(),
        changed_component.identity_seed().basis()
    );
    assert_eq!(
        WorthUiIdentityReplacementClassifier::classify(
            baseline_component.identity_seed(),
            changed_component.identity_seed(),
        ),
        WorthUiIdentityReplacementClass::Replacement
    );
}

#[test]
fn duplicate_authored_component_identities_are_rejected_at_identity_boundary() {
    let report =
        identity_seeding_report_from_modules(duplicate_component_authored_identity_modules());

    assert_duplicate_authored_identity_report(
        &report,
        "component:workspace.component.inspector_panel",
        "component:workspace.component.dashboard",
    );
}

#[test]
fn duplicate_authored_surface_binding_and_token_ids_report_deterministically() {
    let report = identity_seeding_report_from_modules(
        duplicate_surface_binding_and_token_identity_modules(),
    );

    assert_multi_family_duplicate_authored_identity_report(&report);
}
