use worth_ui::facade::{
    ComponentId, SurfaceId, WorthUiAdmittedCompositionGraphReceipt,
    WorthUiCompositionGraphDefinition, WorthUiCompositionRootDefinition,
    WorthUiCompositionRootKind, WorthUiCompositionRootMountAuthoritySet,
    WorthUiCompositionRootMountDenialCode, WorthUiCompositionRootReconciliationOutcome,
    WorthUiCompositionRootReconciliationReceipt, WorthUiCompositionRootSetDefinition,
    WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationLiveViewSource;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

#[path = "support/live_view_product_fixtures.rs"]
#[allow(dead_code)]
mod live_view_product_fixtures;

#[test]
fn mounted_product_view_accepts_multiple_admitted_root_authorities() {
    let app = prepared_app();
    let proof = app
        .live_view_projection_proof()
        .expect("projection admits through runtime");
    let root_set = WorthUiCompositionRootSetDefinition::from_graphs([
        proof.graph_backed_projection().composition_graph().clone(),
        external_root_graph(
            WorthUiCompositionRootKind::PortalEntry,
            "validation.portal.entry",
        ),
    ])
    .admit()
    .expect("distinct roots should admit");
    let authorities = authority_set(&app).with_portal_entry(
        "validation.portal.entry",
        external_surface_id(),
        external_component_id(),
    );

    let mounted = app
        .workbench()
        .runtime()
        .mount_live_view_product_projection_with_roots(
            proof.graph_backed_projection(),
            &root_set,
            &authorities,
        )
        .expect("root authorities should mount through product boundary");

    assert_eq!(mounted.root_entries().len(), 2);
    assert_eq!(mounted.counters().root_entry_count(), 2);
    let portal = mounted
        .root_entries()
        .iter()
        .find(|entry| entry.root_mount().root_kind() == WorthUiCompositionRootKind::PortalEntry)
        .expect("portal root should mount from explicit authority");
    assert_eq!(
        portal.root_mount().resolved_authority().external_kind(),
        Some(WorthUiCompositionRootKind::PortalEntry)
    );
    assert_eq!(
        portal
            .root_mount()
            .resolved_authority()
            .authority_identity(),
        Some("validation.portal.entry")
    );
    assert_root_mount_consumes_authority_and_mosaic(portal.root_mount().consumed_facts());
}

#[test]
fn future_root_kinds_mount_when_their_authority_receipts_are_present() {
    let app = prepared_app();
    for (kind, identity, authorities) in external_authority_cases(&app) {
        let graph = external_root_graph(kind, identity);
        let mount = app
            .workbench()
            .runtime()
            .admit_composition_root_mount_with_authority(&authorities, graph.root())
            .expect("explicit root authority should admit the root mount");

        assert_eq!(mount.root_kind(), kind);
        assert_eq!(mount.resolved_authority().external_kind(), Some(kind));
        assert_eq!(
            mount.resolved_authority().authority_identity(),
            Some(identity)
        );
        assert_eq!(mount.counters().source_reparse_count(), 0);
        assert_eq!(mount.counters().renderer_parse_count(), 0);
        assert_root_mount_consumes_authority_and_mosaic(mount.consumed_facts());
    }
}

#[test]
fn duplicate_root_identity_rejects_at_root_set_admission() {
    let first = external_root_graph(
        WorthUiCompositionRootKind::PortalEntry,
        "validation.portal.entry",
    );
    let duplicate = external_root_graph(
        WorthUiCompositionRootKind::PortalEntry,
        "validation.portal.entry",
    );

    let report = WorthUiCompositionRootSetDefinition::from_graphs([first, duplicate])
        .admit()
        .expect_err("duplicate root identities must reject");

    assert_eq!(report.denials().len(), 1);
    assert_eq!(
        report.denials()[0].code(),
        WorthUiCompositionRootMountDenialCode::DuplicateRootIdentity
    );
    assert_eq!(report.denials()[0].subject(), "validation.portal.entry");
    assert_ne!(report.denial_set_digest(), 0);
}

#[test]
fn denied_mosaic_placement_blocks_root_mounts() {
    let app = prepared_app();
    let graph = page_slot_graph("button_proof");
    let denied_mosaic = app
        .workbench()
        .runtime()
        .deny_mosaic_placement_for_page(app.workbench().page_host_plan());
    let authorities = WorthUiCompositionRootMountAuthoritySet::from_page_plan(
        app.workbench().page_host_plan().clone(),
        denied_mosaic,
    );

    let report = app
        .workbench()
        .runtime()
        .admit_composition_root_mount_with_authority(&authorities, graph.root())
        .expect_err("mosaic placement denial must block root mounting");

    assert_eq!(
        report.denials()[0].code(),
        WorthUiCompositionRootMountDenialCode::MosaicPlacementDenied
    );
    assert!(report.denials()[0]
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::MosaicPlacementLegality));
}

