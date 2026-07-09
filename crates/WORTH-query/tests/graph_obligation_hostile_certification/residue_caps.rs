use worth_query::facade::consumer_kit::{
    WorthQueryGraphObligationConsumerKitErrorKind, WorthQueryGraphObligationResidueManifest,
    WorthQueryGraphObligationResidueRow,
};

#[test]
fn residue_rows_require_owner_intro_cap_and_removal_trigger() {
    let row = residue_row("selector-precision", 1, 1);
    let manifest = WorthQueryGraphObligationResidueManifest::capped([row]).unwrap();
    let row = manifest.rows().first().expect("residue row");

    assert_eq!(row.owner(), "worth-query.phase-20");
    assert_eq!(row.introduced_in(), "milestone-9.9-phase-20");
    assert_eq!(row.must_not_exceed_count(), 1);
    assert_eq!(row.removal_trigger(), "exact selector closure");
    assert!(!row.blocker().is_empty());
    assert!(!row.decision().is_empty());
}

#[test]
fn residue_manifest_rejects_growth_after_introduction() {
    let previous =
        WorthQueryGraphObligationResidueManifest::capped([residue_row("selector-precision", 1, 2)])
            .unwrap();
    let candidate =
        WorthQueryGraphObligationResidueManifest::capped([residue_row("selector-precision", 2, 2)])
            .unwrap();

    let error = WorthQueryGraphObligationResidueManifest::certify_candidate_against_previous(
        &previous, &candidate,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        WorthQueryGraphObligationConsumerKitErrorKind::ResidueGrowthAfterIntroduction
    );
}

fn residue_row(
    class: &str,
    current_count: usize,
    cap: usize,
) -> WorthQueryGraphObligationResidueRow {
    WorthQueryGraphObligationResidueRow::explicit(
        class,
        "worth-query.phase-20",
        "milestone-9.9-phase-20",
        current_count,
        cap,
        "representative closeout residue blocker",
        "exact selector closure",
        "kept only as explicit closeout residue",
    )
    .unwrap()
}
