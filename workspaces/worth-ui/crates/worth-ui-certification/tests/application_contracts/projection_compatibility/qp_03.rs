use worth_ui_query_binding::{
    certification::{collection_projection_workspace, scalar_projection_workspace},
    UiCollectionProjectionBinding, UiCollectionProjectionBindingAdmission,
    UiCollectionProjectionRegistration, UiCollectionProjectionReplacementOutcome,
    UiProjectionBindingStopKind, UiProjectionFieldRequirement, UiProjectionNativeFamily,
    UiScalarProjectionBinding, UiScalarProjectionBindingAdmission, UiScalarProjectionRegistration,
    UiScalarProjectionReplacementOutcome, WorthUiQueryWorkspaceExt,
};

#[test]
fn compatible_scalar_and_collection_pairs_preserve_identity_only_with_query_proof() {
    let scalar_world = scalar_projection_workspace(true);
    let scalar_predecessor = scalar_binding(&scalar_world, "platform.pulse.status");
    let scalar_identity = binding_identity(&scalar_predecessor);
    let scalar = scalar_admitted(scalar_predecessor.replace_with(
        scalar_binding(&scalar_world, "platform.pulse.status"),
        &scalar_world,
    ));
    assert_query_only_compatibility(scalar.proof());
    assert_eq!(
        scalar.into_successor().core().query_binding_reference(),
        &scalar_identity
    );

    let collection_world = collection_projection_workspace();
    let collection_predecessor =
        collection_binding(&collection_world, "platform.pulse.statuses", false, true);
    let collection_identity = collection_predecessor
        .core()
        .query_binding_reference()
        .clone();
    let collection = collection_admitted(collection_predecessor.replace_with(
        collection_binding(&collection_world, "platform.pulse.statuses", false, true),
        &collection_world,
    ));
    assert_query_only_compatibility(collection.proof());
    assert_eq!(
        collection.into_successor().core().query_binding_reference(),
        &collection_identity
    );
}

#[test]
fn schema_native_row_and_cardinality_axes_stop_before_successor_minting() {
    assert_view_schema_stop();
    assert_unprojected_field_candidate_never_admits();
    assert_native_family_candidate_never_admits();
    assert_row_identity_candidate_never_admits();
    assert_cardinality_stop_returns_usable_predecessor();
}

#[test]
fn equal_looking_foreign_and_stale_generation_pairs_remain_distinct() {
    let source = scalar_projection_workspace(true);
    let foreign = scalar_projection_workspace(true);
    let stop = scalar_stopped(
        scalar_binding(&source, "platform.pulse.status")
            .replace_with(scalar_binding(&foreign, "platform.pulse.status"), &source),
    );
    assert_eq!(stop.stop().kind(), UiProjectionBindingStopKind::WrongWorld);
    assert!(stop.stop().predecessor_binding().is_some());

    let mut stale = collection_projection_workspace();
    let predecessor = collection_binding(&stale, "platform.pulse.statuses", false, true);
    let candidate = collection_binding(&stale, "platform.pulse.statuses", false, true);
    worth_query::facade::consumer_kit::advance_test_workspace_domain_installation_generation(
        &mut stale,
    );
    let stop = collection_stopped(predecessor.replace_with(candidate, &stale));
    assert_eq!(
        stop.stop().kind(),
        UiProjectionBindingStopKind::RebindRequired
    );
    assert_ne!(stop.stop().kind(), UiProjectionBindingStopKind::WrongWorld);
}

fn assert_view_schema_stop() {
    let world = scalar_projection_workspace(true);
    let predecessor = scalar_binding(&world, "platform.pulse.status");
    let stop = scalar_stopped(
        predecessor.replace_with(scalar_binding(&world, "platform.pulse.other"), &world),
    );
    assert_eq!(
        stop.stop().kind(),
        UiProjectionBindingStopKind::SchemaMismatch
    );
    let (predecessor, _candidate) = stop.into_bindings();
    drop(
        scalar_admitted(
            predecessor.replace_with(scalar_binding(&world, "platform.pulse.status"), &world),
        )
        .into_successor(),
    );
}

fn assert_unprojected_field_candidate_never_admits() {
    let world = scalar_projection_workspace(true);
    let predecessor = scalar_binding(&world, "platform.pulse.status");
    let view = projection_view(&world, "platform.pulse.status");
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("identity.id").expect("valid declared field"),
    );
    let stop = match registration.admit(&world) {
        UiScalarProjectionBindingAdmission::Stopped(stop) => stop,
        other => panic!("non-projected scalar field must stop, got {other:?}"),
    };
    assert_eq!(stop.kind(), UiProjectionBindingStopKind::SchemaMismatch);
    drop(
        scalar_admitted(
            predecessor.replace_with(scalar_binding(&world, "platform.pulse.status"), &world),
        )
        .into_successor(),
    );
}

fn assert_native_family_candidate_never_admits() {
    let world = scalar_projection_workspace(true);
    let predecessor = scalar_binding(&world, "platform.pulse.status");
    let registration = UiScalarProjectionRegistration::native(
        projection_view(&world, "platform.pulse.status"),
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
        UiProjectionNativeFamily::Boolean,
    );
    let stop = match registration.admit(&world) {
        UiScalarProjectionBindingAdmission::Stopped(stop) => stop,
        other => panic!("unsupported native family must stop, got {other:?}"),
    };
    assert_eq!(
        stop.kind(),
        UiProjectionBindingStopKind::NativeFamilyMismatch
    );
    drop(
        scalar_admitted(
            predecessor.replace_with(scalar_binding(&world, "platform.pulse.status"), &world),
        )
        .into_successor(),
    );
}

