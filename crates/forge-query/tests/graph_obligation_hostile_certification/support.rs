use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectTouch,
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionBudget,
    ForgeQueryGraphObligationExecutionInput, ForgeQueryGraphObligationExecutionScope,
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportMatrix,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationFamily,
};

pub fn authority_matrix() -> ForgeQueryGraphObligationSupportMatrix {
    ForgeQueryGraphObligationSupportMatrix::milestone_9_9_authority_surface()
}

pub fn committed_world() -> ForgeQueryGraphObligationOperatingWorldDescriptor {
    ForgeQueryGraphObligationOperatingWorldDescriptor::any_committed_authority()
}

pub fn graph_mutation_touch() -> ForgeQueryGraphTouchDescriptor {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.edge",
        ForgeQueryMutationFamily::Update,
        None,
        [set_operation("capacity")],
        [touch("capacity")],
    )
    .expect("representative graph mutation touch")
}

pub fn unrelated_touch() -> ForgeQueryGraphTouchDescriptor {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.face",
        ForgeQueryMutationFamily::Delete,
        None,
        [set_operation("boundary")],
        [touch("boundary")],
    )
    .expect("representative unrelated touch")
}

fn set_operation(authored_touch_text: &str) -> ForgeQueryAspectMutationOperation {
    ForgeQueryAspectMutationOperation::set(touch(authored_touch_text))
}

fn touch(authored_touch_text: &str) -> ForgeQueryAspectTouch {
    let mut segments = authored_touch_text.split('.');
    let aspect = segments
        .next()
        .and_then(AspectKey::new)
        .expect("test authored touch aspect should admit");
    let fields = segments
        .map(|segment| FieldKey::new(segment).expect("test authored touch field should admit"))
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::whole_aspect(aspect)
    } else {
        ForgeQueryAspectTouch::aspect_field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("test authored touch should have fields"),
        )
    }
}

pub fn registration_catalog() -> ForgeQueryGraphObligationRegistrationCatalog {
    ForgeQueryGraphObligationRegistrationCatalog::from_registrations(registrations())
        .expect("registration catalog")
}

pub fn registrations() -> Vec<ForgeQueryGraphObligationRegistration> {
    ForgeQueryGraphObligationKind::ALL
        .into_iter()
        .map(|kind| {
            registration_for_kind(
                kind,
                ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
                ForgeQueryGraphObligationSupportLane::GraphComposition,
            )
        })
        .collect()
}

pub fn registration_for_kind(
    kind: ForgeQueryGraphObligationKind,
    selector: ForgeQueryGraphTouchSelector,
    lane: ForgeQueryGraphObligationSupportLane,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::new(
        kind,
        rule_for_kind(kind),
        selector,
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(lane))
}

pub fn budget_limited_registration() -> ForgeQueryGraphObligationRegistration {
    registration_for_kind(
        ForgeQueryGraphObligationKind::CapabilityGapScreen,
        ForgeQueryGraphTouchSelector::collection("topology.edge").unwrap(),
        ForgeQueryGraphObligationSupportLane::GraphComposition,
    )
    .with_execution_budget(
        ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
            ForgeQueryGraphObligationExecutionScope::TouchedCollection,
            ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
        )
        .with_max_state_scope(0),
    )
}

pub fn execution_input(
    registration: ForgeQueryGraphObligationRegistration,
) -> ForgeQueryGraphObligationExecutionInput {
    ForgeQueryGraphObligationExecutionInput::from_selected_registration(
        "phase-20.selection.digest",
        registration,
    )
}

pub fn rule_for_kind(kind: ForgeQueryGraphObligationKind) -> ForgeQueryGraphObligationRuleIdentity {
    ForgeQueryGraphObligationRuleIdentity::new("phase-20.graph-obligation", kind.as_str(), "v1")
        .expect("phase 20 rule identity")
}
