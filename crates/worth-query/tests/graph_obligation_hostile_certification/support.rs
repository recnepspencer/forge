use worth_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectTouch,
    WorthQueryGraphObligationBudgetExceededPolicy, WorthQueryGraphObligationExecutionBudget,
    WorthQueryGraphObligationExecutionInput, WorthQueryGraphObligationExecutionScope,
    WorthQueryGraphObligationKind, WorthQueryGraphObligationOperatingWorldDescriptor,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphObligationSupportLane, WorthQueryGraphObligationSupportMatrix,
    WorthQueryGraphObligationSupportPosture, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchSelector, WorthQueryMutationFamily,
};

pub fn authority_matrix() -> WorthQueryGraphObligationSupportMatrix {
    WorthQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface()
}

pub fn committed_world() -> WorthQueryGraphObligationOperatingWorldDescriptor {
    WorthQueryGraphObligationOperatingWorldDescriptor::any_committed_authority()
}

pub fn graph_mutation_touch() -> WorthQueryGraphTouchDescriptor {
    WorthQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.edge",
        WorthQueryMutationFamily::Update,
        None,
        [set_operation("capacity")],
        [touch("capacity")],
    )
    .expect("representative graph mutation touch")
}

pub fn unrelated_touch() -> WorthQueryGraphTouchDescriptor {
    WorthQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.face",
        WorthQueryMutationFamily::Delete,
        None,
        [set_operation("boundary")],
        [touch("boundary")],
    )
    .expect("representative unrelated touch")
}

fn set_operation(authored_touch_text: &str) -> WorthQueryAspectMutationOperation {
    WorthQueryAspectMutationOperation::set(touch(authored_touch_text))
}

fn touch(authored_touch_text: &str) -> WorthQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments
        .next()
        .and_then(AspectKey::new)
        .expect("test authored touch aspect should admit");
    let fields = segments
        .map(|segment| FieldKey::new(segment).expect("test authored touch field should admit"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        WorthQueryAspectTouch::whole_aspect(aspect)
    } else {
        WorthQueryAspectTouch::aspect_field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("test authored touch should have fields"),
        )
    }
}

pub fn registration_catalog() -> WorthQueryGraphObligationRegistrationCatalog {
    WorthQueryGraphObligationRegistrationCatalog::from_registrations(registrations())
        .expect("registration catalog")
}

pub fn registrations() -> Vec<WorthQueryGraphObligationRegistration> {
    WorthQueryGraphObligationKind::ALL
        .into_iter()
        .map(|kind| {
            registration_for_kind(
                kind,
                WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
                WorthQueryGraphObligationSupportLane::GraphComposition,
            )
        })
        .collect()
}

pub fn registration_for_kind(
    kind: WorthQueryGraphObligationKind,
    selector: WorthQueryGraphTouchSelector,
    lane: WorthQueryGraphObligationSupportLane,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::new(
        kind,
        rule_for_kind(kind),
        selector,
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(WorthQueryGraphObligationSupportPosture::supported(lane))
}

pub fn budget_limited_registration() -> WorthQueryGraphObligationRegistration {
    registration_for_kind(
        WorthQueryGraphObligationKind::CapabilityGapScreen,
        WorthQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        WorthQueryGraphObligationSupportLane::GraphComposition,
    )
    .with_execution_budget(
        WorthQueryGraphObligationExecutionBudget::bounded_sparse(
            WorthQueryGraphObligationExecutionScope::TouchedCollection,
            WorthQueryGraphObligationBudgetExceededPolicy::FailClosed,
        )
        .with_max_state_scope(0),
    )
}

pub fn execution_input(
    registration: WorthQueryGraphObligationRegistration,
) -> WorthQueryGraphObligationExecutionInput {
    WorthQueryGraphObligationExecutionInput::from_selected_registration(
        "phase-20.selection.digest",
        registration,
    )
}

pub fn rule_for_kind(kind: WorthQueryGraphObligationKind) -> WorthQueryGraphObligationRuleIdentity {
    WorthQueryGraphObligationRuleIdentity::new("phase-20.graph-obligation", kind.as_str(), "v1")
        .expect("phase 20 rule identity")
}
