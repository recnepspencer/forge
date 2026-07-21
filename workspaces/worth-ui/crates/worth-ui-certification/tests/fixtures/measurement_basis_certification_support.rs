mod measurement_basis_query_support;

use worth_ui::facade::app::WorthUiApp;
use worth_ui::facade::declaration::{UiDeclarationArtifact, UiDeclaredMeasurementPolicyPosture};
use worth_ui::facade::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};
use worth_ui_host_contract::{
    UiFontMeasurementKey, UiFontMetricsObservation, UiFontMetricsRequest, UiHostObservationValue,
    UiMeasurementEvidenceFamily, UiMeasurementRequest, UiMeasurementRequestIdentity,
    UiPortalAnchorRectObservation, UiPortalAnchorRectRequest, UiScrollContainerViewportObservation,
    UiScrollContainerViewportRequest, UiViewportExtentObservation, UiViewportExtentRequest,
    WorthUiHostCapabilityReport, WorthUiHostContract, WorthUiMeasurementHostAdapter,
};
use worth_ui_inspection::UiEvidenceAuthorityGeneration;
use worth_ui_runtime::facade::evidence::{
    UiMeasurementBasisCertificationHostRequest, UiMeasurementBasisCertificationScenario,
};
use worth_ui_runtime::facade::host_observation::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext, UiPortalAnchorCoordinateSpacePosture,
};

use self::measurement_basis_query_support::{
    display_field_projection_consumption, display_projection_consumptions_across_basis_generations,
    measurement_touch, query_measurement_app, target_bound_to_projection_consumption,
};

#[derive(Clone, Copy)]
pub struct StaticMeasurementAdapter {
    pub viewport_width: f32,
}

impl WorthUiMeasurementHostAdapter for StaticMeasurementAdapter {
    fn observe_measurement(&self, request: &UiMeasurementRequest) -> UiHostObservationValue {
        match request.family() {
            worth_ui_host_contract::UiMeasurementRequestFamily::FontMetrics => {
                UiHostObservationValue::FontMetrics(UiFontMetricsObservation {
                    ascent: 7.0,
                    descent: 2.0,
                    line_gap: 1.0,
                })
            }
            worth_ui_host_contract::UiMeasurementRequestFamily::ViewportExtent => {
                UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
                    width: self.viewport_width,
                    height: 50.0,
                })
            }
            worth_ui_host_contract::UiMeasurementRequestFamily::ScrollContainerViewport => {
                UiHostObservationValue::ScrollContainerViewport(
                    UiScrollContainerViewportObservation {
                        width: self.viewport_width,
                        height: 50.0,
                    },
                )
            }
            worth_ui_host_contract::UiMeasurementRequestFamily::PortalAnchorRect => {
                UiHostObservationValue::PortalAnchorRect(UiPortalAnchorRectObservation {
                    x: 12.0,
                    y: 24.0,
                    width: 36.0,
                    height: 18.0,
                })
            }
            family => panic!("unexpected request family for certification fixture: {family:?}"),
        }
    }
}

pub fn equivalent_scroll_scenarios() -> (
    StaticMeasurementAdapter,
    UiMeasurementBasisCertificationScenario,
    StaticMeasurementAdapter,
    UiMeasurementBasisCertificationScenario,
) {
    let (world_profile, consumption) = display_field_projection_consumption("cert-equivalent");
    let app = query_measurement_app(world_profile.clone());
    let artifact = artifact_from_index(&app, 0);
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let capability_report = capability_report();

    (
        StaticMeasurementAdapter {
            viewport_width: 100.0,
        },
        scroll_owned_scenario(
            &app,
            artifact,
            world_profile.clone(),
            generation,
            consumption.clone(),
            &[
                host_font_request(100, &capability_report),
                host_viewport_request(101, &capability_report),
                host_scroll_container_request(102, &capability_report),
            ],
        ),
        StaticMeasurementAdapter {
            viewport_width: 100.0,
        },
        scroll_owned_scenario(
            &app,
            artifact,
            world_profile,
            generation,
            consumption,
            &[
                host_scroll_container_request(102, &capability_report),
                host_viewport_request(101, &capability_report),
                host_font_request(100, &capability_report),
            ],
        ),
    )
}

