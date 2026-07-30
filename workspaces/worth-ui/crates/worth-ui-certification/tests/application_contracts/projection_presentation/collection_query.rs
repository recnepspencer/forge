use worth_ui::facade::observation::UiChangeClassificationOutcome;
use worth_ui::facade::rebind::{
    UiRebindExecutionPolicy, UiRebindExecutionRequest, UiRebindOutcome,
};
use worth_ui_dsl::{
    WorthUiArtifactInputBodyAtom, WorthUiProjectionCollectionPolicy,
    WorthUiProjectionCollectionSelection, WorthUiProjectionLifecycle,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
use worth_ui_host_contract::UiSemanticTextSlot;
use worth_ui_query_binding::{
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionBudget,
    UiCollectionProjectionOpenOutcome, UiCollectionProjectionRegistration,
    UiProjectionFieldRequirement, UiProjectionObservation, WorthUiQueryWorkspaceExt,
};
use worth_ui_runtime::facade::host::{
    UiHeadlessRecorderCapacity, UiHeadlessUnperformedEffect, WorthUiHeadlessRecorder,
};

use super::scalar_query_only::{
    component_descriptor, mount_and_allocate, status_region_descriptor, text_token_descriptor,
    ACTIVE_COMPONENT, PROJECTION, STATUS_REGION, TEXT_COLOR,
};

#[test]
fn real_query_collection_snapshot_and_patch_publish_keyed_semantic_text() {
    let recorder = WorthUiHeadlessRecorder::with_viewport_extent(
        UiHeadlessRecorderCapacity::production_default(),
        worth_ui::facade::measurement_exchange::UiViewportExtentObservation {
            width: 320.0,
            height: 128.0,
        },
    );
    let (mut workspace, entities) =
        worth_ui_query_binding::certification::seeded_collection_projection_workspace(
            vec![
                ("pulse.alpha".to_owned(), "Alpha".to_owned()),
                ("pulse.bravo".to_owned(), "Bravo".to_owned()),
            ],
            worth_ui_query_binding::certification::WorthUiCollectionProjectionSeedPosture::Complete,
        );
    let domain = workspace.worth_ui().expect("Worth UI domain installed");
    let registration = collection_registration(&domain);
    let mut session = collection_app(registration.clone(), recorder.clone())
        .launch()
        .expect("collection projection application launches");
    let mounted_instances = mount_and_allocate(&mut session);
    let generation = session.generation_identity().clone();

    let binding = match registration.admit(&workspace) {
        UiCollectionProjectionBindingAdmission::Ready(binding) => binding,
        UiCollectionProjectionBindingAdmission::Stopped(stop) => {
            panic!("real collection binding admits: {stop:?}")
        }
    };
    let opened = match binding.open(
        UiCollectionProjectionBudget::new(2, 2, 0, 1024).unwrap(),
        &mut workspace,
    ) {
        UiCollectionProjectionOpenOutcome::Opened(opened) => opened,
        UiCollectionProjectionOpenOutcome::Stopped(stop) => {
            panic!("real collection projection opens: {stop:?}")
        }
    };
    let (mut live, snapshot) = opened.into_parts();
    publish_collection_fact(&mut session, snapshot.into_observation(), 3131);

    worth_ui_query_binding::certification::update_projection_status(
        &mut workspace,
        entities[1].clone(),
        "Bravo updated",
    );
    let patch = match live.refresh(&mut workspace).unwrap() {
        worth_ui_query_binding::UiCollectionProjectionRefreshOutcome::Applied(receipt) => {
            receipt.into_fact()
        }
        worth_ui_query_binding::UiCollectionProjectionRefreshOutcome::NoSemanticDelivery => {
            panic!("the changed Query row produces one exact patch")
        }
    };
    publish_collection_fact(&mut session, patch.into_observation(), 3132);

    let transcripts = recorder.observed_transcripts();
    assert_eq!(transcripts.len(), 2);
    assert_collection_transcript(
        &transcripts[0],
        &mounted_instances,
        &entities,
        ["Alpha", "Bravo"],
    );
    assert_collection_transcript(
        &transcripts[1],
        &mounted_instances,
        &entities,
        ["Alpha", "Bravo updated"],
    );
    assert_ne!(transcripts[0].frame(), transcripts[1].frame());
    assert_ne!(
        transcripts[0].semantic_text()[0].content_generation(),
        transcripts[1].semantic_text()[0].content_generation()
    );
    assert!(session.generation_identity() == &generation);

    match live.close(&mut workspace) {
        worth_ui_query_binding::UiLiveCollectionProjectionCloseOutcome::Closed(_) => {}
        worth_ui_query_binding::UiLiveCollectionProjectionCloseOutcome::Stopped(stop) => {
            panic!("live collection closes: {:?}", stop.query_error())
        }
    }
    let shutdown = session.shutdown();
    assert!(shutdown.rebind().is_empty());
    assert!(shutdown.mounted_presentation().is_empty());
}

fn publish_collection_fact(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    observation: worth_ui_query_binding::UiCollectionProjectionObservation,
    request: u64,
) {
    let plan = collection_plan(session, observation);
    let prepared = session
        .prepare_rebind(plan, UiRebindExecutionRequest::new(request))
        .expect("collection content prepares against exact mounted authority");
    match prepared.execute(request) {
        UiRebindOutcome::Published(receipt) => {
            assert!(receipt.application_publication().is_none());
            assert!(receipt.mounted_publication().is_some());
        }
        _ => panic!("collection content publishes atomically"),
    }
}

pub(crate) fn collection_plan(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
    observation: worth_ui_query_binding::UiCollectionProjectionObservation,
) -> worth_ui::facade::rebind::UiRebindPlan {
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_projection_query(UiProjectionObservation::Collection(observation))
        .unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("the real collection fact changes mounted presentation"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .unwrap()
}

fn assert_collection_transcript(
    transcript: &worth_ui_runtime::facade::host::UiHeadlessMountedFrameTranscript,
    mounted_instances: &[worth_ui_runtime::facade::mounted::UiMountedInstanceIdentity],
    entities: &[worth_query::facade::foundation::WorthQueryEntityIdentity],
    expected_values: [&str; 2],
) {
    assert_eq!(transcript.semantic_text().len(), 3);
    let values = transcript
        .semantic_text()
        .iter()
        .filter(|row| matches!(row.slot(), UiSemanticTextSlot::CollectionValue { .. }))
        .collect::<Vec<_>>();
    assert_eq!(
        values.iter().map(|row| row.text()).collect::<Vec<_>>(),
        expected_values
    );
    assert_eq!(
        values
            .iter()
            .map(|row| {
                row.collection_row()
                    .expect("collection value retains row correlation")
                    .identity_for_reporting()
                    .to_owned()
            })
            .collect::<Vec<_>>(),
        entities
            .iter()
            .map(|entity| {
                entity
                    .evidence_identity()
                    .terminal_projection_for_reporting()
                    .to_owned()
            })
            .collect::<Vec<_>>()
    );
    assert!(values
        .iter()
        .all(|row| mounted_instances.contains(&row.mounted_instance())));
    let posture = transcript
        .semantic_text()
        .iter()
        .find(|row| row.slot() == UiSemanticTextSlot::Posture)
        .expect("collection posture is independently mounted");
    assert_eq!(posture.text(), "CURRENT · COMPLETE");
    assert!(posture.collection_row().is_none());
    assert_eq!(
        transcript.unperformed_effects(),
        &[UiHeadlessUnperformedEffect::NativePaint {
            filled_rect_count: 0,
            semantic_text_count: 3,
            preview_node_count: 0,
        }]
    );
}

pub(crate) fn collection_app(
    registration: UiCollectionProjectionRegistration,
    recorder: WorthUiHeadlessRecorder,
) -> worth_ui::facade::app::WorthUiApp {
    worth_ui::facade::app::WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .register_component(component_descriptor(ACTIVE_COMPONENT))
        .register_mosaic_region_kind(status_region_descriptor())
        .register_theme_token(text_token_descriptor())
        .register_collection_projection(registration)
        .expect("product collection projection registers")
        .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([
            collection_module(false),
        ]))
        .with_host(recorder)
        .freeze()
        .expect("collection content application freezes")
}

pub(crate) fn collection_module(with_region: bool) -> WorthUiRustAuthoredArtifactInputModule {
    let mut body = vec![
        WorthUiArtifactInputBodyAtom::Identifier("content".to_owned()),
        WorthUiArtifactInputBodyAtom::Identifier("projection".to_owned()),
        WorthUiArtifactInputBodyAtom::Identifier(PROJECTION.to_owned()),
    ];
    if with_region {
        body.extend([
            WorthUiArtifactInputBodyAtom::Identifier("region".to_owned()),
            WorthUiArtifactInputBodyAtom::Identifier(STATUS_REGION.to_owned()),
            WorthUiArtifactInputBodyAtom::LeftBrace,
            WorthUiArtifactInputBodyAtom::RightBrace,
        ]);
    }
    WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms_and_authored_identity(
            ACTIVE_COMPONENT,
            "platform-pulse-projected-collection-component",
            body,
        )
        .with_token(TEXT_COLOR, "#ffffff")
        .try_with_query_collection_text(
            PROJECTION,
            PROJECTION,
            "identity.id",
            WorthUiProjectionCollectionSelection::new(
                ["status"],
                WorthUiProjectionLifecycle::Live,
                WorthUiProjectionCollectionPolicy::new(false, false),
            ),
        )
        .unwrap()
}

pub(crate) fn collection_registration(
    domain: &worth_ui_query_binding::WorthUiInstalledQueryDomain,
) -> UiCollectionProjectionRegistration {
    UiCollectionProjectionRegistration::text(
        domain.projection_view(PROJECTION).unwrap(),
        UiProjectionFieldRequirement::declared("identity.id").unwrap(),
        [UiProjectionFieldRequirement::declared("status").unwrap()],
        false,
        false,
    )
    .unwrap()
}
