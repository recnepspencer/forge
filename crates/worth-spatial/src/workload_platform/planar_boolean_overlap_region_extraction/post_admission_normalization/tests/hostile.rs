use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanPostAdmissionNormalizationDenialKind::{
    AmbiguousCanonicalBoundaryDenied, AmbiguousCanonicalWindingDenied,
};

use super::support::{
    ambiguous_admitted_region_bundle, ambiguous_boundary_only_bundle, canonical_winding_graph,
};

#[test]
fn duplicate_admitted_region_witnesses_deny_before_identity_and_ledger_work() {
    let denial = ambiguous_admitted_region_bundle(&canonical_winding_graph())
        .normalize_post_admission_canonical_winding()
        .expect_err("duplicate admitted-region witnesses must deny");

    assert_eq!(denial.kind(), AmbiguousCanonicalWindingDenied);
}

#[test]
fn duplicate_boundary_only_witnesses_deny_before_identity_and_ledger_work() {
    let denial = ambiguous_boundary_only_bundle()
        .normalize_post_admission_canonical_winding()
        .expect_err("duplicate boundary-only witnesses must deny");

    assert_eq!(denial.kind(), AmbiguousCanonicalBoundaryDenied);
}
