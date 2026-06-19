use forge_query::facade::runtime::{
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
        ["set:capacity"],
        ["capacity"],
    )
    .expect("representative graph mutation touch")
}

pub fn unrelated_touch() -> ForgeQueryGraphTouchDescriptor {
    ForgeQueryGraphTouchDescriptor::declared_mutation_collection(
        "topology.face",
        ForgeQueryMutationFamily::Delete,
        None,
        ["set:boundary"],
        ["boundary"],
    )
    .expect("representative unrelated touch")
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
