use worth_ui::facade::{
    AppearanceTokenId, WorthUiCompileBoundaryPosture, WorthUiRuntimeChangeFamily,
    WorthUiRuntimeFactId, WorthUiSemanticSliceId,
};

mod validation_app_reload_fixture;

use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn one_authored_save_certifies_structural_and_appearance_truth_through_one_runtime_batch() {
    let fixture = ValidationAppReloadFixture::new();
    let mut first_app = fixture.build_app();
    fixture.write_source(&alternate_surface_source());
    fixture.write_appearance("validation.appearance.header.menu_min_width = 260px\n");
    let captured = first_app
        .run_one_reload_observation_cycle_with_capture()
        .expect("one authored save should capture the observed authored batch");
    let first = first_app.proof_snapshot();

    let replay_fixture = ValidationAppReloadFixture::new();
    let mut replay_app = replay_fixture.build_app();
    replay_app.replay_captured_authored_batch(&captured);
    let second = replay_app.proof_snapshot();

    let proof = first
        .authoring_truth_final_boss()
        .expect("one mixed authored save should expose the final boss proof");
    let replayed = second
        .authoring_truth_final_boss()
        .expect("replayed authored save should expose the same proof");

    assert_eq!(
        proof.authored_delta_digest(),
        replayed.authored_delta_digest()
    );
    assert!(
        worth_ui_validation_app::ValidationAuthoringTruthFinalBossReplayArtifact::certify_replay(
            proof.replay_artifact(),
            replayed.replay_artifact(),
        )
    );

    let rows = proof.changed_fact_rows();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().any(|row| {
        row.family() == WorthUiRuntimeChangeFamily::ValidationSource
            && row
                .changed_facts()
                .contains(&WorthUiRuntimeFactId::primitive_interaction(
                    "worth.surface.preview.primitive.proof",
                ))
    }));
    assert!(rows.iter().any(|row| {
        row.family() == WorthUiRuntimeChangeFamily::Capability
            && row
                .changed_facts()
                .contains(&WorthUiRuntimeFactId::appearance_token(
                    &AppearanceTokenId::new("validation.appearance.header.menu_min_width").unwrap(),
                ))
    }));

    assert_eq!(proof.counter_posture().family_row_count(), 2);
    assert_eq!(proof.counter_posture().denied_family_count(), 0);
    assert!(proof.projection_counters().dependency_intersection_count() > 0);

    let rebuilt = proof.projection_roster().rebuilt_projection_ids();
    let preserved = proof.projection_roster().preserved_projection_ids();
    assert!(rebuilt.contains("worth-ui.page-host.HeaderProofPage"));
    assert!(rebuilt.contains("worth-ui.header.appearance"));
    assert!(preserved.contains("worth-ui.header.theme"));
    assert!(rebuilt.is_disjoint(&preserved));
    assert!(proof.projection_roster().rows().iter().any(|row| {
        row.projection_identity() == "worth-ui.header.theme" && !row.rebuild_attempted()
    }));

    assert_eq!(
        first.page_slot_interaction().slots()[0].surface_id(),
        "worth.surface.preview.primitive.proof"
    );
    assert_eq!(
        first.header().applied_style().menu_min_width_points(),
        260.0
    );
    assert_eq!(
        proof.compile_boundary().posture(),
        WorthUiCompileBoundaryPosture::HotReloadWithinProductMeaning
    );
    assert!(proof
        .compile_boundary()
        .hot_reload_stays_within_product_meaning());
    assert!(proof
        .compile_boundary()
        .changed_slice_ids()
        .contains(&WorthUiSemanticSliceId::PrimitiveInteraction));
    assert!(proof
        .compile_boundary()
        .changed_slice_ids()
        .contains(&WorthUiSemanticSliceId::AppearanceField));
    assert!(!proof
        .compile_boundary()
        .changed_slice_ids()
        .contains(&WorthUiSemanticSliceId::NewRustComponentImplementation));

    let visible = first
        .visible_authoring_truth_final_boss()
        .expect("final boss should project a visible certification summary");
    assert_eq!(visible.heading(), "Authoring-truth final boss");
    assert!(visible
        .compile_boundary_line()
        .contains("HotReloadWithinProductMeaning"));
    assert!(visible
        .compile_boundary_line()
        .contains("PrimitiveInteraction"));
}

fn alternate_surface_source() -> String {
    worth_ui_validation_app::sample_source::VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_payload \"submit.secondary\"",
        "interaction_payload \"submit.final_boss\"",
    )
}
