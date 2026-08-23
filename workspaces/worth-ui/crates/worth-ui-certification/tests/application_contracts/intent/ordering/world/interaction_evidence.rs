use worth_ui::facade::{
    intent::{
        UiIntentAdmissionDecision, UiIntentDefinition, UiIntentExecutionAdvanceOutcome,
        UiIntentExecutionDispatchOutcome, UiIntentExecutionTransitionPosture,
        UiIntentInoperableCause, UiIntentOccupancyPosture, UiIntentOperabilityOutcome,
        UiIntentRouteResolution, UiIntentRouteSource,
    },
    interaction::{
        UiHostInteractionIngressOutcome, UiInteractionTransition, UiSemanticInteraction,
    },
    observation_report::{
        UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportOutcome,
        WorthUiHostObservationSessionExt,
    },
};
use worth_ui_host_contract::WorthUiHostMechanicsAdapter;
use worth_ui_host_egui::UiEguiRawInputIngressOutcome;
use worth_ui_runtime::facade::{
    entry::UiMountedAllocationMeasurementRequest,
    host::{
        UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
        UiHostMeasurementNormalizationContext,
    },
    measurement_exchange::{UiMeasurementEvidenceFamily, UiViewportExtentRequest},
    mounted::{
        UiHostSurfacePresentationMode, UiMountedFrameOutcome, UiMountedHitTestMechanic,
        UiPresentationDeadline, UiSurfaceBindingGeneration,
    },
};
use worth_ui_test_support::{
    WorthUiActiveSessionCertificationExt, WorthUiMountedAllocationCertificationExt,
    WorthUiMountedIdentityCertificationExt, WorthUiMountedPublicationCertificationExt,
};

use crate::{
    filesystem_mounted_world::prepare_frame,
    host_observation_fixture::{batch, report, source},
    intent::operability::PrimaryIntent,
    mounted_application_lifecycle::{
        known_empty_surface_world::profile,
        published_mounted_world::{presented_epoch, PresentedObservationBasis},
    },
};

pub(super) struct OrderingInteractionWorld {
    pub(super) session: worth_ui::facade::app::WorthUiActiveApplicationSession,
    host: worth_ui_host_egui::WorthUiHostEgui,
    pub(super) binding: UiSurfaceBindingGeneration,
    pub(super) presentation:
        worth_ui::facade::observation_report::UiHostObservationPresentationBasis,
    pub(super) hit_rows: Box<[UiMountedHitTestMechanic]>,
}

impl OrderingInteractionWorld {
    pub(super) fn launch(
        app: worth_ui::facade::app::WorthUiApp,
        host: worth_ui_host_egui::WorthUiHostEgui,
    ) -> Self {
        let mut session = app.launch().expect("the IA-09 application launches");
        let surface = session.create_semantic_surface().unwrap();
        let binding = session
            .register_host_surface(
                surface,
                UiHostSurfacePresentationMode::NativeDisplay,
                profile(1),
            )
            .unwrap()
            .binding_generation();
        let nodes = session.graph().node_identities().collect::<Vec<_>>();
        for node in nodes {
            let handle = session.mounted_graph_node(node).unwrap();
            session.mount_instance(handle, surface).unwrap();
        }
        establish_allocation(&mut session);
        let prepared = prepare_frame(&mut session).expect("the IA-09 mounted frame prepares");
        let hit_rows = prepared.surfaces()[0]
            .projection()
            .hit_tests()
            .rows()
            .to_vec()
            .into_boxed_slice();
        let publication = match session.present_prepared_mounted_frame(
            prepared,
            UiPresentationDeadline::at_tick(1_000),
            0,
        ) {
            UiMountedFrameOutcome::Published(publication) => publication,
            UiMountedFrameOutcome::Unchanged(_) => panic!("the IA-09 frame was unchanged"),
            UiMountedFrameOutcome::Reconciled(_) => panic!("the IA-09 frame reconciled"),
            UiMountedFrameOutcome::RejectedBeforeEffects(rejected) => {
                panic!(
                    "the IA-09 frame was rejected before effects: {:?}",
                    rejected.rejections()
                )
            }
            UiMountedFrameOutcome::InFlight(_) => panic!("the IA-09 frame remained in flight"),
            UiMountedFrameOutcome::PresentationIndeterminate(_) => {
                panic!("the IA-09 frame became indeterminate")
            }
            UiMountedFrameOutcome::RetentionDenied(_) => {
                panic!("the IA-09 frame retention was denied")
            }
            UiMountedFrameOutcome::AdmissionDenied(_) => {
                panic!("the IA-09 frame admission was denied")
            }
            UiMountedFrameOutcome::CompletionDenied(_) => {
                panic!("the IA-09 frame completion was denied")
            }
            UiMountedFrameOutcome::Superseded(_) => {
                panic!("the IA-09 frame was unexpectedly superseded")
            }
        };
        let presentation =
            worth_ui::facade::observation_report::UiHostObservationPresentationBasis::new(
                publication.frame(),
                binding,
                presented_epoch(&session, publication.frame(), binding),
            );
        Self {
            session,
            host,
            binding,
            presentation,
            hit_rows,
        }
    }
}

