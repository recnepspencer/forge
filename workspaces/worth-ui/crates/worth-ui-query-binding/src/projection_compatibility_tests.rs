use crate::{
    scalar_text_projection_fixture::{collection_projection_workspace, projection_workspace},
    UiCollectionProjectionBinding, UiCollectionProjectionBindingAdmission,
    UiCollectionProjectionRegistration, UiCollectionProjectionReplacementOutcome,
    UiProjectionBindingStopKind, UiProjectionFieldRequirement, UiScalarProjectionBinding,
    UiScalarProjectionBindingAdmission, UiScalarProjectionRegistration,
    UiScalarProjectionReplacementOutcome, WorthUiQueryWorkspaceExt,
};

#[test]
fn query_witness_is_required_before_scalar_replacement_preserves_identity() {
    let workspace = projection_workspace(true);
    let predecessor = scalar_binding(&workspace);
    let candidate = scalar_binding(&workspace);
    let logical_identity = predecessor
        .core()
        .query_binding_reporting_projection()
        .clone();

    let replacement = match predecessor.replace_with(candidate, &workspace) {
        UiScalarProjectionReplacementOutcome::Admitted(replacement) => *replacement,
        UiScalarProjectionReplacementOutcome::Stopped(stop) => {
            panic!(
                "equivalent Query contracts must replace: {}",
                stop.stop().summary()
            )
        }
    };

    assert_eq!(replacement.proof().predecessor_binding(), &logical_identity);
    assert_eq!(replacement.proof().successor_binding(), &logical_identity);
    let counters = replacement.proof().query_counters();
    assert!(counters.canonical_comparisons > 0);
    assert!(counters.portable_contract_comparisons > 0);
    assert_eq!(counters.execution_calls, 0);
    assert_eq!(counters.maintenance_calls, 0);
    let successor = replacement.into_successor();
    assert_eq!(
        successor.core().query_binding_reporting_projection(),
        &logical_identity
    );
}

#[test]
fn equal_looking_foreign_binding_stops_and_returns_usable_predecessor() {
    let source = projection_workspace(true);
    let foreign = projection_workspace(true);
    let predecessor = scalar_binding(&source);
    let foreign_candidate = scalar_binding(&foreign);

    let denial = match predecessor.replace_with(foreign_candidate, &source) {
        UiScalarProjectionReplacementOutcome::Admitted(_) => {
            panic!("equal printable contracts from foreign Query worlds must not replace")
        }
        UiScalarProjectionReplacementOutcome::Stopped(stop) => *stop,
    };

    assert_eq!(
        denial.stop().kind(),
        UiProjectionBindingStopKind::WrongWorld
    );
    assert!(denial.stop().predecessor_binding().is_some());
    let (predecessor, _foreign_candidate) = denial.into_bindings();
    let valid_candidate = scalar_binding(&source);
    match predecessor.replace_with(valid_candidate, &source) {
        UiScalarProjectionReplacementOutcome::Admitted(_) => {}
        UiScalarProjectionReplacementOutcome::Stopped(stop) => {
            panic!(
                "a denied replacement must return a usable predecessor: {}",
                stop.stop().summary()
            )
        }
    }
}

#[test]
fn query_witness_is_required_before_collection_replacement_preserves_identity() {
    let workspace = collection_projection_workspace();
    let predecessor = collection_binding(&workspace, false, true);
    let candidate = collection_binding(&workspace, false, true);
    let logical_identity = predecessor
        .core()
        .query_binding_reporting_projection()
        .clone();

    let replacement = match predecessor.replace_with(candidate, &workspace) {
        UiCollectionProjectionReplacementOutcome::Admitted(replacement) => *replacement,
        UiCollectionProjectionReplacementOutcome::Stopped(stop) => {
            panic!(
                "equivalent Query collection contracts must replace: {}",
                stop.stop().summary()
            )
        }
    };

    let counters = replacement.proof().query_counters();
    assert!(counters.canonical_comparisons > 0);
    assert_eq!(counters.execution_calls, 0);
    assert_eq!(counters.maintenance_calls, 0);
    assert_eq!(
        replacement
            .into_successor()
            .core()
            .query_binding_reporting_projection(),
        &logical_identity
    );
}

#[test]
fn collection_cardinality_mismatch_returns_both_bindings_without_a_successor() {
    let workspace = collection_projection_workspace();
    let predecessor = collection_binding(&workspace, false, true);
    let candidate = collection_binding(&workspace, true, false);

    let stop = collection_stop(predecessor.replace_with(candidate, &workspace));
    assert_eq!(
        stop.stop().kind(),
        UiProjectionBindingStopKind::PayloadShapeMismatch
    );
    let (predecessor, _candidate) = stop.into_bindings();
    match predecessor.replace_with(collection_binding(&workspace, false, true), &workspace) {
        UiCollectionProjectionReplacementOutcome::Admitted(_) => {}
        UiCollectionProjectionReplacementOutcome::Stopped(stop) => {
            panic!(
                "cardinality denial damaged predecessor: {}",
                stop.stop().summary()
            )
        }
    }
}

#[test]
fn foreign_and_stale_collection_pairs_never_mint_successors() {
    let source = collection_projection_workspace();
    let foreign = collection_projection_workspace();
    let stop = collection_stop(
        collection_binding(&source, false, true)
            .replace_with(collection_binding(&foreign, false, true), &source),
    );
    assert_eq!(stop.stop().kind(), UiProjectionBindingStopKind::WrongWorld);

    let mut stale = collection_projection_workspace();
    let predecessor = collection_binding(&stale, false, true);
    let candidate = collection_binding(&stale, false, true);
    worth_query::facade::consumer_kit::advance_test_workspace_domain_installation_generation(
        &mut stale,
    );
    let stop = collection_stop(predecessor.replace_with(candidate, &stale));
    assert_eq!(
        stop.stop().kind(),
        UiProjectionBindingStopKind::RebindRequired
    );
}

fn scalar_binding(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
) -> UiScalarProjectionBinding {
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .projection_view("platform.pulse.status")
        .expect("valid Platform Pulse view");
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );
    match registration.admit(workspace) {
        UiScalarProjectionBindingAdmission::Ready(binding) => binding,
        UiScalarProjectionBindingAdmission::Unavailable(unavailable) => {
            panic!("compatibility fixture must be supported: {unavailable:?}")
        }
        UiScalarProjectionBindingAdmission::Stopped(stop) => {
            panic!("scalar binding must admit: {}", stop.summary())
        }
    }
}

fn collection_binding(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    requires_complete_result: bool,
    permits_continuation: bool,
) -> UiCollectionProjectionBinding {
    let view = workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .projection_view("platform.pulse.statuses")
        .expect("valid Platform Pulse collection view");
    let registration = UiCollectionProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("identity.id").expect("valid row identity"),
        [UiProjectionFieldRequirement::declared("status").expect("valid selected field")],
        requires_complete_result,
        permits_continuation,
    )
    .expect("valid collection requirement");
    match registration.admit(workspace) {
        UiCollectionProjectionBindingAdmission::Ready(binding) => binding,
        UiCollectionProjectionBindingAdmission::Stopped(stop) => {
            panic!("collection binding must admit: {}", stop.summary())
        }
    }
}

fn collection_stop(
    outcome: UiCollectionProjectionReplacementOutcome,
) -> Box<crate::UiCollectionProjectionReplacementStop> {
    match outcome {
        UiCollectionProjectionReplacementOutcome::Admitted(_) => {
            panic!("incompatible collection replacement must stop")
        }
        UiCollectionProjectionReplacementOutcome::Stopped(stop) => stop,
    }
}
