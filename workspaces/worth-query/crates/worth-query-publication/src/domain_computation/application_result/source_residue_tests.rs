const APPLICATION_RESULT_CONE: [&str; 7] = [
    include_str!("../../application_aftermath/boundary_evidence.rs"),
    include_str!("../application_result.rs"),
    include_str!("disclosure.rs"),
    include_str!("inspection.rs"),
    include_str!("receipt.rs"),
    include_str!("terminal_release.rs"),
    include_str!("terminal_release/aggregate_tests.rs"),
];

#[test]
fn application_result_publication_cone_contains_no_hashing_helpers() {
    for source in APPLICATION_RESULT_CONE {
        for forbidden in [
            "sha2::",
            "Sha256",
            "publication_digest",
            "canonical_hash",
            "canonical_identity",
            ".finalize()",
        ] {
            assert!(
                !source.contains(forbidden),
                "application-result publication must not contain `{forbidden}`"
            );
        }
    }
}
