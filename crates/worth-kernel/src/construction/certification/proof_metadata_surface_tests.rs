const PROOF_MOD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/construction/proof/mod.rs"
));

#[test]
fn proof_metadata_surface_tests_certify_the_live_proof_owner_inventory() {
    for required in [
        "mod canonical_witness_parity_report;",
        "mod compile_fail_report;",
        "mod digest_protocol_report;",
        "mod geometry_digest_sensitivity_report;",
        "mod shell_with_hole_layout_hostility_suite;",
        "mod simplex_canonical_ratio_report;",
    ] {
        assert!(
            PROOF_MOD.contains(required),
            "proof owner inventory drifted; missing required proof owner module declaration: {required}"
        );
    }

    for forbidden in [
        "mod substrate_closeout_report;",
        "mod truth_projection_matrix;",
        "mod verified_artifact_surface_report;",
    ] {
        assert!(
            !PROOF_MOD.contains(forbidden),
            "proof band reintroduced deleted synthetic metadata shelf: {forbidden}"
        );
    }
}