pub(super) struct CompetingInteraction {
    pub(super) target: worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity,
    pub(super) occupied: bool,
}

pub(super) fn start_effecting_intent(
    world: &mut OrderingInteractionWorld,
) -> worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity {
    let activation = native_activation(world);
    let target = activation.target().mounted_instance();
    let outcome = evaluate_activation(world, activation);
    let admitted = match world.session.admit_intent(
        UiIntentDefinition::<PrimaryIntent>::application_effect(),
        outcome,
    ) {
        UiIntentAdmissionDecision::Admitted(admitted) => admitted,
        _ => panic!("the first exact native activation must admit"),
    };
    assert!(matches!(
        world.session.dispatch_admitted_intent(
            admitted,
            crate::intent::execution::execution_deadline(60_000),
        ),
        UiIntentExecutionDispatchOutcome::AttemptPrepared(_)
    ));
    assert_eq!(
        only_transition(advance(world, 1)).posture(),
        UiIntentExecutionTransitionPosture::Started
    );
    assert_eq!(
        only_transition(advance(world, 2)).posture(),
        UiIntentExecutionTransitionPosture::PendingEffectMayHaveBegun
    );
    target
}

pub(super) fn prepare_competing_interaction(
    world: &mut OrderingInteractionWorld,
) -> CompetingInteraction {
    let activation = native_activation(world);
    let target = activation.target().mounted_instance();
    let outcome = evaluate_activation(world, activation);
    let UiIntentOperabilityOutcome::Inoperable(candidate) = outcome else {
        panic!("the same target route must remain occupied by the effecting intent")
    };
    assert_eq!(
        candidate.decision().occupancy(),
        UiIntentOccupancyPosture::InFlight
    );
    assert_eq!(
        candidate.decision().primary_cause(),
        Some(UiIntentInoperableCause::Occupied)
    );
    CompetingInteraction {
        target,
        occupied: true,
    }
}

fn evaluate_activation(
    world: &mut OrderingInteractionWorld,
    activation: worth_ui::facade::interaction::UiActivateInteraction,
) -> UiIntentOperabilityOutcome {
    let route = match world
        .session
        .resolve_intent_route(UiIntentRouteSource::mounted_interaction(
            UiSemanticInteraction::Activate(activation),
        ))
        .expect("the native activation resolves through the mounted route owner")
    {
        UiIntentRouteResolution::Product(route) => route,
        UiIntentRouteResolution::Confirmation(_) => {
            panic!("the IA-09 product route cannot become confirmation routing")
        }
    };
    let payload = world
        .session
        .prepare_intent_payload(route)
        .expect("the empty IA-09 payload prepares");
    world.session.evaluate_intent_operability(payload)
}

