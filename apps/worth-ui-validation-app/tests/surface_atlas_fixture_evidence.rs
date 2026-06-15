use worth_ui_harness::facade::HarnessEvidenceFamily;
use worth_ui_validation_app::pages::surface_atlas::{
    FixtureEvidenceCompletionDenial, FixtureEvidenceLabelDenial, SurfaceAtlasFixtureEvidence,
};

#[test]
fn surface_atlas_fixture_data_is_labeled_and_cannot_mark_success() {
    let fixture = SurfaceAtlasFixtureEvidence::sample_only();

    assert!(
        fixture.label().contains("SampleOnly"),
        "atlas fixture data must be visibly labeled as sample-only"
    );
    assert_eq!(fixture.validate_label(), Ok(()));
    assert_eq!(
        fixture.mark_success(),
        Err(FixtureEvidenceCompletionDenial::SampleOnlyEvidenceCannotCompleteScenario),
        "sample atlas evidence must never be convertible into scenario success"
    );
}

#[test]
fn surface_atlas_rejects_unlabeled_fixture_evidence() {
    let fixture = SurfaceAtlasFixtureEvidence::with_label_for_diagnostics("");

    assert_eq!(
        fixture.validate_label(),
        Err(FixtureEvidenceLabelDenial::MissingSampleOnlyLabel)
    );
    assert_eq!(
        fixture.mark_success(),
        Err(FixtureEvidenceCompletionDenial::SampleOnlyEvidenceCannotCompleteScenario)
    );
}

#[test]
fn surface_atlas_does_not_create_runtime_receipts_from_fixture_rows() {
    let fixture = SurfaceAtlasFixtureEvidence::sample_only();

    assert!(
        fixture
            .display_families()
            .any(|family| family == HarnessEvidenceFamily::VisibleFrameObservation),
        "the atlas may preview visual observation expectations"
    );
    assert!(
        !fixture
            .evidence()
            .contains(HarnessEvidenceFamily::RuntimeReceipt),
        "fixture rows must not synthesize runtime receipts"
    );
    assert!(
        !fixture
            .display_families()
            .any(|family| family == HarnessEvidenceFamily::RuntimeReceipt),
        "sample-only fixture rows must not imply a run has completed"
    );
}