fn assert_row_identity_candidate_never_admits() {
    let world = collection_projection_workspace();
    let predecessor = collection_binding(&world, "platform.pulse.statuses", false, true);
    let registration = collection_registration(
        &world,
        "platform.pulse.statuses",
        "query_text.status",
        false,
        true,
    );
    let stop = match registration.admit(&world) {
        UiCollectionProjectionBindingAdmission::Stopped(stop) => stop,
        UiCollectionProjectionBindingAdmission::Ready(_) => {
            panic!("wrong Query row identity must not mint a collection binding")
        }
    };
    assert_eq!(
        stop.kind(),
        UiProjectionBindingStopKind::RowIdentityMismatch
    );
    drop(
        collection_admitted(predecessor.replace_with(
            collection_binding(&world, "platform.pulse.statuses", false, true),
            &world,
        ))
        .into_successor(),
    );
}

fn assert_cardinality_stop_returns_usable_predecessor() {
    let world = collection_projection_workspace();
    let predecessor = collection_binding(&world, "platform.pulse.statuses", false, true);
    let candidate = collection_binding(&world, "platform.pulse.statuses", true, false);
    let stop = collection_stopped(predecessor.replace_with(candidate, &world));
    assert_eq!(
        stop.stop().kind(),
        UiProjectionBindingStopKind::PayloadShapeMismatch
    );
    let (predecessor, _candidate) = stop.into_bindings();
    drop(
        collection_admitted(predecessor.replace_with(
            collection_binding(&world, "platform.pulse.statuses", false, true),
            &world,
        ))
        .into_successor(),
    );
}

fn scalar_binding(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    view_identity: &str,
) -> UiScalarProjectionBinding {
    let registration = UiScalarProjectionRegistration::text(
        projection_view(workspace, view_identity),
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );
    match registration.admit(workspace) {
        UiScalarProjectionBindingAdmission::Ready(binding) => binding,
        other => panic!("scalar compatibility binding must admit, got {other:?}"),
    }
}

fn collection_binding(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    view_identity: &str,
    requires_complete_result: bool,
    permits_continuation: bool,
) -> UiCollectionProjectionBinding {
    match collection_registration(
        workspace,
        view_identity,
        "identity.id",
        requires_complete_result,
        permits_continuation,
    )
    .admit(workspace)
    {
        UiCollectionProjectionBindingAdmission::Ready(binding) => binding,
        UiCollectionProjectionBindingAdmission::Stopped(stop) => {
            panic!(
                "collection compatibility binding must admit: {}",
                stop.summary()
            )
        }
    }
}

fn collection_registration(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    view_identity: &str,
    row_identity: &str,
    requires_complete_result: bool,
    permits_continuation: bool,
) -> UiCollectionProjectionRegistration {
    UiCollectionProjectionRegistration::text(
        projection_view(workspace, view_identity),
        UiProjectionFieldRequirement::declared(row_identity).expect("valid row field"),
        [UiProjectionFieldRequirement::declared("status").expect("valid selected field")],
        requires_complete_result,
        permits_continuation,
    )
    .expect("valid collection registration")
}

fn projection_view(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    identity: &str,
) -> worth_ui_query_binding::UiInstalledProjectionView {
    workspace
        .worth_ui()
        .expect("Worth UI domain installed")
        .projection_view(identity)
        .expect("valid installed view identity")
}

fn binding_identity(
    binding: &UiScalarProjectionBinding,
) -> worth_ui_query_binding::UiQueryBindingReference {
    binding.core().query_binding_reference().clone()
}

fn assert_query_only_compatibility(
    proof: &worth_ui_query_binding::UiProjectionBindingCompatibilityProof,
) {
    let counters = proof.query_counters();
    assert!(counters.canonical_comparisons > 0);
    assert!(counters.portable_contract_comparisons > 0);
    assert_eq!(counters.execution_calls, 0);
    assert_eq!(counters.maintenance_calls, 0);
}

fn scalar_admitted(
    outcome: UiScalarProjectionReplacementOutcome,
) -> Box<worth_ui_query_binding::UiScalarProjectionReplacementReceipt> {
    match outcome {
        UiScalarProjectionReplacementOutcome::Admitted(receipt) => receipt,
        UiScalarProjectionReplacementOutcome::Stopped(stop) => {
            panic!(
                "compatible scalar replacement stopped: {}",
                stop.stop().summary()
            )
        }
    }
}

fn scalar_stopped(
    outcome: UiScalarProjectionReplacementOutcome,
) -> Box<worth_ui_query_binding::UiScalarProjectionReplacementStop> {
    match outcome {
        UiScalarProjectionReplacementOutcome::Admitted(_) => {
            panic!("incompatible scalar replacement minted a successor")
        }
        UiScalarProjectionReplacementOutcome::Stopped(stop) => stop,
    }
}

fn collection_admitted(
    outcome: UiCollectionProjectionReplacementOutcome,
) -> Box<worth_ui_query_binding::UiCollectionProjectionReplacementReceipt> {
    match outcome {
        UiCollectionProjectionReplacementOutcome::Admitted(receipt) => receipt,
        UiCollectionProjectionReplacementOutcome::Stopped(stop) => {
            panic!(
                "compatible collection replacement stopped: {}",
                stop.stop().summary()
            )
        }
    }
}

fn collection_stopped(
    outcome: UiCollectionProjectionReplacementOutcome,
) -> Box<worth_ui_query_binding::UiCollectionProjectionReplacementStop> {
    match outcome {
        UiCollectionProjectionReplacementOutcome::Admitted(_) => {
            panic!("incompatible collection replacement minted a successor")
        }
        UiCollectionProjectionReplacementOutcome::Stopped(stop) => stop,
    }
}
