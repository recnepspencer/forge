use std::collections::BTreeSet;

use worth_ui::facade::{
    CommandProjectionSelectionMode, WorthUiHeaderFrameRebindStatus, WorthUiPageHostRebindStatus,
    WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::{
    ValidationAppProofSnapshot, ValidationMixedReloadStormFamily, ValidationMixedReloadStormProof,
    ValidationMixedReloadStormStatus,
};

mod validation_app_reload_fixture;

use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn file_backed_mixed_product_reload_storm_replays_with_visible_mixed_status() {
    let first = run_mixed_product_storm();
    let second = run_mixed_product_storm();

    let storm = first
        .mixed_reload_storm()
        .expect("storm proof should appear once the log contains a mixed sequence");
    let replay = ValidationMixedReloadStormProof::certify_replay(
        storm,
        second
            .mixed_reload_storm()
            .expect("replayed storm should produce the same proof surface"),
    )
    .expect("mixed product storm should converge under replay");

    assert_eq!(
        storm
            .steps()
            .iter()
            .map(|step| (step.family(), step.status()))
            .collect::<Vec<_>>(),
        vec![
            (
                ValidationMixedReloadStormFamily::Source,
                ValidationMixedReloadStormStatus::Activated,
            ),
            (
                ValidationMixedReloadStormFamily::Command,
                ValidationMixedReloadStormStatus::Activated,
            ),
            (
                ValidationMixedReloadStormFamily::CommandProjection,
                ValidationMixedReloadStormStatus::Activated,
            ),
            (
                ValidationMixedReloadStormFamily::Component,
                ValidationMixedReloadStormStatus::Activated,
            ),
            (
                ValidationMixedReloadStormFamily::Appearance,
                ValidationMixedReloadStormStatus::Activated,
            ),
            (
                ValidationMixedReloadStormFamily::Density,
                ValidationMixedReloadStormStatus::EquivalentNoOp,
            ),
            (
                ValidationMixedReloadStormFamily::Appearance,
                ValidationMixedReloadStormStatus::Denied,
            ),
        ]
    );

    let posture = storm.posture();
    assert!(posture.is_mixed());
    assert_eq!(posture.activated_step_count(), 5);
    assert_eq!(posture.equivalent_step_count(), 1);
    assert_eq!(posture.denied_step_count(), 1);

    let counters = storm.projection_counters();
    assert_eq!(
        counters.rebuild_attempt_count(),
        counters.dependency_intersection_count()
    );
    assert_eq!(
        counters.rebuilt_frame_count(),
        counters.rebuild_attempt_count()
    );

    let roster = storm.projection_roster();
    let expected_projection_ids = BTreeSet::from([
        "worth-ui.dropdown:validation.header.menu.edit".to_owned(),
        "worth-ui.dropdown:validation.header.menu.file".to_owned(),
        "worth-ui.dropdown:validation.header.menu.help".to_owned(),
        "worth-ui.dropdown:validation.header.menu.terminal".to_owned(),
        "worth-ui.header.appearance".to_owned(),
        "worth-ui.header.theme".to_owned(),
        "worth-ui.page-host.HeaderProofPage".to_owned(),
    ]);
    assert_eq!(roster.rebuilt_projection_ids(), expected_projection_ids);
    assert_eq!(roster.preserved_projection_ids(), BTreeSet::new());
    assert_eq!(roster.denied_projection_ids(), expected_projection_ids);
    let visible_summary = first
        .visible_mixed_reload_storm()
        .expect("mixed storm should project a visible summary");
    assert_eq!(visible_summary.heading(), "Mixed reload storm");
    assert!(visible_summary
        .projection_rows()
        .iter()
        .any(|row| row.projection_identity() == "worth-ui.page-host.HeaderProofPage"));

    let source_step = step(
        storm,
        ValidationMixedReloadStormFamily::Source,
        ValidationMixedReloadStormStatus::Activated,
    );
    assert!(source_step.changed_facts().iter().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::PrimitiveInteraction
            && fact.identity() == "worth.surface.preview.primitive.proof"
    }));
    assert!(source_step.changed_facts().iter().any(|fact| {
        fact.family() == WorthUiRuntimeFactFamily::AuthoredSurfaceProps
            && fact.identity() == "worth.surface.preview.primitive.proof"
    }));
    assert_eq!(
        source_step
            .page_host_rebind()
            .expect("source activation should carry page-host proof")
            .status(),
        WorthUiPageHostRebindStatus::ReboundAfterActivation
    );

    let density_step = step(
        storm,
        ValidationMixedReloadStormFamily::Density,
        ValidationMixedReloadStormStatus::EquivalentNoOp,
    );
    assert!(density_step.changed_facts().is_empty());
    assert!(
        density_step.header_rebind().is_none(),
        "equivalent density row must not borrow the appearance rebuild receipt"
    );

    let denied_step = step(
        storm,
        ValidationMixedReloadStormFamily::Appearance,
        ValidationMixedReloadStormStatus::Denied,
    );
    assert_eq!(
        denied_step
            .header_rebind()
            .expect("denied appearance step should preserve header proof")
            .status(),
        WorthUiHeaderFrameRebindStatus::PreservedDeniedReload
    );

    let component_step = step(
        storm,
        ValidationMixedReloadStormFamily::Component,
        ValidationMixedReloadStormStatus::Activated,
    );
    assert_eq!(component_step.changed_facts().len(), 1);

    let file_menu = first
        .header()
        .menus()
        .iter()
        .find(|menu| menu.title() == "File")
        .expect("file menu should remain visible");
    let save_command = file_menu
        .commands()
        .iter()
        .find(|command| command.command_id() == "validation.command.file.save")
        .expect("file menu should keep the save command visible");
    assert_eq!(save_command.label(), "Save Everything");
    assert_eq!(
        file_menu.selection_mode(),
        CommandProjectionSelectionMode::MultiSelect
    );
    assert_eq!(
        first.page_slot_interaction().slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(
        first.header().applied_style().menu_min_width_points(),
        260.0
    );
    assert_eq!(first.header().applied_style().container_margin().left, 8);

    assert_eq!(replay.step_count(), storm.steps().len());
    assert_eq!(
        replay.final_active_artifact_digest(),
        storm.final_active_artifact_digest()
    );
    assert_eq!(
        replay.final_capability_snapshot_digest(),
        storm.final_capability_snapshot_digest()
    );
    assert_eq!(
        replay.final_authoring_snapshot_digest(),
        storm.final_authoring_snapshot_digest()
    );
    assert_eq!(
        replay.final_last_valid_artifact_digest(),
        storm.final_last_valid_artifact_digest()
    );
    assert_eq!(
        replay.final_last_valid_plan_digest(),
        storm.final_last_valid_plan_digest()
    );
}

