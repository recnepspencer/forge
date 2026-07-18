use worth_ui_certification::topology::audit_application_authority_topology;

const PUBLIC_COMPILE_CONTRACTS: &str =
    include_str!("../../worth-ui/tests/suites/compile_contract_cases.csv");

const REQUIRED_ANTI_BYPASS_CASES: &[&str] = &[
    "prepared_application_authority_fields_not_public.rs",
    "prepared_application_generation_identity_not_publicly_mintable.rs",
    "prepared_application_authority_has_no_canonical_extraction.rs",
    "candidate_submission_has_no_candidate_extraction.rs",
    "raw_runtime_cannot_lower_artifact_only_replacement.rs",
    "raw_runtime_cannot_open_source_ingress.rs",
    "prepared_application_cannot_launch_twice.rs",
    "prepared_application_has_no_active_inspection.rs",
    "active_application_session_cannot_be_split.rs",
    "host_session_identity_not_publicly_mintable.rs",
    "raw_host_adapter_cannot_submit_to_framework_turn.rs",
    "host_measurement_capability_requires_active_session.rs",
];

#[test]
fn production_has_one_application_authority_topology() {
    let findings = audit_application_authority_topology(super::workspace_source_inventory());
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}

#[test]
fn consolidated_compile_suite_retains_every_application_anti_bypass_contract() {
    let missing = REQUIRED_ANTI_BYPASS_CASES
        .iter()
        .filter(|case| !PUBLIC_COMPILE_CONTRACTS.contains(**case))
        .copied()
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty(),
        "missing compile contracts: {missing:#?}"
    );
}