pub fn stale_query_scenarios() -> (
    StaticMeasurementAdapter,
    UiMeasurementBasisCertificationScenario,
    StaticMeasurementAdapter,
    UiMeasurementBasisCertificationScenario,
    UiEvidenceAuthorityGeneration,
) {
    let ((world_profile, current_consumption), _) =
        display_projection_consumptions_across_basis_generations("cert-stale-query");
    let app = query_measurement_app(world_profile.clone());
    let artifact = artifact_from_index(&app, 0);
    let stale_generation = UiEvidenceAuthorityGeneration::new(17);
    let current_generation = UiEvidenceAuthorityGeneration::new(18);
    let capability_report = capability_report();

    (
        StaticMeasurementAdapter {
            viewport_width: 100.0,
        },
        scroll_owned_scenario(
            &app,
            artifact,
            world_profile.clone(),
            current_generation,
            current_consumption.clone(),
            &[
                host_font_request(200, &capability_report),
                host_viewport_request(201, &capability_report),
                host_scroll_container_request(202, &capability_report),
            ],
        )
        .with_query_receipt_authority_generation(stale_generation),
        StaticMeasurementAdapter {
            viewport_width: 100.0,
        },
        scroll_owned_scenario(
            &app,
            artifact,
            world_profile,
            current_generation,
            current_consumption,
            &[
                host_font_request(203, &capability_report),
                host_viewport_request(204, &capability_report),
                host_scroll_container_request(205, &capability_report),
            ],
        ),
        stale_generation,
    )
}

pub fn divergent_admitted_scenarios() -> (
    StaticMeasurementAdapter,
    UiMeasurementBasisCertificationScenario,
    StaticMeasurementAdapter,
    UiMeasurementBasisCertificationScenario,
) {
    let (world_profile, consumption) = display_field_projection_consumption("cert-divergent");
    let app = query_measurement_app(world_profile.clone());
    let generation = UiEvidenceAuthorityGeneration::new(17);
    let capability_report = capability_report();

    (
        StaticMeasurementAdapter {
            viewport_width: 100.0,
        },
        scroll_owned_scenario(
            &app,
            artifact_from_index(&app, 0),
            world_profile.clone(),
            generation,
            consumption,
            &[
                host_font_request(300, &capability_report),
                host_viewport_request(301, &capability_report),
                host_scroll_container_request(302, &capability_report),
            ],
        ),
        StaticMeasurementAdapter {
            viewport_width: 100.0,
        },
        portal_scenario(
            &app,
            artifact_from_index(&app, 1),
            world_profile,
            generation,
            303,
        ),
    )
}

fn scroll_owned_scenario(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
    world_profile: UiGraphWorldProfile,
    generation: UiEvidenceAuthorityGeneration,
    authority: worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
    host_requests: &[UiMeasurementBasisCertificationHostRequest],
) -> UiMeasurementBasisCertificationScenario {
    let capability_report = capability_report();
    let target = admission_target(app, artifact, &authority);
    UiMeasurementBasisCertificationScenario::new(
        artifact.identity().clone(),
        graph_node_identity(app, artifact),
        world_profile,
        generation,
        measurement_policy(artifact),
        capability_report.clone(),
    )
    .with_query_authority(
        target
            .query_prerequisites()
            .expect("query-backed measurement target should carry prerequisites")
            .clone(),
        authority,
    )
    .with_host_requests(host_requests.to_vec().into_boxed_slice())
}