#[test]
fn mixed_reload_storm_summary_only_appears_for_a_qualified_phase17_sequence() {
    let single_edit_fixture = ValidationAppReloadFixture::new();
    let mut single_edit_app = single_edit_fixture.build_app();

    assert!(single_edit_app
        .proof_snapshot()
        .mixed_reload_storm()
        .is_none());

    single_edit_fixture.write_command(
        "\
validation.command.file.new = New File
validation.command.file.open = Open File
validation.command.file.save = Save Everything
validation.command.file.exit = Exit
validation.command.edit.undo = Undo
validation.command.edit.redo = Redo
validation.command.edit.cut = Cut
validation.command.edit.copy = Copy
validation.command.edit.paste = Paste
validation.command.terminal.new = New Terminal
validation.command.terminal.split = Split Terminal
validation.command.terminal.clear = Clear Terminal
validation.command.help.palette = Command Palette
validation.command.help.docs = Worth UI Docs
validation.command.help.about = About Worth UI
",
    );
    single_edit_app.run_one_reload_observation_cycle();
    assert!(
        single_edit_app
            .proof_snapshot()
            .mixed_reload_storm()
            .is_none(),
        "a single-family edit must not claim a mixed reload storm"
    );

    let qualified_fixture = ValidationAppReloadFixture::new();
    let mut qualified_app = qualified_fixture.build_app();
    apply_mixed_product_storm(&qualified_fixture, &mut qualified_app);
    assert!(qualified_app
        .proof_snapshot()
        .mixed_reload_storm()
        .is_some());
    assert!(qualified_app
        .proof_snapshot()
        .visible_mixed_reload_storm()
        .is_some());

    qualified_fixture.write_theme("validation.theme.header.panel = #102030\n");
    qualified_app.run_one_reload_observation_cycle();
    assert!(
        qualified_app
            .proof_snapshot()
            .mixed_reload_storm()
            .is_none(),
        "the summary must clear once the latest suffix is no longer a Phase 17-shaped storm"
    );
    assert!(qualified_app
        .proof_snapshot()
        .visible_mixed_reload_storm()
        .is_none());
}

