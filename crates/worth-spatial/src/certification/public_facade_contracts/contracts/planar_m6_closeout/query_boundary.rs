use worth_spatial::facade::planar_m6_closeout::M6PlanarCloseoutQueryCertification;

use super::fixture::{closeout_contracts, complete_certification};

#[test]
fn m6_closeout_proves_query_owned_runtime_lanes() {
    let contracts = closeout_contracts("m6-closeout-query-boundary");
    let receipt = M6PlanarCloseoutQueryCertification::from_certification(complete_certification(
        "m6-closeout-query-boundary",
    ))
    .compile(&contracts)
    .expect("M6 Query-bound closeout plan")
    .certify()
    .expect("M6 Query-bound closeout receipt");

    assert!(receipt.proves_query_owned_runtime_lanes());
    assert!(!receipt.declaration_digest().is_empty());
    assert!(!receipt.envelope_digest().is_empty());
    assert!(!receipt.closeout_digest().is_empty());
    assert_eq!(receipt.counters().query_boundary_rows(), 1);
}
