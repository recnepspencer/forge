use crate::scalar_text_projection_fixture::{
    collection_projection_workspace, projection_workspace,
};
use crate::{
    UiCollectionProjectionRegistration, UiProjectionFieldRequirement,
    UiScalarProjectionRegistration, WorthUiQueryBindingPlan,
    WorthUiQueryBindingRegistrationDenialKind, WorthUiQueryWorkspaceExt,
};

#[test]
fn shape_specific_projection_registrations_are_retained_by_identity() {
    let workspace = projection_workspace(true);
    let domain = workspace.worth_ui().expect("Worth UI domain installed");
    let identity = crate::WorthUiQueryViewIdentity::new("platform.pulse.status")
        .expect("valid projection identity");
    let registration = scalar_registration(&domain, identity.as_str());

    let plan = WorthUiQueryBindingPlan::default()
        .register_scalar_projection(registration.clone())
        .expect("scalar projection registration");

    assert_eq!(
        plan.scalar_projection_registration(&identity),
        Some(&registration)
    );
    assert!(plan.collection_projection_registration(&identity).is_none());
    let downstream = plan.prepare_downstream_state();
    assert_eq!(
        downstream.scalar_projection_registration(&identity),
        Some(&registration)
    );
}

#[test]
fn duplicate_identity_is_rejected_across_projection_shapes() {
    let workspace = projection_workspace(true);
    let domain = workspace.worth_ui().expect("Worth UI domain installed");
    let plan = WorthUiQueryBindingPlan::default()
        .register_scalar_projection(scalar_registration(&domain, "platform.pulse.duplicate"))
        .expect("first projection registration");

    let denial = plan
        .register_collection_projection(collection_registration(
            &domain,
            "platform.pulse.duplicate",
        ))
        .expect_err("one projection identity cannot cross shapes");

    assert_eq!(
        denial.kind(),
        WorthUiQueryBindingRegistrationDenialKind::DuplicateProjectionIdentity
    );
}

#[test]
fn foreign_projection_domain_is_rejected_without_losing_local_plan() {
    let local = projection_workspace(true);
    let foreign = collection_projection_workspace();
    let local_domain = local.worth_ui().expect("local Worth UI domain");
    let foreign_domain = foreign.worth_ui().expect("foreign Worth UI domain");
    let plan = WorthUiQueryBindingPlan::default()
        .register_scalar_projection(scalar_registration(&local_domain, "platform.pulse.local"))
        .expect("local projection registration");

    let denial = plan
        .register_collection_projection(collection_registration(
            &foreign_domain,
            "platform.pulse.foreign",
        ))
        .expect_err("foreign Query authority must stop at registration");

    assert_eq!(
        denial.kind(),
        WorthUiQueryBindingRegistrationDenialKind::ForeignInstalledDomain
    );
}

fn scalar_registration(
    domain: &crate::WorthUiInstalledQueryDomain,
    identity: &str,
) -> UiScalarProjectionRegistration {
    UiScalarProjectionRegistration::text(
        domain
            .projection_view(identity)
            .expect("valid scalar projection view"),
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    )
}

fn collection_registration(
    domain: &crate::WorthUiInstalledQueryDomain,
    identity: &str,
) -> UiCollectionProjectionRegistration {
    UiCollectionProjectionRegistration::text(
        domain
            .projection_view(identity)
            .expect("valid collection projection view"),
        UiProjectionFieldRequirement::declared("identity.id").expect("valid row identity"),
        [UiProjectionFieldRequirement::declared("status").expect("valid selected field")],
        true,
        false,
    )
    .expect("valid collection requirement")
}