fn run_mixed_product_storm() -> ValidationAppProofSnapshot {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();

    apply_mixed_product_storm(&fixture, &mut app);

    app.proof_snapshot()
}

fn apply_mixed_product_storm(
    fixture: &ValidationAppReloadFixture,
    app: &mut worth_ui_validation_app::ValidationWorkbenchApp,
) {
    fixture.write_source(&alternate_surface_source());
    app.run_one_reload_observation_cycle();

    fixture.write_command(
        "\
validation.command.file.new = New File
validation.command.file.open = Open File
validation.command.file.save = Save Everything
validation.command.file.exit = Exit
validation.command.edit.undo = Undo
validation.command.edit.redo = Redo
validation.command.edit.cut = Cut
validation.command.edit.copy = Copy
validation.command.edit.paste = Paste
validation.command.terminal.new = New Terminal
validation.command.terminal.split = Split Terminal
validation.command.terminal.clear = Clear Terminal
validation.command.help.palette = Command Palette
validation.command.help.docs = Worth UI Docs
validation.command.help.about = About Worth UI
",
    );
    app.run_one_reload_observation_cycle();

    fixture.write_command_projection(
        "\
validation.header.menu.file = multi
validation.header.menu.edit = multi
validation.header.menu.terminal = single
validation.header.menu.help = single
",
    );
    app.run_one_reload_observation_cycle();

    fixture.write_component(
        "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive
command_binding_slots = validation.command.header.refresh
",
    );
    app.run_one_reload_observation_cycle();

    fixture.write_appearance(
        "\
validation.appearance.header.menu_min_width = 260px
validation.appearance.header.panel_shadow = #00000066 0px 1px 3px 0px
validation.appearance.header.font_size = 13px
validation.appearance.header.border_width = 1px
",
    );
    fixture.write_density(
        "\
validation.density.header.container_padding = 4.0px 8.0px 4.0px 8.0px
validation.density.header.control_spacing = 8.0px
validation.density.header.row_padding = 1.0px 6.0px
",
    );
    app.run_one_reload_observation_cycle();

    fixture.write_appearance("validation.appearance.header.font_size = #102030\n");
    app.run_one_reload_observation_cycle();
}

fn alternate_surface_source() -> String {
    worth_ui_validation_app::sample_source::VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_payload \"submit.secondary\"",
        "interaction_payload \"submit.storm\"",
    )
}

fn step(
    storm: &ValidationMixedReloadStormProof,
    family: ValidationMixedReloadStormFamily,
    status: ValidationMixedReloadStormStatus,
) -> &worth_ui_validation_app::ValidationMixedReloadStormStep {
    storm
        .steps()
        .iter()
        .find(|step| step.family() == family && step.status() == status)
        .expect("expected mixed storm step should exist")
}
