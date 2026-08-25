use worth_ui_query_binding::{
    UiProjectionFieldRequirement, UiScalarProjectionRegistration, UiScalarSchemaRequirement,
    WorthUiQueryWorkspaceExt,
};

use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership,
};
use crate::runtime::observation::UiChangeClassificationOutcome;
use crate::runtime::rebind::{
    UiProjectionPredecessorValuePolicy, UiProjectionSchemaRequirement,
    UiProjectionSchemaTransitionKind, UiRebindExecutionPolicy,
};
use crate::runtime::{WorthUiSourceProvider, WorthUiWatcherEvent};

const COMPONENT: &str = "platform.pulse.component.projected_status";
const PROJECTION: &str = "platform.pulse.status";

#[test]
fn real_source_schema_edit_compiles_typed_stop_and_preserves_mounted_value() {
    let registration = status_registration();
    let capability_app = schema_app_builder(registration.clone())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("schema capability application prepares");
    let active = source_submission(
        "schema-active-status",
        "status",
        capability_app.capabilities(),
    );
    let mut session = schema_app_builder(registration)
        .with_candidate_submission(active)
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("real status source prepares")
        .launch()
        .expect("real status source launches");
    let candidate = source_submission(
        "schema-candidate-revision",
        "revision",
        session.capabilities(),
    );

    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("real field edit must change authored meaning"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    let plan = session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .expect("real mismatch compiles an explicit preservation plan");

    let [transition] = plan.projection_schema_transitions() else {
        panic!("one governed projection must carry one typed schema transition")
    };
    assert_eq!(transition.kind(), UiProjectionSchemaTransitionKind::Stopped);
    assert_eq!(
        transition.predecessor_policy(),
        UiProjectionPredecessorValuePolicy::Preserve
    );
    assert_eq!(
        transition.component_identity(),
        "component:platform.pulse.component.projected_status"
    );
    assert_eq!(transition.declaration_identity(), PROJECTION);
    assert_eq!(transition.view_identity().as_str(), PROJECTION);
    assert_eq!(selected_field(transition.predecessor()), "status");
    assert_eq!(selected_field(transition.candidate()), "value");
    assert_eq!(
        typed_field(transition.candidate()),
        worth_ui_query_binding::WorthUiProjectionField::QueryRevisionValue
    );
    assert_eq!(selected_field(transition.installed()), "status");
    let Some(crate::mounting::UiMountedSemanticTextContent::Scalar(content)) =
        plan.content().get(transition.graph_node())
    else {
        panic!("the exact governed graph node must receive scalar preservation content")
    };
    assert!(matches!(
        content.value(),
        crate::mounting::UiMountedSemanticTextValueDirective::Preserve
    ));
    assert_eq!(content.posture().as_ref(), "SCHEMA MISMATCH");

    drop(plan);
    let _ = session.shutdown();
}

#[test]
fn schema_transition_classifier_distinguishes_stop_recovery_and_steady_current() {
    let status = scalar_requirement("status");
    let revision = scalar_requirement("revision");
    assert_eq!(
        super::schema_transition::classify_transition(&status, &revision, &status),
        Some(UiProjectionSchemaTransitionKind::Stopped)
    );
    assert_eq!(
        super::schema_transition::classify_transition(&revision, &status, &status),
        Some(UiProjectionSchemaTransitionKind::Recovered)
    );
    assert_eq!(
        super::schema_transition::classify_transition(&status, &status, &status),
        None
    );
}

#[test]
fn equivalent_schema_recovery_prepares_authored_content_without_structural_allocation() {
    let registration = status_registration();
    let capability_app = schema_app_builder(registration.clone())
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("schema capability application prepares");
    let active = source_submission(
        "schema-active-revision",
        "revision",
        capability_app.capabilities(),
    );
    let mut session = schema_app_builder(registration)
        .with_candidate_submission(active)
        .freeze()
        .map(crate::facade::entry::WorthUiCertificationApplicationTransition::activate_builder_host)
        .expect("real mismatched revision source prepares")
        .launch()
        .expect("real mismatched revision source launches");
    let prior = session.generation_identity().clone();
    let candidate = source_submission(
        "schema-candidate-status-recovery",
        "status",
        session.capabilities(),
    );

    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_source(candidate).unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("real recovery must change authored meaning"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    let plan = session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .expect("real recovery compiles");
    assert!(matches!(
        plan.semantic_proof(),
        super::super::UiRebindSemanticProof::AuthoredContent(_)
    ));
    let candidate = plan.basis().candidate_generation().clone();
    let prepared = session
        .prepare_rebind(
            plan,
            crate::runtime::rebind::UiRebindExecutionRequest::new(1),
        )
        .expect("authored recovery avoids structural candidate allocation");
    assert_eq!(prepared.candidate_generation(), &candidate);
    assert!(prepared.prepared_frame().is_some());
    drop(prepared);
    assert_eq!(session.generation_identity(), &prior);
    let _ = session.shutdown();
}

fn schema_app_builder(
    registration: UiScalarProjectionRegistration,
) -> crate::facade::entry::WorthUiCertificationApplicationBuilder {
    crate::facade::WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_component(ComponentDescriptor::new(
            ComponentId::new(COMPONENT).unwrap(),
            ComponentPropSchema::named("platform.pulse.projected_status.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_scalar_projection(registration)
        .expect("product scalar projection registration")
}

fn source_submission(
    source_name: &str,
    selected_field: &str,
    capabilities: &crate::capability::CapabilitySnapshot,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    let source = format!(
        "component {COMPONENT} {{ content projection {PROJECTION} }}\n\
         query_scalar {PROJECTION} {{ view {PROJECTION} field {selected_field} \
         require text lifecycle live }}"
    );
    crate::runtime::tests::source_ingress_boundary_test_support::lower_file_submission(
        WorthUiSourceProvider::in_memory(source_name).with_file("app/main.wui", source),
        [WorthUiWatcherEvent::provider_revision(source_name)],
        capabilities,
    )
}

fn status_registration() -> UiScalarProjectionRegistration {
    let workspace = worth_ui_query_binding::certification::scalar_projection_workspace(true);
    let domain = workspace
        .worth_ui()
        .expect("Worth UI Query domain installed");
    UiScalarProjectionRegistration::text(
        domain.projection_view(PROJECTION).unwrap(),
        UiProjectionFieldRequirement::query_text_status(),
    )
}

fn scalar_requirement(selected_field: &str) -> UiProjectionSchemaRequirement {
    let selected_field = match selected_field {
        "status" => UiProjectionFieldRequirement::query_text_status(),
        "revision" => UiProjectionFieldRequirement::query_revision(),
        other => UiProjectionFieldRequirement::declared(other).unwrap(),
    };
    UiProjectionSchemaRequirement::Scalar(UiScalarSchemaRequirement::text(
        selected_field,
        worth_ui_query_binding::UiProjectionLifecycleRequirement::Live,
    ))
}

fn selected_field(requirement: &UiProjectionSchemaRequirement) -> &str {
    match requirement {
        UiProjectionSchemaRequirement::Scalar(requirement) => {
            requirement.selected_field().declared_name()
        }
        UiProjectionSchemaRequirement::Collection(_) => {
            panic!("the scalar Pulse requirement must not change shape")
        }
    }
}

fn typed_field(
    requirement: &UiProjectionSchemaRequirement,
) -> worth_ui_query_binding::WorthUiProjectionField {
    match requirement {
        UiProjectionSchemaRequirement::Scalar(requirement) => requirement
            .selected_field()
            .typed_field()
            .expect("real schema transition fields carry typed Worth UI authority"),
        UiProjectionSchemaRequirement::Collection(_) => {
            panic!("the scalar Pulse requirement must not change shape")
        }
    }
}