#[test]
fn root_reconciliation_names_mount_moves_without_rebinding_composition_graph() {
    let app = prepared_app();
    let graph = page_slot_graph("button_proof");
    let prior = app
        .workbench()
        .runtime()
        .admit_composition_root_mount_with_authority(&authority_set(&app), graph.root())
        .expect("page slot mount should admit");
    let surface_graph = WorthUiCompositionGraphDefinition::for_root(
        WorthUiCompositionRootDefinition::surface("worth.surface.preview.primitive.proof"),
    )
    .admit()
    .expect("surface root should admit");
    let next = app
        .workbench()
        .runtime()
        .admit_composition_root_mount_with_authority(&authority_set(&app), surface_graph.root())
        .expect("surface mount should admit");

    let reconciliation = WorthUiCompositionRootReconciliationReceipt::from_root_mounts(
        &prior,
        &next,
        graph.receipt_digest(),
        graph.receipt_digest(),
    );

    assert_eq!(
        reconciliation.outcome(),
        WorthUiCompositionRootReconciliationOutcome::Moved
    );
    assert_eq!(
        reconciliation.prior_composition_graph_digest(),
        graph.receipt_digest()
    );
    assert_eq!(
        reconciliation.next_composition_graph_digest(),
        graph.receipt_digest()
    );
    assert_ne!(reconciliation.receipt_digest(), 0);
    assert!(reconciliation
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::MosaicPlacementLegality));
}

fn prepared_app() -> worth_ui_validation_app::ValidationWorkbenchApp {
    let launch = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(
            ValidationWorkbenchAuthoredInputs::sample().with_live_view_source(
                ValidationLiveViewSource::new(live_view_product_fixtures::contact_form_source()),
            ),
        )
        .expect("validation app should prepare");
    worth_ui_validation_app::ValidationWorkbenchApp::new(launch)
}

fn external_authority_cases(
    app: &worth_ui_validation_app::ValidationWorkbenchApp,
) -> Vec<(
    WorthUiCompositionRootKind,
    &'static str,
    WorthUiCompositionRootMountAuthoritySet,
)> {
    vec![
        (
            WorthUiCompositionRootKind::ComponentInstance,
            "validation.component.instance",
            authority_set(app).with_component_instance(
                "validation.component.instance",
                external_surface_id(),
                external_component_id(),
            ),
        ),
        (
            WorthUiCompositionRootKind::PortalEntry,
            "validation.portal.entry",
            authority_set(app).with_portal_entry(
                "validation.portal.entry",
                external_surface_id(),
                external_component_id(),
            ),
        ),
        (
            WorthUiCompositionRootKind::CollectionItem,
            "validation.collection.item",
            authority_set(app).with_collection_item(
                "validation.collection.item",
                external_surface_id(),
                external_component_id(),
            ),
        ),
        (
            WorthUiCompositionRootKind::DiagnosticPanel,
            "validation.diagnostic.panel",
            authority_set(app).with_diagnostic_panel(
                "validation.diagnostic.panel",
                external_surface_id(),
                external_component_id(),
            ),
        ),
    ]
}

fn authority_set(
    app: &worth_ui_validation_app::ValidationWorkbenchApp,
) -> WorthUiCompositionRootMountAuthoritySet {
    let mosaic = app
        .workbench()
        .runtime()
        .admit_mosaic_placement_for_page(app.workbench().page_host_plan());
    WorthUiCompositionRootMountAuthoritySet::from_page_plan(
        app.workbench().page_host_plan().clone(),
        mosaic,
    )
}

fn page_slot_graph(identity: &str) -> WorthUiAdmittedCompositionGraphReceipt {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::new(
        WorthUiCompositionRootKind::PageContentSlot,
        identity,
    ))
    .admit()
    .expect("page slot graph should admit")
}

fn external_root_graph(
    kind: WorthUiCompositionRootKind,
    identity: &str,
) -> WorthUiAdmittedCompositionGraphReceipt {
    WorthUiCompositionGraphDefinition::for_root(WorthUiCompositionRootDefinition::new(
        kind, identity,
    ))
    .admit()
    .expect("external root graph should admit")
}

fn external_surface_id() -> SurfaceId {
    SurfaceId::new("validation.surface.external").expect("valid external surface id")
}

fn external_component_id() -> ComponentId {
    ComponentId::new("validation.component.external").expect("valid external component id")
}

fn assert_root_mount_consumes_authority_and_mosaic(
    facts: &[worth_ui::facade::WorthUiRuntimeFactId],
) {
    assert!(facts
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::CompositionRootMountAuthority));
    assert!(facts
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::MosaicPlacementLegality));
}