fn native_activation(
    world: &mut OrderingInteractionWorld,
) -> worth_ui::facade::interaction::UiActivateInteraction {
    let adapter = world.host.observe_native_input(&egui::RawInput {
        events: vec![pointer_button(true), pointer_button(false)],
        ..Default::default()
    });
    let UiEguiRawInputIngressOutcome::Retained(retained) = adapter else {
        panic!("the installed egui pointer translators retain the IA-09 activation")
    };
    assert_eq!(retained.report_count(), 2);
    let mut runtime = world
        .host
        .drain_mechanical_host_observations(world.session.host_session_identity().as_u64())
        .expect("the IA-09 native observation drain is bounded")
        .into_batches()
        .into_vec()
        .into_iter()
        .map(|batch| world.session.admit_host_interaction_batch(batch))
        .collect::<Vec<_>>();
    assert_eq!(runtime.len(), 1);
    let UiHostInteractionIngressOutcome::Applied(receipt) = runtime.remove(0) else {
        panic!("the native pointer batch reaches the production interaction owner")
    };
    receipt
        .into_transitions()
        .into_vec()
        .into_iter()
        .find_map(|transition| match transition {
            UiInteractionTransition::Semantic(UiSemanticInteraction::Activate(activation)) => {
                Some(activation)
            }
            _ => None,
        })
        .expect("one native press/release pair mints one activation")
}

fn pointer_button(pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: egui::pos2(10.0, 20.0),
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

pub(super) fn validated_viewport(
    world: &mut OrderingInteractionWorld,
    target: worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity,
) -> worth_ui::facade::observation_report::UiValidatedHostObservationBatch {
    let row = world
        .hit_rows
        .iter()
        .find(|row| row.mounted_instance() == target)
        .expect("the exact native target has one mounted hit row");
    let basis = PresentedObservationBasis {
        frame: world.presentation.frame(),
        epoch: world.presentation.epoch(),
        instance: row.mounted_instance(),
        receipt: row.node_receipt(),
    };
    let raw = batch(
        source(&world.session, world.binding, &basis),
        (5, 5),
        UiHostObservationLoss::Complete,
        vec![report(
            5,
            UiHostObservationPayload::Viewport {
                width_subpixels: 321_000,
                height_subpixels: 129_000,
            },
            &basis,
        )],
    );
    match world.session.validate_host_observation_batch(raw) {
        UiHostObservationReportOutcome::Validated(batch) => batch,
        other => panic!("the exact post-interaction viewport report validates: {other:?}"),
    }
}

pub(super) fn advance(
    world: &mut OrderingInteractionWorld,
    tick: u64,
) -> worth_ui::facade::intent::UiIntentExecutionAdvanceReport {
    match world
        .session
        .advance_intent_executions(crate::intent::execution::execution_reading(tick))
    {
        UiIntentExecutionAdvanceOutcome::Advanced(report) => report,
        UiIntentExecutionAdvanceOutcome::Stopped(stop) => {
            panic!("the IA-09 monotonic execution turn stopped: {stop:?}")
        }
    }
}

pub(super) fn only_transition(
    report: worth_ui::facade::intent::UiIntentExecutionAdvanceReport,
) -> worth_ui::facade::intent::UiIntentExecutionTransition {
    let mut transitions = report.into_transitions().into_vec();
    assert_eq!(transitions.len(), 1);
    transitions.pop().unwrap()
}

pub(super) fn native_host() -> worth_ui_host_egui::WorthUiHostEgui {
    let context = egui::Context::default();
    let _ = context.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(320.0, 128.0),
            )),
            ..Default::default()
        },
        |_| {},
    );
    worth_ui_host_egui::WorthUiHostEgui::new(context)
}

fn establish_allocation(session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession) {
    let capability = session.host_measurement_capability();
    let assumptions = UiHostMeasurementAssumptionProfile::from_capability_report(
        capability.capability_report(),
        1,
        2,
        3,
        4,
    );
    let receipt = session
        .establish_mounted_allocation_catalog(
            1,
            [UiMountedAllocationMeasurementRequest::new(
                UiMeasurementEvidenceFamily::ViewportExtent,
                UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
                UiHostMeasurementNormalizationContext::viewport_logical_exact(assumptions),
            )],
        )
        .expect("the IA-09 host viewport establishes production allocation");
    assert_eq!(receipt.committed().receipts().len(), 2);
}
