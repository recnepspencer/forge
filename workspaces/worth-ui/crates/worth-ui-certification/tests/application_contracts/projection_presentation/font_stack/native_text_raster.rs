use super::*;
use worth_ui_runtime::certification_support::ScriptedPresentationHost;

#[test]
fn authored_text_crosses_the_mounted_native_raster_transaction_and_pins_its_layout() {
    let (fonts, _, primary, secondary, _, _, _, _) = application_fonts();
    let primary_style = style([primary, secondary]);
    let secondary_style = style([secondary, primary]);
    let host = ScriptedPresentationHost::native_display();
    host.set_capabilities(
        worth_ui_host_contract::WorthUiHostCapabilityReport::available(vec![
            worth_ui_host_contract::WorthUiHostCapability::NativePaint,
            worth_ui_host_contract::WorthUiHostCapability::MountedFrameRecording,
            worth_ui_host_contract::WorthUiHostCapability::ViewportObservation,
            worth_ui_host_contract::WorthUiHostCapability::DpiObservation,
        ]),
    );
    let (mut query, completion) = ScalarLifecycleWorld::standard(NodeId::new(41_411, 0), VALUE);
    let component = component_descriptor(ACTIVE_COMPONENT).with_semantic_text(
        ComponentSemanticTextContract::spanned(
            ThemeTokenId::new(super::super::scalar_query_only::TEXT_COLOR).unwrap(),
            1,
            [
                span(
                    0,
                    6,
                    super::super::scalar_query_only::TEXT_COLOR,
                    primary_style.clone(),
                ),
                span(6, 10, ACCENT_COLOR, secondary_style),
                span(10, VALUE.len() as u32, ACCENT_COLOR, primary_style),
            ],
        )
        .unwrap(),
    );
    let app = worth_ui::facade::app::WorthUi::app()
        .with_change_profile(worth_ui::facade::rebind::UiChangeProfile::platform_pulse())
        .with_font_collection(Arc::new(fonts))
        .register_component(component)
        .register_component(component_descriptor(CANDIDATE_COMPONENT))
        .register_mosaic_region_kind(status_region_descriptor())
        .register_theme_token(text_token_descriptor())
        .register_theme_token(accent_token_descriptor())
        .register_scalar_projection(scalar_registration(&query))
        .unwrap()
        .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([
            projection_module_with_additional_token(ACTIVE_COMPONENT, ACCENT_COLOR, "#f7812f"),
        ]))
        .freeze()
        .map(|application| {
            worth_ui_runtime::facade::entry::WorthUiCertificationApplicationTransition::activate_scripted_presentation(
                application,
                host.clone(),
            )
        })
        .unwrap();
    let mut session = app.launch().unwrap();
    mount_and_allocate(&mut session);

    let pending = query.initial().into_fact_and_predecessor().0;
    let current = query.advance(
        BridgeMixedCauseOrderingInput::AsyncCompletion(completion),
        Some(pending),
    );
    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_projection_query(UiProjectionObservation::Scalar(
        current.into_fact_and_predecessor().0.into_observation(),
    ))
    .unwrap();
    let admitted = turn.seal().unwrap();
    let changed = match session.classify_observations(admitted).unwrap() {
        UiChangeClassificationOutcome::Changed(changed) => changed,
        _ => panic!("font-stack text must publish"),
    };
    let lifecycle = session
        .resolve_affected_scope(changed)
        .unwrap()
        .resolve_identity_lifecycle()
        .unwrap();
    let plan = session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .unwrap();
    let prepared = session
        .prepare_rebind(plan, UiRebindExecutionRequest::new(415))
        .unwrap();
    let UiRebindOutcome::RejectedBeforeEffects(receipt) = prepared.execute(1) else {
        panic!("Gate D raster completion must stop at the Gate E paint boundary");
    };
    assert_eq!(receipt.host_rejections().len(), 1);
    assert_eq!(
        receipt.host_rejections()[0].denial(),
        worth_ui_host_contract::UiHostSurfacePresentationDenial::TextAtlasPresentationDeferred
    );

    assert_eq!(host.text_raster_calls(), 1);
    assert!(host.text_rasterized_records() > 0);
    let pins = host.live_text_pins();
    assert!(!pins.is_empty());
    let mut layouts = Vec::new();
    for layout in pins.iter().map(|pin| pin.layout_identity()) {
        if !layouts.contains(&layout) {
            layouts.push(layout);
        }
    }
    assert_eq!(layouts.len(), 2);
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P5-ATLAS-PINNING-01\":{}}}",
        layouts.len()
    );
}
