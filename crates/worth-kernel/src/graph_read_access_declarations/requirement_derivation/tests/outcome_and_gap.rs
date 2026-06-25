use crate::graph_read_access_declarations::{
    current_worth_graph_read_requirement_derivation_closeout,
    WorthGraphReadRequirementDerivationCapabilityGapKind,
    WorthGraphReadRequirementDerivationErrorKind, WorthGraphReadRequirementDerivationOutcome,
};

use super::super::query_projection::derive_query_requirement_outcome_for_catalog_record_with_requirement_labels;
use super::common::{phase_two_closeout_from_seed, production_seed};

#[test]
fn query_derives_requirement_rows_or_gap_for_registered_families() {
    let phase_two = phase_two_closeout_from_seed(&production_seed());
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 2 catalog records should produce Phase 4 requirement outcomes");

    assert_eq!(
        phase_four.derivation_summary().catalog_record_count(),
        phase_two.declaration_catalog().records().len()
    );
    assert_eq!(
        phase_four.derivation_summary().catalog_record_count(),
        phase_four.derivation_summary().query_derived_record_count()
            + phase_four.derivation_summary().derivation_gap_count()
    );
    assert!(phase_four
        .requirement_records()
        .iter()
        .all(|record| matches!(
            record.derivation_outcome(),
            WorthGraphReadRequirementDerivationOutcome::QueryDerived(_)
                | WorthGraphReadRequirementDerivationOutcome::QueryCapabilityGap(_)
        )));
}

#[test]
fn vocabulary_only_query_capability_routes_to_gap_when_derivation_is_missing() {
    let phase_two = phase_two_closeout_from_seed(&production_seed());
    let phase_four = current_worth_graph_read_requirement_derivation_closeout(&phase_two)
        .expect("Phase 4 should expose derivation gaps instead of fabricating rows");

    assert_eq!(
        phase_four.derivation_summary().query_derived_record_count(),
        0
    );
    assert_eq!(
        phase_four.derivation_summary().derivation_gap_count(),
        phase_four.derivation_summary().catalog_record_count()
    );
    assert!(phase_four.requirement_records().iter().all(|record| {
        let gap = record
            .derivation_outcome()
            .capability_gap()
            .expect("current catalog records should route to derivation gaps");
        gap.kind()
            == WorthGraphReadRequirementDerivationCapabilityGapKind::MissingQueryReadFamilyArtifact
            && gap.source_catalog_record_digest() == record.source_catalog_record_digest()
            && gap.query_family_anchor_digest() == record.query_family_digest_seed()
            && gap.missing_prerequisite() == "ForgeQueryReadFamily"
            && gap.query_api_required() == "explain_graph_read_access_requirements_for_family(...)"
            && !gap.query_capability_labels().is_empty()
            && !gap.blocker().is_empty()
            && !gap.removal_trigger().is_empty()
            && !gap.claims_query_requirement_rows_derived()
    }));
}

#[test]
fn empty_query_requirement_capability_inventory_fails_closed() {
    let phase_two = phase_two_closeout_from_seed(&production_seed());
    let record = phase_two
        .declaration_catalog()
        .records()
        .first()
        .expect("production seed should contain a catalog record");
    let error =
        derive_query_requirement_outcome_for_catalog_record_with_requirement_labels(record, &[])
            .expect_err("empty capability labels should not become a generic gap");

    assert_eq!(
        error.kind(),
        WorthGraphReadRequirementDerivationErrorKind::MissingQueryRequirementCapabilityInventory
    );
}
