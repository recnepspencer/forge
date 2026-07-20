use worth_ui::facade::app::{
    WorthUiActiveApplicationSession, WorthUiApplicationCutoverDenial,
    WorthUiExecutablePlanDecisionKind, WorthUiOrdinaryFrameTarget,
    WorthUiPendingApplicationCutover,
};
use worth_ui::facade::host::{
    WorthUiHeadlessHost, WorthUiHostOutputDisposition, WorthUiHostOutputEnvelope,
    WorthUiHostOutputLane, WorthUiHostOutputPayload,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_host_egui::WorthUiHostEgui;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn real_egui_frame_lowers_only_the_sealed_host_neutral_envelope() {
    let scenario = FilesystemApplicationLifecycleScenario::new("egui-host-execution");
    let workspace = FilesystemContractWorkspace::new("egui-host-execution");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
    );
    let capabilities = scenario.capability_application();
    let headless_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("headless source snapshot settles"),
        capabilities.capabilities(),
    );
    let egui_submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("egui source snapshot settles"),
        capabilities.capabilities(),
    );

    let mut headless = scenario
        .prepare_application_with_host(headless_submission, WorthUiHeadlessHost)
        .launch()
        .expect("headless peer launches");
    let headless_expected = headless.inspect_runtime();
    let headless_execution = headless
        .execute_framework_turn(|_| {})
        .into_execution()
        .unwrap_or_else(|_| panic!("headless framework turn"));
    let headless_frame = headless_execution
        .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
        .expect("headless frame executes");
    let headless_payload = headless_frame.output().payload();
    let headless_receipt = headless_frame.output().receipt_reference();
    drop(headless_frame);
    drop(headless_execution);

    let context = egui::Context::default();
    let mut egui_session = scenario
        .prepare_application_with_host(egui_submission, WorthUiHostEgui::new(context.clone()))
        .launch()
        .expect("egui peer launches");
    let expected = egui_session.inspect_runtime();
    let expected_host_session = egui_session.host_session_identity().as_u64();
    let mut observed: Option<WorthUiHostOutputEnvelope> = None;
    let native_output = context.run(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        },
        |_| {
            let execution = egui_session
                .execute_framework_turn(|_| {})
                .into_execution()
                .unwrap_or_else(|_| panic!("egui framework turn"));
            let frame = execution
                .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
                .expect("egui frame executes");
            assert_eq!(frame.disposition(), WorthUiHostOutputDisposition::Consumed);
            observed = Some(*frame.output());
        },
    );
    let envelope = observed.expect("adapter received one sealed envelope");
    let generation = envelope.generation();
    assert_eq!(generation.host_session_identity(), expected_host_session);
    assert_eq!(
        generation.active_artifact_digest(),
        expected.artifact_digest()
    );
    assert_eq!(
        generation.active_plan_digest(),
        expected.active_plan_digest()
    );
    assert_eq!(
        generation.active_artifact_digest(),
        headless_expected.artifact_digest(),
        "host choice must not change the admitted artifact"
    );
    assert_eq!(
        generation.active_plan_digest(),
        headless_expected.active_plan_digest(),
        "host choice must not change canonical plan meaning"
    );
    assert_eq!(
        expected.cross_lane_bundle().plan_digest(),
        headless_expected.cross_lane_bundle().plan_digest(),
        "both adapters must consume constituents from the same host-neutral bundle meaning"
    );
    assert_eq!(generation.frame_epoch(), expected.frame_epoch().as_u64());
    assert_eq!(envelope.payload(), headless_payload);
    assert_eq!(envelope.receipt_reference().lane(), headless_receipt.lane());
    assert_ne!(
        envelope.receipt_reference().digest(),
        headless_receipt.digest(),
        "equivalent envelope meaning must retain distinct session receipt authority"
    );
    assert_eq!(
        envelope.receipt_reference().lane(),
        WorthUiHostOutputLane::Ordinary
    );
    assert!(matches!(
        envelope.payload(),
        WorthUiHostOutputPayload::Ordinary(_)
    ));
    assert!(
        !native_output.shapes.is_empty(),
        "production egui adapter must perform native lowering"
    );

    let _ = egui_session.shutdown();
    let _ = headless.shutdown();
    workspace.close();
}

