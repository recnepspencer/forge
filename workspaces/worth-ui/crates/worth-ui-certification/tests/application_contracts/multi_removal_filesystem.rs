use worth_ui::facade::app::{WorthUi, WorthUiApplicationBuilder};
use worth_ui::facade::declaration::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MeasurementConstraint, MeasurementValue, MosaicChildRule,
    MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority,
    MosaicOverflowBehavior, MosaicParentGrowthBehavior, MosaicRegionKindDescriptor,
    MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission,
    MosaicScrollOwnership, MosaicSizingBehavior, MosaicSizingContractDescriptor,
    MosaicSizingContractId, MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint,
    NamedMeasurementDefinition, NamedMeasurementToken, SurfacePlacementClass,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::{
    admit_candidate_catalog, admit_candidate_catalog_with_removed_roots,
};
use worth_ui_certification::scenario::installed_query_world;
use worth_ui_host_contract::{
    UiHostMeasurementObservationValue, UiHostMeasurementRequest, UiMeasurementRequestFamily,
    UiPortalAnchorRectObservation, WorthUiHostCapability, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiMeasurementHostAdapter,
};
use worth_ui_runtime::facade::application::UiAllocationCatalogRowDisposition;
use worth_ui_runtime::facade::host::{
    UiHostAdapterSessionAuthority, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    WorthUiOperationalHostAdapter,
};
use worth_ui_test_support::{
    WorthUiApplicationBuilderCertificationExt, WorthUiFrameworkTurnCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

const BASE: &str = "workspace.component.removal_base";
const FIRST: &str = "workspace.component.removal_first";
const SECOND: &str = "workspace.component.removal_second";
const BASE_REGION: &str = "workspace.region.removal_base";
const FIRST_REGION: &str = "workspace.region.removal_first";
const SECOND_REGION: &str = "workspace.region.removal_second";
const BASE_SIZING: &str = "workspace.sizing.removal_base";
const FIRST_SIZING: &str = "workspace.sizing.removal_first";
const SECOND_SIZING: &str = "workspace.sizing.removal_second";

#[test]
fn multiple_real_file_removals_publish_one_complete_public_successor() {
    let workspace = FilesystemContractWorkspace::new("multi-removal");
    workspace.write("app/main.wui", &base_source());
    let mut session = file_application(&workspace)
        .launch()
        .expect("real-file baseline should launch");
    let mut baseline_probe = session
        .prepare_replacement(filesystem_submission(&workspace, &session))
        .expect("equivalent real-file baseline should prepare");
    let baseline_roots = admit_candidate_catalog(&session, &mut baseline_probe)
        .changed_roots()
        .collect::<std::collections::BTreeSet<_>>();

    workspace.write_atomic(
        "app/main.wui",
        &format!(
            "{}component {FIRST} {{ region {FIRST_REGION} {{ sizing {FIRST_SIZING}; }} }}\ncomponent {SECOND} {{ region {SECOND_REGION} {{ sizing {SECOND_SIZING}; }} }}\n",
            base_source()
        ),
    );
    let pair_submission = filesystem_submission(&workspace, &session);
    let mut pair = session
        .prepare_replacement(pair_submission)
        .expect("two real-file composition roots should prepare");
    let pair_delta = admit_candidate_catalog(&session, &mut pair);
    let removed_roots = pair_delta
        .changed_roots()
        .filter(|root| !baseline_roots.contains(root))
        .collect::<Vec<_>>();
    let removed_count = removed_roots.len();
    assert_eq!(removed_count, 2);
    let pair_pending = session
        .stage_prepared_replacement(
            session
                .lower_prepared_replacement(*pair)
                .expect("pair should lower"),
        )
        .expect("pair should stage");
    let pair_boundary = boundary(&mut session);
    let pair_activation = session
        .activate_prepared_replacement(pair_pending, pair_delta, pair_boundary, None)
        .expect("pair should publish")
        .into_activation()
        .expect("pair changes active allocation truth");
    assert_eq!(
        pair_activation
            .allocation_catalog_successor()
            .successor_rows(),
        baseline_roots.len() + removed_count
    );

    workspace.write_atomic("app/main.wui", &base_source());
    let removal_submission = filesystem_submission(&workspace, &session);
    let mut removal = session
        .prepare_replacement(removal_submission)
        .expect("real-file removal candidate should prepare");
    let removal_delta =
        admit_candidate_catalog_with_removed_roots(&session, &mut removal, removed_roots.clone());
    let removal_pending = session
        .stage_prepared_replacement(
            session
                .lower_prepared_replacement(*removal)
                .expect("removal should lower"),
        )
        .expect("removal should stage");
    let removal_boundary = boundary(&mut session);
    let activation = session
        .activate_prepared_replacement(removal_pending, removal_delta, removal_boundary, None)
        .expect("both removals should publish atomically")
        .into_activation()
        .expect("removing active allocation truth is not a semantic no-op");
    let successor = activation.allocation_catalog_successor();

    let published_removals = successor
        .transitions()
        .iter()
        .filter(|transition| transition.disposition() == UiAllocationCatalogRowDisposition::Removed)
        .map(|transition| transition.root())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(published_removals, removed_roots.into_iter().collect());
    assert_eq!(published_removals.len(), removed_count);
    assert!(session
        .graph()
        .allocation_planning_node_identities()
        .all(|identity| !published_removals.contains(&identity)));

    let _ = session.shutdown();
    workspace.close();
}

fn filesystem_submission(
    workspace: &FilesystemContractWorkspace,
    session: &worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui::facade::source::WorthUiWatchedCandidateSubmission {
    WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("production filesystem provider should read committed bytes")
        .attempt_candidate_for_certification(session.capabilities())
        .expect("real source should lower through production semantics")
}

fn boundary(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> worth_ui_runtime::facade::execution::WorthUiFrameBoundary {
    session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_completion()
        .into_execution()
        .unwrap_or_else(|_| panic!("empty boundary turn should execute"))
        .into_activation_boundary()
}

fn file_application(workspace: &FilesystemContractWorkspace) -> worth_ui::facade::app::WorthUiApp {
    let capabilities = builder().freeze().expect("capability application");
    let submission = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("baseline should be read from disk")
        .attempt_candidate_for_certification(capabilities.capabilities())
        .expect("baseline should lower");
    builder()
        .with_candidate_submission(submission)
        .freeze()
        .expect("real-file application should prepare")
}

fn builder() -> WorthUiApplicationBuilder {
    WorthUi::app()
        .with_host(MultiRemovalHost)
        .with_graph_world_profile(installed_query_world::settled_query_world_profile(
            worth_ui::facade::declaration::ViewBindingId::new("multi.removal.filesystem").unwrap(),
            "worth-ui.phase14.filesystem.multi-removal",
        ))
        .register_component(component(BASE))
        .register_component(component(FIRST))
        .register_component(component(SECOND))
        .register_mosaic_region_kind(overlay_region(BASE_REGION))
        .register_mosaic_region_kind(overlay_region(FIRST_REGION))
        .register_mosaic_region_kind(overlay_region(SECOND_REGION))
        .register_mosaic_sizing_contract(fixed_sizing(
            BASE_SIZING,
            "workspace.measurement.removal_base",
        ))
        .register_mosaic_sizing_contract(fixed_sizing(
            FIRST_SIZING,
            "workspace.measurement.removal_first",
        ))
        .register_mosaic_sizing_contract(fixed_sizing(
            SECOND_SIZING,
            "workspace.measurement.removal_second",
        ))
}

fn base_source() -> String {
    format!("component {BASE} {{ region {BASE_REGION} {{ sizing {BASE_SIZING}; }} }}\n")
}

#[derive(Clone, Copy, Default)]
struct MultiRemovalHost;

impl WorthUiMeasurementHostAdapter for MultiRemovalHost {
    fn observe_measurement(
        &self,
        request: &UiHostMeasurementRequest,
    ) -> UiHostMeasurementObservationValue {
        match request.family() {
            UiMeasurementRequestFamily::PortalAnchorRect => {
                UiHostMeasurementObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
                    x: 24.0,
                    y: 48.0,
                    width: 640.0,
                    height: 360.0,
                })
            }
            family => panic!("unexpected multi-removal measurement request: {family:?}"),
        }
    }
}

impl WorthUiOperationalHostAdapter for MultiRemovalHost {
    fn operational_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::headless()
    }

    fn operational_capability_report(&self) -> WorthUiHostCapabilityReport {
        WorthUiHostCapabilityReport::available(vec![WorthUiHostCapability::PortalAnchorObservation])
    }

    fn release_host_session(
        &self,
        authority: &UiHostAdapterSessionAuthority,
    ) -> UiHostSessionReleaseOutcome {
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            authority.host_session_identity(),
            0,
        ))
    }
}

fn component(identity: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(identity).expect("component id"),
        ComponentPropSchema::named(format!("{identity}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn overlay_region(identity: &str) -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(
        MosaicRegionKindId::new(identity).expect("region id"),
        MosaicRegionRole::overlay(),
    )
    .with_sizing_behavior(MosaicSizingBehavior::overlay_anchored())
    .with_scroll_ownership(MosaicScrollOwnership::no_scrolling())
    .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(SurfacePlacementClass::overlay_layer())
    .with_persistence(MosaicRegionPersistence::restorable())
    .with_clipping(MosaicClippingPosture::clip_to_region())
    .with_hit_test(MosaicHitTestPosture::participates())
}

fn fixed_sizing(identity: &str, measurement: &str) -> MosaicSizingContractDescriptor {
    sizing_with_kind(identity, measurement, MosaicSizingKind::fixed())
}

fn sizing_with_kind(
    identity: &str,
    measurement: &str,
    kind: MosaicSizingKind,
) -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new(identity).expect("sizing id"),
        kind,
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new(measurement).expect("measurement token"),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(200),
            MeasurementValue::logical_pixels(640),
        ),
    ))
}
