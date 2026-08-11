use super::super::row_catalog::{CanonicalRowSpec, DigestRelation, CANONICAL_ROW_SPECS};
use super::super::MilestoneFivePointOneLiveCertificationAdapter;
use super::canonical_locality_expectations::assert_canonical_locality_expectation;
use super::canonical_stream_expectations::assert_canonical_stream_expectation;
use super::row_lookup::canonical_row;
use crate::harness::live_certification::LiveCertificationMatrix;

#[test]
fn region_live_concrete_rows_enforce_exact_counter_shapes() {
    let matrix = MilestoneFivePointOneLiveCertificationAdapter::
        region_scoped_live_narrowing_and_stream_contract_test();

    for expectation in CANONICAL_ROW_SPECS {
        assert_canonical_expectation(&matrix, expectation);
    }

    for expectation in super::super::row_catalog::REJECTION_ROW_SPECS {
        super::rejection_expectations::assert_rejection_expectation(&matrix, expectation);
    }
}

fn assert_canonical_expectation(matrix: &LiveCertificationMatrix, expectation: &CanonicalRowSpec) {
    let row = canonical_row(matrix, expectation.row_name);
    assert_eq!(row.control_lane.family, expectation.family);
    assert_eq!(row.control_lane.outcome_kind, expectation.outcome_kind);
    match expectation.digest_relation {
        DigestRelation::MatchesDeliveryDigest => {
            assert_eq!(
                row.control_lane.outcome_digest, row.control_lane.delivery_digest,
                "canonical row {} should keep outcome digest aligned with delivery digest",
                expectation.row_name
            );
        }
        DigestRelation::DiffersFromDeliveryDigest => {
            assert_ne!(
                row.control_lane.outcome_digest, row.control_lane.delivery_digest,
                "canonical row {} should keep stream contract digest distinct from delivery digest",
                expectation.row_name
            );
        }
    }

    match expectation.row_name {
        "region-live-convergence"
        | "off-region-suppression-parity"
        | "collection-partition-hit"
        | "bounded-materialization-region-hit"
        | "detail-region-single-peer-widening"
        | "broad-vs-region-narrowing-control"
        | "locality-breadth-budget-enforcement"
        | "locality-work-avoided-counter-parity" => {
            assert_canonical_locality_expectation(row, expectation.row_name);
        }
        "cdc-stream-lowered-parity"
        | "stream-contract-delivery-parity"
        | "stream-member-width-budget-enforcement" => {
            assert_canonical_stream_expectation(row, expectation.row_name);
        }
        _ => panic!(
            "missing canonical row expectation handler for {}",
            expectation.row_name
        ),
    }
}
