use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitErrorKind, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow,
};

#[test]
fn residue_rows_require_owner_intro_cap_and_removal_trigger() {
    let row = residue_row("selector-precision", 1, 1);
    let manifest = ForgeQueryGraphObligationResidueManifest::capped([row]).unwrap();
    let row = manifest.rows().first().expect("residue row");

    assert_eq!(row.owner(), "forge-query.phase-20");
    assert_eq!(row.introduced_in(), "milestone-9.9-phase-20");
    assert_eq!(row.must_not_exceed_count(), 1);
    assert_eq!(row.removal_trigger(), "exact selector closure");
    assert!(!row.blocker().is_empty());
    assert!(!row.decision().is_empty());
}

#[test]
fn residue_manifest_rejects_growth_after_introduction() {
    let previous =
        ForgeQueryGraphObligationResidueManifest::capped([residue_row("selector-precision", 1, 2)])
            .unwrap();
    let candidate =
        ForgeQueryGraphObligationResidueManifest::capped([residue_row("selector-precision", 2, 2)])
            .unwrap();

    let error = ForgeQueryGraphObligationResidueManifest::certify_candidate_against_previous(
        &previous, &candidate,
    )
    .unwrap_err();

    assert_eq!(
        error.kind(),
        ForgeQueryGraphObligationConsumerKitErrorKind::ResidueGrowthAfterIntroduction
    );
}

fn residue_row(
    class: &str,
    current_count: usize,
    cap: usize,
) -> ForgeQueryGraphObligationResidueRow {
    ForgeQueryGraphObligationResidueRow::explicit(
        class,
        "forge-query.phase-20",
        "milestone-9.9-phase-20",
        current_count,
        cap,
        "representative closeout residue blocker",
        "exact selector closure",
        "kept only as explicit closeout residue",
    )
    .unwrap()
}
