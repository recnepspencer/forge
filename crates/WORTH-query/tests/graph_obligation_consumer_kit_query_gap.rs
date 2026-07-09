use worth_query::facade::consumer_kit::{
    WorthQueryGraphObligationResidueManifest, WorthQueryGraphObligationResidueRow,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthGraphAuthorityPhaseTwoGapFamily {
    Selector,
    Support,
    Receipt,
    ConsumerKit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthGraphAuthorityExcludedSurface {
    DurableGraphProofArchive,
}

fn phase_two_query_gap_row(
    family: WorthGraphAuthorityPhaseTwoGapFamily,
) -> WorthQueryGraphObligationResidueRow {
    let (id, blocker, removal_trigger) = match family {
        WorthGraphAuthorityPhaseTwoGapFamily::Selector => (
            "worth-query-graph-authority.query-gap.selector-contract",
            "an admitted Worth graph-authority lane is blocked on a Query selector API",
            "ship the missing Query selector API before adding a Worth-local selector",
        ),
        WorthGraphAuthorityPhaseTwoGapFamily::Support => (
            "worth-query-graph-authority.query-gap.support-contract",
            "an admitted Worth graph-authority lane is blocked on a Query support posture API",
            "ship the missing Query support API before adding Worth-local support pinning",
        ),
        WorthGraphAuthorityPhaseTwoGapFamily::Receipt => (
            "worth-query-graph-authority.query-gap.receipt-contract",
            "an admitted Worth graph-authority lane is blocked on a Query receipt API",
            "ship the missing Query receipt API before fabricating a Worth receipt",
        ),
        WorthGraphAuthorityPhaseTwoGapFamily::ConsumerKit => (
            "worth-query-graph-authority.query-gap.consumer-kit-contract",
            "an admitted Worth graph-authority lane is blocked on a Query Consumer Kit API",
            "ship the missing Consumer Kit API before adding a Worth-local adoption facade",
        ),
    };
    WorthQueryGraphObligationResidueRow::explicit(
        id,
        "worth-query",
        "worth-query-graph-authority-hardening.phase-2",
        1,
        1,
        blocker,
        removal_trigger,
        "query-gap",
    )
    .unwrap()
}

fn excluded_surface_query_gap_row(
    surface: WorthGraphAuthorityExcludedSurface,
) -> Result<WorthQueryGraphObligationResidueRow, &'static str> {
    match surface {
        WorthGraphAuthorityExcludedSurface::DurableGraphProofArchive => {
            Err("durable graph proof archives are excluded from this gate")
        }
    }
}

#[test]
fn query_capability_blockers_are_explicit_query_gap_rows() {
    for family in [
        WorthGraphAuthorityPhaseTwoGapFamily::Selector,
        WorthGraphAuthorityPhaseTwoGapFamily::Support,
        WorthGraphAuthorityPhaseTwoGapFamily::Receipt,
        WorthGraphAuthorityPhaseTwoGapFamily::ConsumerKit,
    ] {
        let manifest =
            WorthQueryGraphObligationResidueManifest::capped([phase_two_query_gap_row(family)])
                .unwrap();
        let row = &manifest.rows()[0];
        assert_eq!(row.decision(), "query-gap");
        assert_eq!(row.owner(), "worth-query");
        assert_eq!(row.current_count(), 1);
        assert!(row
            .blocker()
            .contains("admitted Worth graph-authority lane"));
    }
}

#[test]
fn excluded_surfaces_cannot_certify_phase_two_query_gaps() {
    let result = excluded_surface_query_gap_row(
        WorthGraphAuthorityExcludedSurface::DurableGraphProofArchive,
    );

    assert_eq!(
        result,
        Err("durable graph proof archives are excluded from this gate")
    );
}