fn portal_scenario(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
    world_profile: UiGraphWorldProfile,
    generation: UiEvidenceAuthorityGeneration,
    request_identity: u64,
) -> UiMeasurementBasisCertificationScenario {
    let capability_report = capability_report();
    UiMeasurementBasisCertificationScenario::new(
        artifact.identity().clone(),
        graph_node_identity(app, artifact),
        world_profile,
        generation,
        measurement_policy(artifact),
        capability_report.clone(),
    )
    .with_host_requests(
        vec![UiMeasurementBasisCertificationHostRequest::new(
            UiMeasurementRequestIdentity::new(request_identity),
            UiMeasurementEvidenceFamily::PortalAnchorRect,
            UiHostMeasurementNeed::PortalAnchorRect(UiPortalAnchorRectRequest::new(77)),
            UiHostMeasurementNormalizationContext::portal_anchor_logical_exact_in(
                UiPortalAnchorCoordinateSpacePosture::PortalLayer,
                assumption_profile(&capability_report),
            ),
        )]
        .into_boxed_slice(),
    )
}

fn measurement_policy(artifact: &UiDeclarationArtifact) -> UiDeclaredMeasurementPolicyPosture {
    artifact
        .declared_posture()
        .expect("declaration posture should admit")
        .measurement_policy()
        .admitted()
        .expect("measurement policy should admit")
        .clone()
}

fn admission_target(
    app: &WorthUiApp,
    artifact: &UiDeclarationArtifact,
    authority: &worth_ui_query_binding::compatibility::managed_live::WorthUiQueryAuthorityHandle,
) -> worth_ui::facade::admission::UiAdmissionTarget {
    let touch = measurement_touch(
        app,
        artifact
            .provenance()
            .source_provenance()
            .declaration_index(),
    );
    target_bound_to_projection_consumption(&touch, authority)
}

fn graph_node_identity(app: &WorthUiApp, artifact: &UiDeclarationArtifact) -> UiGraphNodeIdentity {
    app.graph()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()[0]
}

fn artifact_from_index(app: &WorthUiApp, declaration_index: usize) -> &UiDeclarationArtifact {
    app.declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/query_measurement_eligibility.wui"
                && provenance.declaration_index() == declaration_index
        })
        .expect("test declaration should exist")
}

fn capability_report() -> WorthUiHostCapabilityReport {
    WorthUiHostCapabilityReport::from_contract(WorthUiHostContract::egui())
}

fn assumption_profile(report: &WorthUiHostCapabilityReport) -> UiHostMeasurementAssumptionProfile {
    UiHostMeasurementAssumptionProfile::from_capability_report(report, 11, 22, 33, 44)
}

fn host_font_request(
    request_identity: u64,
    capability_report: &WorthUiHostCapabilityReport,
) -> UiMeasurementBasisCertificationHostRequest {
    UiMeasurementBasisCertificationHostRequest::new(
        UiMeasurementRequestIdentity::new(request_identity),
        UiMeasurementEvidenceFamily::FontMetrics,
        UiHostMeasurementNeed::FontMetrics(UiFontMetricsRequest::new(UiFontMeasurementKey::new(
            "body-md",
        ))),
        UiHostMeasurementNormalizationContext::font_metrics_surface_logical_exact(
            assumption_profile(capability_report),
        ),
    )
}

fn host_viewport_request(
    request_identity: u64,
    capability_report: &WorthUiHostCapabilityReport,
) -> UiMeasurementBasisCertificationHostRequest {
    UiMeasurementBasisCertificationHostRequest::new(
        UiMeasurementRequestIdentity::new(request_identity),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
        UiHostMeasurementNormalizationContext::viewport_logical_exact(assumption_profile(
            capability_report,
        )),
    )
}

fn host_scroll_container_request(
    request_identity: u64,
    capability_report: &WorthUiHostCapabilityReport,
) -> UiMeasurementBasisCertificationHostRequest {
    UiMeasurementBasisCertificationHostRequest::new(
        UiMeasurementRequestIdentity::new(request_identity),
        UiMeasurementEvidenceFamily::ScrollContainerViewport,
        UiHostMeasurementNeed::ScrollContainerViewport(UiScrollContainerViewportRequest::new(77)),
        UiHostMeasurementNormalizationContext::scroll_container_logical_exact(assumption_profile(
            capability_report,
        )),
    )
}
