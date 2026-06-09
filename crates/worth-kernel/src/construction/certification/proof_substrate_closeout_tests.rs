use crate::construction::proof::canonical_witness_parity_report::prepare_primitive_canonical_witness_parity_report;
use crate::construction::proof::compile_fail_report::{
    prepare_primitive_construction_proof_boundary_compile_fail_report,
    PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES,
};
use crate::construction::proof::digest_protocol_report::prepare_primitive_construction_digest_protocol_report;
use crate::construction::proof::geometry_digest_sensitivity_report::prepare_primitive_geometry_digest_sensitivity_report;
use crate::construction::proof::shell_with_hole_layout_hostility_suite::prepare_shell_with_hole_layout_hostility_suite;
use crate::construction::proof::simplex_canonical_ratio_report::prepare_simplex_canonical_ratio_report;

#[test]
fn proof_substrate_closeout_report_certifies_named_proof_runtime_properties() {
    let digest_protocol_report = prepare_primitive_construction_digest_protocol_report();
    let geometry_digest_sensitivity_report = prepare_primitive_geometry_digest_sensitivity_report();
    let canonical_witness_parity_report = prepare_primitive_canonical_witness_parity_report();
    let shell_with_hole_layout_hostility_suite = prepare_shell_with_hole_layout_hostility_suite();
    let simplex_canonical_ratio_report = prepare_simplex_canonical_ratio_report();
    let proof_boundary_compile_fail_report =
        prepare_primitive_construction_proof_boundary_compile_fail_report();

    assert_eq!(
        digest_protocol_report.version_prefix(),
        "worth-primitives-digest:v1"
    );
    assert!(geometry_digest_sensitivity_report.covers_expected_mutation_cases());
    assert!(canonical_witness_parity_report.covers_expected_families());
    assert!(shell_with_hole_layout_hostility_suite
        .containment()
        .containment_verified());
    assert!(shell_with_hole_layout_hostility_suite
        .non_overlap()
        .non_overlap_verified());
    assert!(shell_with_hole_layout_hostility_suite.rejected_missing_hole_loop());
    assert!(
        (simplex_canonical_ratio_report.definition().lateral_ratio()
            - worth_primitives::CANONICAL_SIMPLEX_LATERAL_RATIO)
            .abs()
            <= f64::EPSILON
    );
    assert_eq!(
        proof_boundary_compile_fail_report.fixtures().len(),
        PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES.len()
    );
}