#[test]
fn real_egui_contact_changes_only_after_atomic_replacement_activation() {
    let scenario = FilesystemApplicationLifecycleScenario::new("egui-replacement");
    let workspace = FilesystemContractWorkspace::new("egui-replacement");
    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::current_source_text(),
    );
    let capabilities = scenario.capability_application();
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("predecessor source settles"),
        capabilities.capabilities(),
    );
    let context = egui::Context::default();
    let mut session = scenario
        .prepare_application_with_host(submission, WorthUiHostEgui::new(context.clone()))
        .launch()
        .expect("egui predecessor launches");
    let predecessor = run_egui_ordinary_frame(&context, &mut session);
    let predecessor_generation = session.generation_identity().clone();

    workspace.write(
        "app/main.wui",
        &FilesystemApplicationLifecycleScenario::candidate_source_text(),
    );
    let denied_pending = prepare_pending_replacement(&workspace, &session);
    let (_, foreign_catalog) = prepare_replacement_with_catalog(&workspace, &session);
    let boundary = activation_boundary(&mut session);
    let denial = match session.activate_prepared_replacement(
        denied_pending,
        foreign_catalog,
        boundary,
        None,
    ) {
        Ok(_) => panic!("foreign catalog cannot publish a candidate"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        WorthUiApplicationCutoverDenial::PreparedApplicationGraphMismatch
    ));
    assert_eq!(session.generation_identity(), &predecessor_generation);
    let after_denial = run_egui_ordinary_frame(&context, &mut session);
    assert_eq!(
        after_denial.envelope.generation(),
        predecessor.envelope.generation()
    );
    assert_eq!(after_denial.native_contact, predecessor.native_contact);

    let (prepared, catalog) = prepare_replacement_with_catalog(&workspace, &session);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("valid candidate lowers");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("valid candidate stages");
    let boundary = activation_boundary(&mut session);
    let cutover = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("valid candidate activates atomically");
    let cutover = cutover
        .into_activation()
        .expect("changed executable meaning publishes a successor");
    assert_eq!(
        cutover.plan_decision().kind(),
        WorthUiExecutablePlanDecisionKind::BoundedChangedRegions
    );
    assert_ne!(cutover.active_generation(), &predecessor_generation);

    let successor = run_egui_ordinary_frame(&context, &mut session);
    assert_ne!(
        successor.envelope.generation().active_plan_digest(),
        predecessor.envelope.generation().active_plan_digest()
    );
    assert_ne!(successor.native_contact, predecessor.native_contact);
    assert!(successor.native_contact.contains(
        &successor
            .envelope
            .generation()
            .active_plan_digest()
            .to_string()
    ));

    let _ = session.shutdown();
    workspace.close();
}

struct EguiFrameContact {
    envelope: WorthUiHostOutputEnvelope,
    native_contact: String,
}

fn run_egui_ordinary_frame(
    context: &egui::Context,
    session: &mut WorthUiActiveApplicationSession,
) -> EguiFrameContact {
    let mut envelope = None;
    let output = context.run(raw_input(), |_| {
        let execution = session
            .execute_framework_turn(|_| {})
            .into_execution()
            .unwrap_or_else(|_| panic!("egui framework turn"));
        let frame = execution
            .execute_ordinary_frame(WorthUiOrdinaryFrameTarget::root_shell())
            .expect("egui ordinary frame");
        envelope = Some(*frame.output());
    });
    let native_contact = output
        .shapes
        .iter()
        .find_map(|clipped| match &clipped.shape {
            egui::Shape::Text(text) => Some(text.galley.job.text.clone()),
            _ => None,
        })
        .expect("adapter emits one native text contact");
    EguiFrameContact {
        envelope: envelope.expect("frame emits one sealed envelope"),
        native_contact,
    }
}

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(800.0, 600.0),
        )),
        ..Default::default()
    }
}

fn prepare_replacement_with_catalog(
    workspace: &FilesystemContractWorkspace,
    session: &WorthUiActiveApplicationSession,
) -> (
    Box<worth_ui::facade::app::WorthUiPreparedApplicationReplacement>,
    worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta,
) {
    let submission = FilesystemApplicationLifecycleScenario::lower_snapshot(
        WorthUiFilesystemSourceProvider::new(workspace.root())
            .read()
            .expect("candidate source settles from disk"),
        session.capabilities(),
    );
    let mut prepared = session
        .prepare_replacement(submission)
        .expect("candidate prepares");
    let catalog = admit_candidate_catalog(&mut prepared);
    (prepared, catalog)
}

fn prepare_pending_replacement(
    workspace: &FilesystemContractWorkspace,
    session: &WorthUiActiveApplicationSession,
) -> WorthUiPendingApplicationCutover {
    let (prepared, _) = prepare_replacement_with_catalog(workspace, session);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("candidate lowers");
    session
        .stage_prepared_replacement(lowered)
        .expect("candidate stages")
}

fn activation_boundary(
    session: &mut WorthUiActiveApplicationSession,
) -> worth_ui::facade::runtime::WorthUiFrameBoundary {
    session
        .execute_framework_turn(|_| {})
        .into_completion()
        .into_execution()
        .unwrap_or_else(|_| panic!("activation boundary turn"))
        .into_activation_boundary()
}
