use crate::{
    application_binding::{
        WorthUiScalarTextConsumptionOutcome, WorthUiScalarTextExecutionOutcome,
        WorthUiScalarTextPublicationOutcome, WorthUiScalarTextSettlementOutcome,
    },
    scalar_text_projection_fixture::{insert_status, projection_workspace},
    UiProjectionBindingStopKind, UiProjectionConsumptionBudget, UiProjectionFieldRequirement,
    UiProjectionUnavailableKind, UiScalarProjectionBindingAdmission,
    UiScalarProjectionRegistration, WorthUiQueryOperationAttemptDenial, WorthUiQueryWorkspaceExt,
};

#[test]
fn scalar_registration_admits_only_the_installed_live_view_contract() {
    let workspace = projection_workspace(true);
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let view = installed
        .projection_view("platform.pulse.status")
        .expect("valid Platform Pulse view identity");
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );

    let binding = match registration.admit(&workspace) {
        UiScalarProjectionBindingAdmission::Ready(binding) => binding,
        UiScalarProjectionBindingAdmission::Unavailable(unavailable) => {
            panic!("the supported scalar contract was unavailable: {unavailable:?}")
        }
        UiScalarProjectionBindingAdmission::Stopped(stop) => {
            panic!(
                "the installed live scalar contract must bind: {}",
                stop.summary()
            )
        }
    };

    assert_eq!(binding.view_identity().as_str(), "platform.pulse.status");
    assert_eq!(
        binding.requirement().selected_field().declared_name(),
        "status"
    );
    assert!(!binding
        .core()
        .query_binding_identity_for_reporting()
        .is_empty());
}

#[test]
fn scalar_registration_rejects_a_foreign_query_world_before_binding() {
    let source = projection_workspace(true);
    let foreign = projection_workspace(true);
    let view = source
        .worth_ui()
        .expect("source Worth UI domain")
        .projection_view("platform.pulse.status")
        .expect("valid Platform Pulse view identity");
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );

    let stop = match registration.admit(&foreign) {
        UiScalarProjectionBindingAdmission::Ready(_) => {
            panic!("foreign Query authority must not open scalar binding")
        }
        UiScalarProjectionBindingAdmission::Unavailable(_) => {
            panic!("foreign Query authority must stop, not become unavailable")
        }
        UiScalarProjectionBindingAdmission::Stopped(stop) => stop,
    };

    assert_eq!(stop.kind(), UiProjectionBindingStopKind::WrongWorld);
}

#[test]
fn scalar_registration_rejects_missing_async_lifecycle_support() {
    let workspace = projection_workspace(false);
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .projection_view("platform.pulse.status")
        .expect("valid Platform Pulse view identity");
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );

    let unavailable = match registration.admit(&workspace) {
        UiScalarProjectionBindingAdmission::Ready(_) => {
            panic!("a live scalar binding must require Query async lifecycle support")
        }
        UiScalarProjectionBindingAdmission::Unavailable(unavailable) => unavailable,
        UiScalarProjectionBindingAdmission::Stopped(stop) => {
            panic!("unsupported Query lifecycle is a result posture: {stop:?}")
        }
    };

    assert_eq!(unavailable.kind(), UiProjectionUnavailableKind::Unsupported);
    assert!(!unavailable
        .query_transition_identity_for_reporting()
        .is_empty());
}

#[test]
fn scalar_registration_rejects_an_uninstalled_selected_field() {
    let workspace = projection_workspace(true);
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .projection_view("platform.pulse.status")
        .expect("valid Platform Pulse view identity");
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("revision").expect("valid selected field"),
    );

    let stop = match registration.admit(&workspace) {
        UiScalarProjectionBindingAdmission::Ready(_) => {
            panic!("the binding must not silently substitute the status field")
        }
        UiScalarProjectionBindingAdmission::Unavailable(_) => {
            panic!("field mismatch must stop, not become unavailable")
        }
        UiScalarProjectionBindingAdmission::Stopped(stop) => stop,
    };

    assert_eq!(stop.kind(), UiProjectionBindingStopKind::SchemaMismatch);
}

#[test]
fn scalar_text_progression_carries_exact_authority_into_indexed_native_access() {
    let mut workspace = projection_workspace(true);
    insert_status(&mut workspace, "Ready");
    let installed = workspace.worth_ui().expect("Worth UI domain installed");
    let operation = installed.scalar_text_operation_reference();
    let prepared = match operation
        .enter_attempt(&workspace)
        .expect("exact installed authority enters its operating world")
        .prepare_consumer("status")
    {
        Ok(prepared) => prepared,
        Err(_) => panic!("the supported scalar consumer must prepare"),
    };
    let executed = match prepared.execute(&mut workspace) {
        WorthUiScalarTextExecutionOutcome::Executed(executed) => *executed,
        _ => panic!("the in-memory scalar operation must execute immediately"),
    };
    let published = match executed.publish() {
        WorthUiScalarTextPublicationOutcome::Published(published) => *published,
        _ => panic!("the executed scalar operation must publish"),
    };
    let consumed = match published.consume() {
        WorthUiScalarTextConsumptionOutcome::Consumed(consumed) => *consumed,
        _ => panic!("the exact scalar projection must be consumed"),
    };
    let settled = match consumed.settle() {
        WorthUiScalarTextSettlementOutcome::Settled(settled) => *settled,
        _ => panic!("the consumed scalar projection must settle"),
    };
    let derived = match settled.derive_native_text(UiProjectionConsumptionBudget::platform_pulse())
    {
        Ok(derived) => derived,
        Err(_) => panic!("the scalar native value must fit the Platform Pulse budget"),
    };

    assert!(derived.installation_is_current());
    let resolution = derived.resolution_counters();
    assert_eq!(resolution.declaration_checks, 1);
    assert_eq!(resolution.indexed_slot_lookups, 2);
    assert_eq!(resolution.path_matches, 0);
    assert_eq!(resolution.key_scans, 0);
    assert_eq!(resolution.path_parses, 0);
    let access = derived.access_counters();
    assert_eq!(access.indexed_accesses, 1);
    assert_eq!(access.fact_scans, 0);
    assert_eq!(access.row_scans, 0);
    assert_eq!(access.path_parses, 0);
    assert_eq!(access.view_registry_inspections, 0);
    assert_eq!(access.domain_registry_inspections, 0);
    assert_eq!(derived.into_value().as_str(), "Ready");
}

#[test]
fn scalar_text_reference_rejects_a_foreign_operating_world_before_binding() {
    let source = projection_workspace(true);
    let foreign = projection_workspace(true);
    let operation = source
        .worth_ui()
        .expect("source Worth UI domain")
        .scalar_text_operation_reference();

    assert!(matches!(
        operation.enter_attempt(&foreign),
        Err(WorthUiQueryOperationAttemptDenial::InstalledDomainAuthorityMismatch)
    ));
}
